/*!
    This is the home of all http routes accessible via the api

*/
// allow unused import here, because its needed to make the internal doc link work
#[allow(unused_imports)]
use crate::models::user::User;

/// Fetch some infos about the running azumaneo server version
pub mod api;
/// Upgrade http connection to websocket
pub mod init_ws;
/// Everything related to messages
pub mod message;
/// Every route related to a users onlinestatus
pub mod onlinestatus;
/// Textchannel stuff is stored here
pub mod textchannel;
/// Bindings to the internal [`User`] model
pub mod user;
