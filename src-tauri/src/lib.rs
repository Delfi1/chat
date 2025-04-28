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

    pub fn on_chat_insert(&mut self, chat: &Chat) {
        println!("Chat inserted: {:?}", chat);

        self.app.emit("chat_inserted", 
            (chat.id, chat.name.clone(), chat.users.clone())
        ).expect("Emit error");
    }

    pub fn on_message_insert(&mut self, message: &Message) {
        println!("Message inserted: {:?}", message);

        let sent = message.sent.to_duration_since_unix_epoch().expect("Unexpected");
        self.app.emit("message_inserted", 
            (message.chat, message.sender, sent, message.text.clone())
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
    ctx.db.chat().on_insert(move |_ctx, chat| {
        inner.lock().unwrap().on_chat_insert(chat);
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
        .subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}

#[tauri::command]
fn connect(addr: Option<String>, session: State<SessionState>) -> bool { 
    let session_inner = session.inner().clone();

    let addr = addr.unwrap_or(ADDR.to_string());
    let uri = get_uri(addr.clone());
    let s_addr = addr.clone();
    let res = DbConnection::builder()
        .on_connect(move |_ctx, identity, token| {
            // Save current identity
            session_inner.lock().unwrap().identity = Some(identity);

            if let Err(e) = creds_store(addr).save(token) {
                eprintln!("Failed to save credentials: {:?}", e);
            }
        })
        .with_token(creds_store(s_addr).load().expect("Error loading credentials"))
        .with_module_name(DB_NAME.to_string())
        .with_uri(uri)
        .build();

    match res {
        Ok(connection) => {
            // Setup spacetime callbacks, get tables
            register_callbacks(&connection, session.inner().clone());
            subscribe_to_tables(&connection);

            connection.run_threaded();
            session.lock().unwrap().connection = Some(connection);
            return true;
        },
        Err(e) => {
            eprintln!("Spacetime error: {}", e);
            return false;
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![connect])
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
