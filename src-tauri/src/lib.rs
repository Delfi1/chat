use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State, RunEvent};

mod bindings;
use bindings::*;
use spacetimedb_sdk::*;

/// STDB connection consts
const ADDR: &str = "localhost";
const DB_NAME: &str = "chat";

/// Get server Uri from address
fn get_uri(addr: impl Into<String>) -> String {
    format!("http://{}:3000", addr.into())
}

/// Load saved user credentials
fn creds_store(addr: String) -> credentials::File {
    credentials::File::new(format!("delfi-chat-{}", addr))
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
            identity: None
        }
    }

    pub fn on_connect(&mut self, identity: Identity) {
        self.identity = Some(identity);

        self.app.emit("on_connect", ()).expect("Emit error");
    }

    pub fn on_connect_error(&mut self, error: String) {
        self.connection = None;
        self.identity = None;
        //println!("Error: {}", error);
        self.app.emit("on_connect_error", error).expect("Emit error");
    }

    pub fn on_disconnect(&mut self, error: Option<String>) {
        self.connection = None;
        self.identity = None;

        self.app.emit("on_disconnect", error).expect("Emit error");
    }

    pub fn on_user_insert(&mut self, user: &User) {
        let identity = &self.identity.unwrap();
        if user.online.contains(&identity) {
            self.app.emit("loginned",
                (user.id, user.name.clone(), !user.online.is_empty())
            ).expect("Emit error");
        }

        self.app.emit("user_inserted", 
            (user.id, user.name.clone(), !user.online.is_empty())
        ).expect("Emit error");
    }

    pub fn on_message_insert(&mut self, message: &Message) {
        let sent = message.sent.to_duration_since_unix_epoch().expect("Unexpected");
        self.app.emit("message_inserted", 
            (message.sender, sent, message.text.clone())
        ).expect("Emit error");
    }
    
    pub fn exit(&self) {
        println!("Exit");

        let Some(connection) = &self.connection
        else { return };

        match connection.disconnect() {
            Err(e) => eprintln!("Disconnect error {}", e),
            _ => ()
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
}

fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
    std::process::exit(1);
}

fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_error(on_sub_error)
        .subscribe(["SELECT * FROM user", "SELECT * FROM message",]);
}

fn try_connect(addr: Option<String>, session: SessionState) {
    let on_connect_inner = session.clone();
    let on_disconnect_inner = session.clone();
    
    let addr = addr.unwrap_or(ADDR.to_string());
    let uri = get_uri(addr.clone());
    let s_addr = addr.clone();
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
        .with_token(creds_store(s_addr).load().expect("Error loading credentials"))
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
        },
        Err(e) => {
            session.lock().unwrap().on_connect_error(e.to_string());
        }
    }
}

#[tauri::command]
fn connect(addr: Option<String>, session: State<SessionState>, connect_handler: State<ConnectionState>) { 
    // Clear connect handler if finished
    if let Some(handler) = connect_handler.lock().unwrap().0.take() {
        if !handler.is_finished() {
            connect_handler.lock().unwrap().0 = Some(handler);
        }
    }

    // Connect in thread
    if connect_handler.lock().unwrap().0.is_none() {
        let session_inner = session.inner().clone();
        let handler = std::thread::spawn(move || try_connect(addr, session_inner));
        
        connect_handler.lock().unwrap().0 = Some(handler);
    }
}

#[tauri::command]
fn disconnect(session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    match connection.disconnect() {
        _ => ()
    }
}

#[tauri::command]
fn signup(name: String, password: String, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection.reducers.signup(name, password).expect("Spacetime error");
}

#[tauri::command]
fn send_message(text: String, session: State<SessionState>) {
    let Some(connection) = &session.lock().unwrap().connection else {
        return;
    };

    connection.reducers.send_message(text).expect("Spacetime error");
}

#[tauri::command]
fn count_messages(session: State<SessionState>) -> u64 {
    let Some(connection) = &session.lock().unwrap().connection else {
        return 0;
    };

    connection.db.message().count()
}

#[tauri::command]
fn get_messages(session: State<SessionState>, start: usize, end: usize) -> Vec<(u32, i64, String)> {
    let Some(connection) = &session.lock().unwrap().connection else {
        return vec![];
    };

    let mut messages = connection.db.message().iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.sent);

    let payload = messages[start..end].to_vec();
    payload.into_iter().map(|p| (p.sender, p.sent.to_micros_since_unix_epoch(), p.text)).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(ConnectionHandler::default())))
        .invoke_handler(tauri::generate_handler![connect, disconnect, signup, send_message, count_messages, get_messages])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|handle, event| match event {
        RunEvent::Ready => {
            // Setup session data
            handle.manage(Arc::new(Mutex::new(SessionInner::new(handle))));
        },
        RunEvent::Exit => {
            let session = handle.state::<SessionState>();
            session.lock().unwrap().exit();
        },
        _ => ()
    });
}
