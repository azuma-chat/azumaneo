use std::collections::{HashMap};

use actix::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::awsp::wrapper::AwspWrapper;
use crate::models::awsp::error::AwspError;

#[derive(Message)]
#[rtype(result = "Uuid")]
struct Msg(Uuid);


/// Chat server sends this messages to session
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(result = "Uuid")]
pub struct Connect {
    pub addr: Recipient<Message>,
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
    pub sessions: HashMap<Uuid, Recipient<Message>>,
    ///Database
    pub db: PgPool,
}

impl ChatServer {
    pub fn new(db: PgPool) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            db: db,
        }
    }
    /// broadcast message to all users
    pub fn broadcast_all_str(&self, message: &str) {
        for (_sess_id, addr) in &self.sessions {
            let _ = addr.do_send(Message(message.to_owned()));
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

        let db = self.db.clone();
        // remove address from sessions map
        self.sessions.remove(&msg.id);

        //remove saved wssession id from the database
        /*actix_web::rt::spawn(async move {
            let sess = match Session::get_by_wssession(&msg.id.to_string(), &db).await {
                Ok(sess) => sess,
                Err(_) => {
                    return info!("The ID of the leaving was never registered!");
                }
            };
            Session::rm_wssession(sess.borrow(), &db).await;
        });*/
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

impl Handler<AwspError> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: AwspError, _: &mut Context<Self>) {
        //this should be safe as the struct is generated internally
        let json = serde_json::to_string(&msg)
            .expect("An error occurred while translating message to json");
        self.broadcast_all_str(json.as_str());
    }
}

impl Handler<Msg> for ChatServer {
    type Result = MessageResult<Msg>;

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        MessageResult(msg.0)
    }
}