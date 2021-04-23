/*!

    All things related to websocket communication are stored here.

    We use actix actors to represent the actual sessions the server has with its clients, for the controller etc.
    For a detailed explanation of how these work please refer to the [actix documentation](https://docs.rs/actix/), but I'll will explain the basics to you down below.

    <br>
    You can think of actors like of persons. Every one of these persons has a mailbox and a stack of envelopes.
    If the ChatServer - lets just call him Michael - wants to send a message to a websocket client (Anna) he first has to tell Annas friend, the websocket actor (Clara) to forward his message to Anna.
    These actors live only as long as necessary (the ChatServer as long as the server runs, each ws actor lives only as long as the connection persists)
*/

/// Everything related to channels is stored here
pub mod channelhandler;
/// The [`chatserver::ChatServer`] does most of the handling for azumaneos websocket communication
pub mod chatserver;
/// This holds the [`ws_connection_handler::Ws`] actor struct and all messages related to it
pub mod ws_connection_handler;
