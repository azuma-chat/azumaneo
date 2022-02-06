//!  The majority of necessary structs and enums are located in here
//!
//!  We try to keep most of the structs (and their trait implementations) here in order to keep it organized

/// We use a generic error type for all the errors occurring in azumaneo
pub mod error;
/// Textmessage struct and its impls
pub mod message;
/// Session related stuff
pub mod session;
pub mod stateactor;
pub mod pub_sub;
/// The textchannel struct representation and all its trait implementations
pub mod textchannel;
/// Database and internal representations of a user
pub mod user;
pub mod ws;
