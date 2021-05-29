use crate::models::error::AzumaError;
use actix::{Actor, Context, Handler, Message};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// `StateActor` holds all the runtime required data, which is not needed in a permanent database (e.g. because someone can't be online if the server isn't)
pub struct StateActor {
    onlinestatus: HashMap<Uuid, OnlineStatus>,
}

impl Actor for StateActor {
    type Context = Context<Self>;
}

impl StateActor {
    pub fn new() -> Self {
        StateActor {
            onlinestatus: HashMap::new(),
        }
    }
}

// Messages
#[derive(Message)]
#[rtype(result = "Option<OnlineStatus>")]
/// `GetOnlineStatus` returns the onlinestatus of a user
pub struct GetOnlineStatus {
    /// Subject
    pub user: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Set the [`OnlineStatus`] of a specific user
pub struct SetOnlineStatus {
    /// The subject
    pub user: Uuid,
    /// New [`OnlineStatus`]
    pub status: OnlineStatus,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Remove the [`OnlineStatus`] of a specific user
pub struct RemoveOnlineStatus {
    /// The subject
    pub user: Uuid,
}

impl Handler<GetOnlineStatus> for StateActor {
    type Result = Option<OnlineStatus>;

    fn handle(&mut self, msg: GetOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        match self.onlinestatus.get(&msg.user) {
            None => None,
            Some(status) => Some(*status),
        }
    }
}

impl Handler<SetOnlineStatus> for StateActor {
    type Result = ();

    fn handle(&mut self, msg: SetOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        self.onlinestatus.insert(msg.user, msg.status);
    }
}

impl Handler<RemoveOnlineStatus> for StateActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        self.onlinestatus.remove(&msg.user);
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum OnlineStatus {
    /// This is set if a user establishes a connection and there is currently no other ws session connected which already set a onlinestatus used to represent the default state if a user is available
    #[serde(rename = "ONLINE")]
    Online,
    /// Marks the user as `AFK`
    #[serde(rename = "AFK")]
    Afk,
    #[serde(rename = "DND")]
    /// Used to represent the `Do not disturb` mode of the client which doesn't display any notifications to the user
    Dnd,
    #[serde(rename = "OFFLINE")]
    #[serde(alias = "APPEAR_AS_OFFLINE")]
    AppearAsOffline,
    // We skip deserializing the offline state. This makes setting itself to `OFFLINE` internally impossible
    #[serde(skip_deserializing)]
    // If we set the real offline response the same as the [`AppearAsOffline`] response, the user a never knows if b is really offline or just appearing as if he/she is
    #[serde(rename = "OFFLINE")]
    /// This is used to represent a user as offline internally. A user cannot set itself to Offline, which is the state if no ws sessions of this user are currently connected.
    /// If a user wants to appear as offline, he/she **must** send [`OnlineStatus::AppearAsOffline`]
    Offline,
}
