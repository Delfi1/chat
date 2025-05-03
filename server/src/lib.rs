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
    #[unique]
    name: String,
    online: Vec<Identity>,
    // todo: answer
}

#[table(name=message, public)]
pub struct Message {
    #[primary_key]
    #[auto_inc]
    id: u32,
    sender: u32,
    sent: Timestamp,
    text: String,
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

    let user = ctx.db.user().insert(User { id: 0, name, online: vec![ctx.sender] });
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
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = text.trim().to_string();
    let Some(creds) = get_creds(ctx) else {
        return Err("Not loginned in".to_string());
    };

    if text.is_empty() {
        return Err("Empty message".to_string());
    }

    ctx.db.message().insert(Message {
        id: 0,
        sender: creds.user_id,
        sent: ctx.timestamp,
        text
    });

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
}
