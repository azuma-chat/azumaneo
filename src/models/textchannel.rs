use uuid::Uuid;

// permission int is still missing
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextChannel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}