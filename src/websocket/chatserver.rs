use std::collections::HashMap;

use actix::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::awsp::wrapper::AwspWrapper;
use crate::websocket::handler::{UpdateRequest as WsUpdateRequest, Ws};

#[derive(Message)]
#[rtype(result = "Uuid")]
struct UuidWrapper(Uuid);

/// Chat server sends this messages to session
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(result = "Uuid")]
pub struct Connect {
    pub addr: Addr<Ws>,
    pub id: Uuid,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: Uuid,
    /// Peer message
    pub msg: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ChatServer {
    ///This map holds all sessions and their ids
    pub sessions: HashMap<Uuid, Addr<Ws>>,
    ///Database
    pub db: PgPool,
}

impl ChatServer {
    pub fn new(db: PgPool) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
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

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = MessageResult<Connect>;

    #[allow(unused_must_use)]
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.id, msg.addr);

        // send id back
        MessageResult(msg.id)
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    #[allow(unused_must_use)]
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.id);
        // remove address from sessions map
        self.sessions.remove(&msg.id);
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.broadcast_all_str(msg.msg.as_str());
    }
}

impl Handler<AwspWrapper> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: AwspWrapper, _: &mut Context<Self>) {
        //this should be safe as the struct is generated internally
        let json = serde_json::to_string(&msg)
            .expect("An error occurred while translating message to json");
        self.broadcast_all_str(json.as_str());
    }
}

impl Handler<UuidWrapper> for ChatServer {
    type Result = MessageResult<UuidWrapper>;

    fn handle(&mut self, msg: UuidWrapper, _: &mut Context<Self>) -> Self::Result {
        MessageResult(msg.0)
    }
}

impl Handler<WsUpdateRequest> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: WsUpdateRequest, _: &mut Context<Self>) -> Self::Result {
        let recipient = self
            .sessions
            .get(&msg.to_update)
            .expect("this ws does not exist!");
        println!("In chatserver req: {:?}", msg);
        recipient.do_send(WsUpdateRequest {
            ws: msg.ws,
            to_update: msg.to_update,
        });
    }
}
