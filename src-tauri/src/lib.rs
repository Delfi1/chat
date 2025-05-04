use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State};
use tauri_plugin_updater::UpdaterExt;

mod bindings;
use bindings::*;
use spacetimedb_sdk::*;

/// STDB connection consts
const ADDR: &str = "localhost";
const DB_NAME: &str = "chat";

/// Get server Uri from address
fn get_uri(addr: String) -> String {
    format!("http://{}:3000", addr)
}

/// Load saved user credentials
fn creds_store(addr: String) -> credentials::File {
    credentials::File::new(format!("delfi-chat-{}", addr))
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;
        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

#[derive(Clone, serde::Serialize)]
pub struct MessagePayload {
    pub id: u32,
    pub sender: u32,
    pub sent: u128,
    pub text: String,
}

impl MessagePayload {
    pub fn new(message: Message) -> Self {
        let sent = message
            .sent
            .to_duration_since_unix_epoch()
            .unwrap()
            .as_millis();
        Self {
            id: message.id,
            sender: message.sender,
            sent,
            text: message.text,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct UserPayload {
    pub id: u32,
    pub name: String,
    pub is_admin: bool,
    pub online: bool,
}

impl UserPayload {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            is_admin: user.is_admin,
            online: !user.online.is_empty(),
        }
    }
}

#[derive(Default)]
struct ConnectionHandler(Option<std::thread::JoinHandle<()>>);
type ConnectionState = Arc<Mutex<ConnectionHandler>>;

/// Session handler
struct SessionInner {
    /// Window events emitter
    pub app: AppHandle,
    pub connection: Option<DbConnection>,
    pub identity: Option<Identity>,
}

type SessionState = Arc<Mutex<SessionInner>>;

impl SessionInner {
    pub fn new(handle: &AppHandle) -> Self {
        Self {
            app: handle.clone(),
            connection: None,
            identity: None,
        }
    }

    pub fn on_connect(&mut self, identity: Identity) {
        self.identity = Some(identity);

        self.app.emit("on_connect", ()).expect("Emit error");
    }

    pub fn get_users(&self) -> Vec<UserPayload> {
        let Some(connection) = &self.connection else {
            return Vec::new();
        };

        connection
            .db
            .user()
            .iter()
            .map(|u| UserPayload::new(u))
            .collect()
    }

    pub fn on_connect_error(&mut self, error: String) {
        self.connection = None;
        self.identity = None;

        self.app
            .emit("on_connect_error", error)
            .expect("Emit error");
    }

    pub fn on_disconnect(&mut self, error: Option<String>) {
        self.connection = None;
        self.identity = None;

        self.app.emit("on_disconnect", error).expect("Emit error");
    }

    pub fn on_login_error(&mut self, error: String) {
        self.app.emit("on_login_error", error).expect("Emit error");
    }

    pub fn on_user_insert(&mut self, user: &User) {
        let identity = &self.identity.unwrap();
        if user.online.contains(&identity) {
            self.app
                .emit("loginned", UserPayload::new(user.clone()))
                .expect("Emit error");
        }

        self.app.emit("user_inserted", ()).expect("Emit error");
    }

    pub fn on_user_removed(&mut self, _user: &User) {
        self.app.emit("user_removed", ()).expect("Emit error");
    }

    pub fn on_user_updated(&mut self, _old: &User, new: &User) {
        let identity = &self.identity.unwrap();
        if new.online.contains(&identity) {
            self.app
                .emit("loginned", UserPayload::new(new.clone()))
                .expect("Emit error");
        }

        self.app.emit("user_updated", ()).expect("Emit error");
    }

    pub fn on_message_insert(&mut self, message: &Message) {
        self.app
            .emit("message_inserted", MessagePayload::new(message.clone()))
            .expect("Emit error");
    }

    pub fn on_message_removed(&mut self, _message: &Message) {
        self.app.emit("message_removed", ()).expect("Emit error");
    }

    pub fn on_message_updated(&mut self) {
        self.app.emit("message_updated", ()).expect("Emit error");
    }

    pub fn exit(&self) {
        println!("Exit");

        let Some(connection) = &self.connection else {
            return;
        };

        match connection.disconnect() {
            Err(e) => eprintln!("Disconnect error {}", e),
            _ => (),
        }
    }
}

fn register_callbacks(ctx: &DbConnection, session: SessionState) {
    let inner = session.clone();
    ctx.db.user().on_insert(move |_ctx, user| {
        inner.lock().unwrap().on_user_insert(user);
    });

    let inner = session.clone();
    ctx.db.message().on_insert(move |_ctx, message| {
        inner.lock().unwrap().on_message_insert(message);
    });

    let inner = session.clone();
    ctx.db.message().on_update(move |_ctx, _old, _new| {
        inner.lock().unwrap().on_message_updated();
    });

    let inner = session.clone();
    ctx.db.message().on_delete(move |_ctx, message| {
        inner.lock().unwrap().on_message_removed(message);
    });

    let inner = session.clone();
    ctx.db.user().on_update(move |_ctx, old, new| {
        inner.lock().unwrap().on_user_updated(old, new);
    });

    let inner = session.clone();
    ctx.db.user().on_delete(move |_ctx, user| {
        inner.lock().unwrap().on_user_removed(user);
    });

    let inner = session.clone();
    ctx.reducers.on_login(move |ctx, _name, _password| {
        if let Status::Failed(err) = &ctx.event.status {
            inner.lock().unwrap().on_login_error(err.to_string());
        }
    });

    let inner = session.clone();
    ctx.reducers.on_signup(move |ctx, _name, _password| {
        if let Status::Failed(err) = &ctx.event.status {
            inner.lock().unwrap().on_login_error(err.to_string());
        }
    });
}

fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
    std::process::exit(1);
}

fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_error(on_sub_error)
        .subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}

