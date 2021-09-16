use crate::websocket::connection::Ws;
use actix::{
    dev::{MessageResponse, OneshotSender},
    Actor, Addr, Context, Handler, Message,
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::mem::drop;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};
use uuid::Uuid;

use super::error::AzumaError;

/// `StateActor` holds all the runtime required data, which is not needed in a permanent database (e.g. because someone can't be online if the server isn't)
pub struct StateActor {
    onlinestatus: HashMap<Uuid, OnlineStatus>,
    usersessions: HashMap<Uuid, RefCell<HashMap<Uuid, Addr<Ws>>>>,
}

impl Actor for StateActor {
    type Context = Context<Self>;
}

impl StateActor {
    pub fn new() -> Self {
        StateActor {
            onlinestatus: HashMap::new(),
            usersessions: HashMap::new(),
        }
    }
}

// Messages
#[derive(Message)]
#[rtype(result = "OnlineStatus")]
/// `GetOnlineStatus` returns the onlinestatus of a user
pub struct GetOnlineStatus {
    /// Subject
    pub user: Uuid,
}

#[derive(Message)]
#[rtype(result = "Result<(), AzumaError>")]
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
    type Result = OnlineStatus;

    fn handle(&mut self, msg: GetOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        match self.onlinestatus.get(&msg.user) {
            None => OnlineStatus::Offline,
            Some(status) => *status,
        }
    }
}

impl Handler<SetOnlineStatus> for StateActor {
    type Result = Result<(), AzumaError>;

    fn handle(&mut self, msg: SetOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        match self.usersessions.get(&msg.user) {
            Some(_) => (),
            None => return Err(AzumaError::BadRequest),
        };
        self.onlinestatus.insert(msg.user, msg.status);
        Ok(())
    }
}

impl Handler<RemoveOnlineStatus> for StateActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveOnlineStatus, _ctx: &mut Self::Context) -> Self::Result {
        self.onlinestatus.remove(&msg.user);
    }
}

impl<A, M> MessageResponse<A, M> for OnlineStatus
where
    A: Actor,
    M: Message<Result = Self>,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
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

#[derive(Message)]
#[rtype(result = "()")]
/// Actor message to add a usersession to the stored ones
pub struct AddUserSession {
    pub subject: Uuid,
    pub connection_id: Uuid,
    pub addr: Addr<Ws>,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Actor message to have a specific usersession removed from the stored ones
pub struct RemoveUserSession {
    pub subject: Uuid,
    pub connection_id: Uuid,
}

impl Handler<AddUserSession> for StateActor {
    type Result = ();

    fn handle(&mut self, msg: AddUserSession, _ctx: &mut Self::Context) -> Self::Result {
        let sessions = match self.usersessions.get(&msg.subject) {
            Some(sessions) => {
                sessions.borrow_mut().insert(msg.connection_id, msg.addr);
                sessions.clone()
            }
            None => {
                let sessions: RefCell<HashMap<Uuid, Addr<Ws>>> = RefCell::new(HashMap::new());
                sessions.borrow_mut().insert(msg.connection_id, msg.addr);
                sessions
            }
        };
        self.usersessions.insert(msg.subject, sessions);
    }
}

impl Handler<RemoveUserSession> for StateActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveUserSession, _ctx: &mut Self::Context) -> Self::Result {
        let sessions = match self.usersessions.get(&msg.subject) {
            Some(sessions) => {
                let mut x = sessions.borrow_mut();
                x.remove(&msg.connection_id);
                if x.len() == 0 {
                    // if tuple return is true, hashset is empty and can be cleaned up from the sessions map
                    // because that was the only session the user had connected
                    (x, true)
                } else {
                    // if return.1 is false the KV pair mustn't be removed, because the user has other active sessions
                    (x, false)
                }
            }
            None => {
                return;
            }
        };

        if sessions.1 == true {
            // drop `sessions` in order to borrow mutable later
            drop(sessions);
            self.usersessions.remove(&msg.subject);
        }
    }
}
