use spacetimedb::*;

#[spacetimedb::table(name=credentials)]
// User private data
struct UserCredentials {
    #[primary_key]
    user_id: u32,
    connections: Vec<Identity>,
    password: String
}

/// Identity linked user with
#[spacetimedb::table(name=linked)]
struct LinkedUser {
    id: u32,
    identity: Identity
}

#[spacetimedb::table(name=user, public)]
pub struct User {
    #[primary_key]
    #[auto_inc]
    id: u32,
    name: String,
    online: Vec<Identity>,
}

#[spacetimedb::table(name=chat, public)]
pub struct Chat {
    #[primary_key]
    #[auto_inc]
    id: u32,
    name: String,
    /// Vec of users ids
    users: Vec<u32>
}

#[spacetimedb::table(name=message, public)]
pub struct Message {
    chat: u32,
    sender: u32,
    sent: Timestamp,
    text: String,
}

#[spacetimedb::table(name=view)]
/// User access to chat
struct ViewAccess {
    chat: u32,
    user: u32
}

// Content filters
#[client_visibility_filter]
const CHAT_ACCESS_FILTER: Filter = Filter::Sql("
    SELECT c.*
    FROM chat c
    JOIN view v ON v.chat = c.id
    JOIN linked l ON l.id = v.user
    WHERE l.identity = :sender
");

#[client_visibility_filter]
const MESSAGE_ACCESS_FILTER: Filter = Filter::Sql("
    SELECT m.*
    FROM message m
    JOIN chat c ON c.id = m.chat
    JOIN view v ON v.chat = c.id
    JOIN linked l ON l.id = v.user
    WHERE l.identity = :sender
");

// Reducers
#[spacetimedb::reducer]
pub fn signup(_ctx: &ReducerContext, _name: String, _password: String) {
    
}

#[spacetimedb::reducer]
pub fn login(_ctx: &ReducerContext, _name: String, _password: String) {
    
}

#[spacetimedb::reducer]
pub fn logout(_ctx: &ReducerContext) {

}

#[spacetimedb::reducer]
pub fn send_message(_ctx: &ReducerContext, _chat: u32, _text: String) {

}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {

}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {

}