fn try_connect(addr: Option<String>, session: SessionState) {
    let on_connect_inner = session.clone();
    let on_disconnect_inner = session.clone();

    let addr = addr.unwrap_or(ADDR.to_string());
    let uri = get_uri(addr.clone());
    let token_addr = addr.clone();
    let res = DbConnection::builder()
        .on_connect(move |_ctx, identity, token| {
            on_connect_inner.lock().unwrap().on_connect(identity);
            println!("Connected!");

            if let Err(e) = creds_store(addr).save(token) {
                eprintln!("Failed to save credentials: {:?}", e);
            }
        })
        .on_disconnect(move |_ctx, err| {
            let error = err.and_then(|e| Some(e.to_string()));
            on_disconnect_inner.lock().unwrap().on_disconnect(error);
        })
        .with_token(
            creds_store(token_addr)
                .load()
                .expect("Error loading credentials"),
        )
        .with_module_name(DB_NAME.to_string())
        .with_uri(uri)
        .build();

    match res {
        Ok(connection) => {
            // Setup spacetime callbacks, get tables
            register_callbacks(&connection, session.clone());
            subscribe_to_tables(&connection);

            connection.run_threaded();
            session.lock().unwrap().connection = Some(connection);
        }
        Err(e) => {
            session.lock().unwrap().on_connect_error(e.to_string());
        }
    }
}

#[tauri::command]
fn connect(
    addr: Option<String>,
    session: State<SessionState>,
    connect_handler: State<ConnectionState>,
) {
    if let Some(connection) = session.lock().unwrap().connection.take() {
        match connection.disconnect() {
            Err(e) => eprintln!("Disconnect error {}", e),
            _ => (),
        }
    }

    // Clear connect handler and join;
    if let Some(handler) = connect_handler.lock().unwrap().0.take() {
        match handler.join() {
            _ => (),
        }
    }

    // Connect to STDB in thread
    let session_inner = session.inner().clone();
    let handler = std::thread::spawn(move || try_connect(addr, session_inner));

    connect_handler.lock().unwrap().0 = Some(handler);
}

#[tauri::command]
fn disconnect(session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    match connection.disconnect() {
        _ => (),
    }
}

#[tauri::command]
fn login(name: String, password: String, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection
        .reducers
        .login(name, password)
        .expect("Spacetime error");
}

#[tauri::command]
fn logout(session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection.reducers.logout().expect("Spacetime error");
}

#[tauri::command]
fn signup(name: String, password: String, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection
        .reducers
        .signup(name, password)
        .expect("Spacetime error");
}

#[tauri::command]
fn send_message(text: String, reply: Option<u32>, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection
        .reducers
        .send_message(text, reply, Vec::new())
        .expect("Spacetime error");
}

#[tauri::command]
fn remove_message(id: u32, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection
        .reducers
        .remove_message(id)
        .expect("Spacetime error");
}

#[tauri::command]
fn messages_len(session: State<SessionState>) -> usize {
    let Some(connection) = &session.lock().unwrap().connection else {
        return 0;
    };

    connection.db.message().iter().count()
}

#[tauri::command]
fn get_messages(session: State<SessionState>, start: usize, end: usize) -> Vec<MessagePayload> {
    let Some(connection) = &session.lock().unwrap().connection else {
        return vec![];
    };

    let mut messages = connection.db.message().iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.sent);

    let end = end.min(messages.len());
    let start = start.min(end);

    messages[start..end]
        .into_iter()
        .map(|m| MessagePayload::new(m.clone()))
        .collect()
}

#[tauri::command]
fn get_users(session: State<SessionState>) -> Vec<UserPayload> {
    session.lock().unwrap().get_users()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    let app = builder
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(ConnectionHandler::default())))
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect,
            signup,
            login,
            logout,
            send_message,
            remove_message,
            messages_len,
            get_messages,
            get_users,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|handle, event| match event {
        RunEvent::Ready => {
            // Setup session data
            handle.manage(Arc::new(Mutex::new(SessionInner::new(handle))));
        }
        RunEvent::Exit => {
            let session = handle.state::<SessionState>();
            session.lock().unwrap().exit();
        }
        _ => (),
    });
}
