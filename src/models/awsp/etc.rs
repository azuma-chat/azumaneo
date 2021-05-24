use actix::Message;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// Only used internally if no other state applies
    // Never let the `Undefined` state appear outside of the backend
    #[serde(skip)]
    Undefined,
}

#[derive(Message)]
#[rtype(result = "()")]
/// actix actor message sent to [`Ws`] actors when a users [`OnlineStatus`] is updated
pub struct OnlineStatusUpdate {
    /// ID of the user updating itself
    pub subject: Uuid,
    /// new and updated [`OnlineStatus`]
    pub status: OnlineStatus,
}

impl Deref for OnlineStatus {
    type Target = Self;

    fn deref(&self) -> &Self {
        &self
    }
}
