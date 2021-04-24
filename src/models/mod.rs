//!  The majority of necessary structs and enums are located in here
//!
//!  We try to keep most of the structs (and their trait implementations) here in order to keep it organized

pub mod awsp;
/// We use a generic error type for all the errors occurring in azumaneo
pub mod error;
/// Everything not related to other categories is stored here
pub mod etc;
/// Textmessage struct and its impls
pub mod message;
/// Session related stuff
pub mod session;
/// The textchannel struct representation and all its trait implementations
pub mod textchannel;
/// Database and internal representations of a user
pub mod user;
