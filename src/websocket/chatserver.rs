use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerIssue;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::awsp::etc::OnlineStatus;
use crate::models::awsp::wrapper::AwspWrapper;
use crate::websocket::ws_connection_handler::{UpdateRequest as WsUpdateRequest, Ws};

// This is just a wrapper struct
#[doc(hidden)]
#[derive(Message)]
#[rtype(result = "Uuid")]
struct UuidWrapper(Uuid);

/// This struct is used to notify the ChatServer about a user changing its online status
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct UpdateUserOnlinestatus {
    pub user: Uuid,
    pub status: OnlineStatus,
}

/// Chat server sends this messages to session
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Sent by [`Ws`] actor on startup
///
/// This message is send by the [`Ws`] actor to the ChatServer in order to notify it about its existence and provides an Addr<Self> to make it contactable by other actors
#[derive(Message)]
#[rtype(result = "Uuid")]
pub struct Connect {
    pub addr: Addr<Ws>,
    pub id: Uuid,
}

/// Sent by [`Ws`] actor on shutdown
///
/// Indicates that a [`Ws`] actor lost the connection to its client and is about to be shut down
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

/// Only used for debugging.
///
/// This triggers a complete debug formatted print of the [`ChatServer`] struct
#[derive(Message)]
#[rtype(result = "()")]
pub struct Debug;

/// The core handler for all things related to websocket communication
///
/// The `ChatServer` actor is used to coordinate all websocket communications
#[derive(Clone, Debug)]
pub struct ChatServer {
    ///This map holds all sessions and their ids
    pub sessions: HashMap<Uuid, Addr<Ws>>,
    /// This HashMap holds every user who has a currently connected ws session and the corresponding online status
    pub onlinestatuses: HashMap<Uuid, OnlineStatus>,
    /// Database connection pool
    pub db: PgPool,
}

impl ChatServer {
    /// Create a instance of the `ChatServer` struct
    pub fn new(db: PgPool) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            onlinestatuses: HashMap::new(),
            db,
        }
    }

    /// broadcast message to all users
    pub fn broadcast_all_str(&self, message: &str) {
        for addr in self.sessions.values() {
            addr.do_send(Message(message.to_owned()));
        }
    }

    pub fn broadcast_all_awsp(&self, message: AwspWrapper) {
        let message = Message(
            serde_json::to_string(&message)
                .expect("Internally generated message couldn't be serialized to JSON!"),
        );
        for (_sess_id, addr) in &self.sessions {
            let _ = addr.do_send(message.to_owned());
        }
    }
}

/// Make actor from [`ChatServer`]
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message. <br>
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = MessageResult<Connect>;

    #[allow(unused_must_use)]
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.id, msg.addr);
        //TODO: remember the last online state of the user
        if !self.onlinestatuses.contains_key(&msg.id) {
            self.onlinestatuses.insert(msg.id, OnlineStatus::Online);
        }
        // send id back
        MessageResult(msg.id)
    }
}

/// Handler for Disconnect message.
/// If a client disconnects from the websocket server the corresponding ws actor sends this message and then shuts down
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    #[allow(unused_must_use)]
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);
        // remove address from sessions map
        self.sessions.remove(&msg.id);
    }
}

impl Handler<AwspWrapper> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: AwspWrapper, _: &mut Context<Self>) {
        //this should be safe as the struct is generated internally
        let json = msg.to_string();
        self.broadcast_all_str(json.as_str());
    }
}

impl Handler<UuidWrapper> for ChatServer {
    type Result = MessageResult<UuidWrapper>;

    fn handle(&mut self, msg: UuidWrapper, _: &mut Context<Self>) -> Self::Result {
        MessageResult(msg.0)
    }
}

/// In order to be able to update the [`Ws`] struct out of an asynchronous context we have to do an intermediate step and send a message to the [`ChatServer`] who relays it to the [`Ws`] back again
impl Handler<WsUpdateRequest> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: WsUpdateRequest, _: &mut Context<Self>) -> Self::Result {
        let recipient = self
            .sessions
            .get(&msg.to_update)
            .expect("The websocket session tried to update does not exist!");
        recipient.do_send(WsUpdateRequest {
            ws: msg.ws,
            to_update: msg.to_update,
        });
    }
}

impl Handler<UpdateUserOnlinestatus> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: UpdateUserOnlinestatus, _ctx: &mut Context<Self>) -> Self::Result {
        self.onlinestatuses.remove(&msg.user);
        self.onlinestatuses.insert(msg.user, msg.status);
        self.issue_system_async(msg);
    }
}
