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
}

#[table(name=message, public)]
pub struct Message {
    sender: u32,
    sent: Timestamp,
    text: String,
}

#[reducer]
pub fn signup(ctx: &ReducerContext, name: String, password: String) -> Result<(), String> {
    if get_creds(ctx).is_some() {
        return Err("Already loginned in".to_string());
    };

    if ctx.db.user().name().find(name.clone()).is_some() {
        return Err("User with this name is already exists".to_string());
    };

    let user = ctx.db.user().insert(User { id: 0, name, online: vec![ctx.sender] });
    ctx.db.credentials().insert( UserCredentials { user_id: user.id, password, connections: vec![ctx.sender] });

    Ok(())
}

#[reducer]
pub fn login(ctx: &ReducerContext, name: String, password: String) -> Result<(), String> {
    if get_creds(ctx).is_some() {
        return Err("Already loginned in".to_string());
    };

    if ctx.db.user().name().find(name.clone()).is_none() {
        return Err("User with this name is not exists".to_string());
    };

    let user = ctx.db.user().insert(User { id: 0, name, online: vec![ctx.sender] });
    ctx.db.credentials().insert( UserCredentials { user_id: user.id, password, connections: vec![ctx.sender] });

    Ok(())
}

#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let Some(creds) = get_creds(ctx) else {
        return Err("Not loginned in".to_string());
    };

    ctx.db.message().insert(Message {
        sender: creds.user_id,
        sent: ctx.timestamp,
        text
    });

    Ok(())
}