use uuid::Uuid;
use crate::models::message::ChatMessage;
use crate::models::error::AzumaError;

// permission int is still missing
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextChannel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}