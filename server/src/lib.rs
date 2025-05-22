use spacetimedb::*;

#[table(name=credentials)]
// User private data
struct UserCredentials {
    #[primary_key]
    user_id: u32,
    password: String,
    connections: Vec<Identity>,
}

/// Get user credentials linked with identity 
fn get_creds(ctx: &ReducerContext) -> Option<UserCredentials> {
    ctx.db.credentials().iter()
        .find(|c| c.connections.contains(&ctx.sender))
}

#[table(name=user, public)]
pub struct User {
    #[primary_key]
    #[auto_inc]
    id: u32,
    is_admin: bool,
    avatar: Option<Vec<u8>>,
    #[unique]
    name: String,
    online: Vec<Identity>,
}

#[table(name=message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u32,
    sender: u32,
    reply: Option<u32>,
    edited: Option<Timestamp>,
    sent: Timestamp,
    text: String,
    file: Option<FileRef>,
}

#[table(name=request, public)]
pub struct FileRequest {
    // file sender
    #[primary_key]
    sender: Identity,
    finished: bool,
    file: u32
}

#[table(name=temp_file)]
struct TempFile {
    #[auto_inc]
    #[primary_key]
    id: u32,
    name: String,
    // Current data
    data: Vec<u8>,
    // Result size
    size: u64
}

#[derive(SpacetimeType)]
pub struct FileRef {
    id: u32,
    name: String,
    size: u64,
}

#[table(name=file, public)]
pub struct File {
    #[primary_key]
    id: u32,
    name: String,
    data: Vec<u8>
}

#[reducer]
pub fn request_stream(ctx: &ReducerContext, name: String, size: u64) -> Result<(), String> {
    if ctx.db.request().sender().find(&ctx.sender).is_some() {
        return Err("Stream is aleready exists".to_string());
    }

    let temp = ctx.db.temp_file().insert(TempFile {
        id: 0,
        name,
        data: Vec::with_capacity(32768),
        size
    });

    ctx.db.request().insert(FileRequest {
        sender: ctx.sender,
        finished: false,
        file: temp.id
    });

    Ok(())
}

// Send data pocket
#[reducer]
pub fn send_packet(ctx: &ReducerContext, mut pocket: Vec<u8>) -> Result<(), String> {
    if get_creds(ctx).is_none() {
        return Err("Not loginned in".to_string());
    };

    // get stream
    let Some(mut request) = ctx.db.request().sender().find(ctx.sender) else {
        return Err("Request stream not found".to_string());
    };

    if request.finished {
        return Err("Request is finished".to_string());
    }

    let mut file = ctx.db.temp_file().id().find(request.file).unwrap();
    file.data.append(&mut pocket);

    // Update request and temp file
    if file.data.len() as u64 == file.size {
        request.finished = true;
        ctx.db.request().sender().update(request);
    }
    ctx.db.temp_file().id().update(file);
    
    Ok(())
}

#[reducer]
pub fn signup(ctx: &ReducerContext, name: String, password: String) -> Result<(), String> {
    if get_creds(ctx).is_some() {
        return Err("Already loginned in".to_string());
    };

    if name.len() < 3 {
        return Err("Name must be at least 3 characters long".to_string());
    }
    if password.len() < 4 {
        return Err("Password must be at least 4 characters long".to_string());
    } 

    if ctx.db.user().name().find(name.clone()).is_some() {
        return Err("User with this name is already exists".to_string());
    };

    let user = ctx.db.user().insert(User { id: 0, name, avatar: None, online: vec![ctx.sender], is_admin: false });
    ctx.db.credentials().insert( UserCredentials { user_id: user.id, password, connections: vec![ctx.sender] });

    Ok(())
}

#[reducer]
pub fn login(ctx: &ReducerContext, name: String, password: String) -> Result<(), String> {
    let Some(mut user) = ctx.db.user().name().find(name) else {
        return Err("User with this name is not exists".to_string());
    };
    let mut creds = ctx.db.credentials().user_id().find(user.id).unwrap();

    if creds.password != password {
        return Err("Invalid password".to_string());
    }

    creds.connections.push(ctx.sender);
    user.online.push(ctx.sender);

    ctx.db.credentials().user_id().update(creds);
    ctx.db.user().id().update(user);

    Ok(())
}

#[reducer]
pub fn logout(ctx: &ReducerContext) -> Result<(), String> {
    let Some(mut creds) = get_creds(ctx) else {
        return Err("You are not logged in".to_string());
    };
    let mut user = ctx.db.user().id().find(creds.user_id).unwrap();
    creds.connections.retain(|i| i != &ctx.sender);
    user.online.retain(|i| i != &ctx.sender);

    ctx.db.user().id().update(user);
    ctx.db.credentials().user_id().update(creds);
    
    Ok(())
}

