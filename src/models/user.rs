use bson::{doc, from_bson, oid::ObjectId, to_bson, Bson::Document, Document as StructDoc};
use pbkdf2::pbkdf2_simple;
use serde::{Deserialize, Serialize};
use crate::models::rejection::AzumaRejection;
use crate::util::{get_mongodb, prettyprint_option_string};
use serde::ser::{SerializeStruct, Serializer};
use strum_macros::EnumIter;
use crate::routes::user::update::UpdatableUser;
use mongodb::results::UpdateResult;


#[derive(Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub password: String,
    pub icon: Option<String>,
    pub status: UserStatus,
}


#[derive(Debug, EnumIter, Hash, PartialEq, Eq)]
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
            UserProperties::STATUS => "status"
        }
    }
}


impl User {
    pub async fn new(name: String, password: String) -> Result<User, AzumaRejection> {
        #[allow(non_snake_case)]
            let AZUMA_DB = match get_mongodb().await {
            Ok(user) => user,
            Err(_) => return Err(AzumaRejection::InternalServerError)
        };
        let coll = AZUMA_DB.collection("users");
        match coll.find_one(Some(doc! { "name": name.clone() }), None).await {
            Ok(doc) => match doc {
                None => match pbkdf2_simple(&password, 10000) {
                    Ok(hashed_password) => {
                        let user = User {
                            id: ObjectId::new(),
                            name,
                            password: hashed_password,
                            icon: None,
                            status: UserStatus::OFFLINE,
                        };

                        match coll.insert_one(
                            to_bson(&user).unwrap().as_document().unwrap().clone(),
                            None,
                        ).await
                        {
                            Ok(_) => Ok(user),
                            Err(_) => Err(AzumaRejection::InternalServerError),
                        }
                    }
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                Some(_) => Err(AzumaRejection::AlreadyExists),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub async fn get(name: String) -> Result<User, AzumaRejection> {
        #[allow(non_snake_case)]
            let AZUMA_DB = match get_mongodb().await {
            Ok(user) => user,
            Err(_) => return Err(AzumaRejection::InternalServerError)
        };
        let coll = AZUMA_DB.collection("users");
        match coll.find_one(Some(doc! { "name": name }), None).await {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<User>(Document(doc)) {
                    Ok(user_result) => Ok(user_result),
                    Err(_) =>
                        Err(AzumaRejection::InternalServerError),
                },
                None => Err(AzumaRejection::NotFound),
            },
            Err(_) =>
                Err(AzumaRejection::InternalServerError),
        }
    }


    pub async fn get_by_id(id: String) -> Result<User, AzumaRejection> {
        #[allow(non_snake_case)]
            let AZUMA_DB = match get_mongodb().await {
            Ok(user) => user,
            Err(_) => return Err(AzumaRejection::InternalServerError)
        };
        let obj_id = match ObjectId::with_string(id.as_str()) {
            Ok(id) => id,
            Err(_) => return Err(AzumaRejection::BadRequest)
        };
        let coll = AZUMA_DB.collection("users");
        match coll.find_one(Some(doc! { "_id": obj_id }), None).await {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<User>(Document(doc)) {
                    Ok(user_result) => Ok(user_result),
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                None => Err(AzumaRejection::NotFound),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub async fn update(id: ObjectId, updates: UpdatableUser) -> Result<UpdateResult, AzumaRejection> {
        let old_user = match User::get_by_id(id.to_string()).await {
            Ok(user) => user,
            Err(err) => {
                return Err(err);
            }
        };

        #[allow(non_snake_case)]
            let AZUMA_DB = match get_mongodb().await {
            Ok(user) => user,
            Err(_) => return Err(AzumaRejection::InternalServerError)
        };
        let coll = AZUMA_DB.collection("users");
        let doc = generate_updated_document(old_user, &updates);
        if doc.is_empty() {
            return Err(AzumaRejection::BadRequest);
        }
        let res = match coll.update_one(doc! { "_id": id }, doc, None).await {
            Ok(res) => {
                println!("update result: {:?}", res);
                res
            }
            Err(err) => {
                println!("update err: {:?}", err);
                return Err(AzumaRejection::InternalServerError);
            }
        };

        Ok(res)
    }
}

//TODO: Hash password before saving it
fn generate_updated_document(old_user: User, updates: &UpdatableUser) -> StructDoc {
    let mut doc = StructDoc::new();
    if updates.name != None {
        doc.insert("name", updates.name.as_ref().unwrap());
    } else {
        doc.insert("name", old_user.name);
    }
    if updates.password != None {
        doc.insert("password", updates.password.as_ref().unwrap());
    } else {
        doc.insert("password", old_user.password);
    }
    if updates.icon != None {
        //TODO fix "socket hang up" error in case of no image
        doc.insert("icon", match updates.icon.as_ref().unwrap() {
            None => "".to_string(),
            Some(opt) => opt.to_string(),
        });
    } else {
        doc.insert("icon", old_user.icon.unwrap());
    }
    if !(updates.status.eq(&None)) {
        doc.insert("status", format!("{:?}", updates.status.unwrap()));
    } else {
        doc.insert("status", format!("{:?}", old_user.status));
    }
    doc
}