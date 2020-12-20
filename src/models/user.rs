use crate::models::rejection::AzumaRejection;
use crate::routes::user::update::UpdatableUser;
use crate::util::prettyprint_option_string;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: u64,
    pub name: String,
    pub password: String,
    pub icon: Option<String>,
    pub status: UserStatus,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum UserProperties {
    ID,
    NAME,
    PASSWORD,
    ICON,
    STATUS,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserStatus {
    AVAILABLE,
    AFK,
    DND,
    OFFLINE,
}

impl UserStatus {
    pub fn from_string(value: String) -> UserStatus {
        //Todo fix automatic defaulting to OFFLINE in case of error
        let value = value.as_str();
        match value {
            "AVAILABLE" => UserStatus::AVAILABLE,
            "AFK" => UserStatus::AFK,
            "DND" => UserStatus::DND,
            "OFFLINE" => UserStatus::OFFLINE,
            _ => UserStatus::OFFLINE,
        }
    }
}

impl serde::ser::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("User", 5)?;
        s.serialize_field("_id", &self.id)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("password", &self.password)?;
        s.serialize_field("icon", &self.icon)?;
        s.serialize_field("status", &self.status)?;
        s.end()
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "User {{ id: '{id}', username : '{username}', passwd: '{passwd}', icon: '{icon}', status: '{status}' }}",
               id = self.id, username = self.name, passwd = self.password, icon = prettyprint_option_string(self.icon.clone()), status = format!("{:?}", self.status))
    }
}

impl UserProperties {
    pub fn get_default_header_name(property: &UserProperties) -> &'static str {
        match property {
            UserProperties::ID => "id",
            UserProperties::NAME => "name",
            UserProperties::PASSWORD => "password",
            UserProperties::ICON => "icon",
            UserProperties::STATUS => "status",
        }
    }
}

impl User {
    pub async fn new(name: String, password: String) -> Result<(), AzumaRejection> {
        // TODO: create new user
        Ok(())
    }

    pub async fn get(name: String) -> Result<(), AzumaRejection> {
        // TODO: get user by name
        Ok(())
    }

    pub async fn get_by_id(id: String) -> Result<(), AzumaRejection> {
        // TODO: get user by id
        Ok(())
    }

    pub async fn update(id: u64, updates: UpdatableUser) -> Result<(), AzumaRejection> {
        // TODO: update user
        Ok(())
    }
}