#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String, reply: Option<u32>) -> Result<(), String> {
    let text = text.trim().to_string();
    let Some(creds) = get_creds(ctx) else {
        return Err("Not loginned in".to_string());
    };

    let file = match ctx.db.request().sender().find(ctx.sender) {
        Some(request) => {
            if !request.finished {
                return Err("Can't send message - file is not uploaded".to_string());
            }
            let temp = ctx.db.temp_file().id().find(request.file).unwrap();            

            let file = ctx.db.file().insert(File {
                id: temp.id,
                name: temp.name,
                data: temp.data
            });

            // Cleanup request and temp file
            ctx.db.request().sender().delete(request.sender);
            ctx.db.temp_file().id().delete(temp.id);

            Some(FileRef { id: file.id, name: file.name, size: file.data.len() as u64 })
        },
        None => None
    };

    if text.is_empty() && file.is_none() {
        return Err("Empty message".to_string());
    }

    // Create files
    ctx.db.message().insert(Message {
        id: 0,
        sender: creds.user_id,
        sent: ctx.timestamp,
        reply,
        edited: None,
        text,
        file
    });

    Ok(())
}

#[reducer]
pub fn remove_message(ctx: &ReducerContext, id: u32) -> Result<(), String> {
    let Some(creds) = get_creds(ctx) else {
        return Err("You are not logged in".to_string());
    };
    let user = ctx.db.user().id().find(creds.user_id).unwrap();
    let Some(message) = ctx.db.message().id().find(id) else {
        return Err("Message not found".to_string());
    };
    
    if !(user.id == message.sender || user.is_admin) {
        return Err("Permission denied".to_string());
    }

    // Remove attached file
    if let Some(file_ref) = message.file {
        ctx.db.file().id().delete(file_ref.id);
    }
    
    ctx.db.message().id().delete(id);
    Ok(())
}

#[reducer]
pub fn edit_message(ctx: &ReducerContext, id: u32, text: String) -> Result<(), String> {
    let text = text.trim().to_string();
    let Some(creds) = get_creds(ctx) else {
        return Err("Not loginned in".to_string());
    };

    let user = ctx.db.user().id().find(creds.user_id).unwrap();
    let Some(mut message) = ctx.db.message().id().find(id) else {
        return Err("Message not found".to_string());
    };
    
    if !(user.id == message.sender) {
        return Err("Permission denied".to_string());
    }

    if text.is_empty() && message.file.is_none() {
        return Err("Empty message".to_string());
    }

    if message.text != text {
        message.text = text;
        message.edited = Some(ctx.timestamp);
    }

    ctx.db.message().id().update(message);
    Ok(())
}

pub fn upload_file(ctx: &ReducerContext, name: String, data: Vec<u8>) -> Result<(), String> {
    if get_creds(ctx).is_none() {
        return Err("You are not logged in".to_string());
    };

    ctx.db.file().insert(File { id: 0, name, data });
    Ok(())
}

pub fn update_online(ctx: &ReducerContext, value: bool) {
    if let Some(creds) = get_creds(ctx) {
        let mut user = ctx.db.user().id().find(creds.user_id).unwrap();
        user.online.retain(|v| v != &ctx.sender);
        if value { user.online.push(ctx.sender) };

        ctx.db.user().id().update(user);
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    update_online(ctx, true);
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    update_online(ctx, false);

    // Close request if exists
    if let Some(request) = ctx.db.request().sender().find(ctx.sender) {
        ctx.db.request().sender().delete(ctx.sender);
        ctx.db.temp_file().id().delete(request.file);
    }
}

#[table(name=voice_packet, public)]
pub struct VoicePacket {
    room_id: u32,
    sender: u32,
    data: Vec<f32>,
}

fn get_room(ctx: &ReducerContext, creds: &UserCredentials) -> Option<VoiceRoom> {
    ctx.db.room().iter()
        .find(|r| r.users.contains(&creds.user_id))
}

#[table(name=room, public)]
pub struct VoiceRoom {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    // Users by id
    pub users: Vec<u32>
}

#[reducer]
pub fn send_voice_packet(ctx: &ReducerContext, data: Vec<f32>) -> Result<(), String> {
    let Some(creds) = get_creds(ctx) else {
        return Err("You are not logged in".to_string());
    };

    let Some(room) = get_room(ctx, &creds) else {
        return Err("User not in a voice room".to_string());
    };

    ctx.db.voice_packet().insert(VoicePacket { 
        room_id: room.id,
        sender: creds.user_id,
        data
    });

    Ok(())
}

#[reducer]
pub fn set_avatar(ctx: &ReducerContext, data: Vec<u8>) -> Result<(), String> {
    let Some(creds) = get_creds(ctx) else {
        return Err("You are not logged in".to_string());
    };

    let image = match image::load_from_memory(&data) {
        Ok(image) => image,
        Err(e) => return Err(format!("Image error: {}", e))
    };

    if image.width() != image.height() {
        return Err(format!("Image size error w{} not equals h{}", image.width(), image.height()));
    }

    // Update user avatar
    let mut user = ctx.db.user().id().find(creds.user_id).unwrap();
    user.avatar = Some(data);
    ctx.db.user().id().update(user);

    Ok(())
}