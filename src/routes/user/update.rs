use actix_web::{HttpRequest, HttpResponse, body::Body};
use crate::util::{get_header_value_simple, get_header_value};
use crate::models::user::{User, UserProperties, UserStatus};
use crate::models::rejection::{handle_rejection};
use pbkdf2::{pbkdf2_check};
use strum::IntoEnumIterator;
use std::collections::HashMap;
use bson::oid::ObjectId;
use crate::models::etc::DefaultResponse;

pub async fn update_user(req: HttpRequest) -> HttpResponse {

    //TODO: Implement session check instead of username/passwd check

    let username = match get_header_value_simple(&req, "name") {
        Ok(name) => name,
        Err(err) => return err,
    };

    let passwd = match get_header_value_simple(&req, "password") {
        Ok(name) => name,
        Err(err) => return err,
    };

    let fetched_user = match User::get(username).await {
        Ok(user) => user,
        Err(err) => return handle_rejection(&req, err),
    };


    match pbkdf2_check(passwd.as_str(), fetched_user.password.as_str()) {
        Ok(_) => (),
        Err(_) => return HttpResponse::Unauthorized().body(Body::from("Password don't match")),
    };


    let update_result = User::update(fetched_user.id, generate_updated_user(&req)).await;

    let success = DefaultResponse { code: 200, message: "Successfully updated the user".to_string()};
    let fail = DefaultResponse { code: 417, message: "The requested user does not exist".to_string()};

    return match update_result {
        Ok(result) => match result.modified_count {
            1 => HttpResponse::Ok().body(Body::from(match serde_json::to_string(&success) {
                Ok(res) => res,
                Err(_) => return HttpResponse::InternalServerError().body(Body::from("An error occurred while parsing the response"))
            })),
            _ => HttpResponse::ExpectationFailed().body(Body::from(match serde_json::to_string(&fail) {
                Ok(res) => res,
                Err(_) => return HttpResponse::InternalServerError().body(Body::from("An error occurred while parsing the response"))
            }))
        },
        Err(err) => handle_rejection(&req, err),
    }
}


pub fn generate_updated_user(req: &HttpRequest) -> UpdatableUser {
    let mut update: HashMap<UserProperties, String> = HashMap::new();
    for property in UserProperties::iter() {
        UserProperties::get_default_header_name(&property);
        match get_header_value(req, format!("new_{}", UserProperties::get_default_header_name(&property)).as_str()) {
            Some(header) => update.insert(property, header),
            _ => None,
        };
    }
    let mut user = UpdatableUser::new();
    for property in update {
        user = UpdatableUser::update_property(user, property.0, property.1);
    }
    user
}

#[derive(Clone, Debug)]
pub struct UpdatableUser {
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub icon: Option<Option<String>>,
    pub status: Option<UserStatus>,
}

impl UpdatableUser {
    pub fn new() -> UpdatableUser {
        UpdatableUser {
            id: None,
            name: None,
            password: None,
            icon: None,
            status: None,
        }
    }
    fn update_property(mut user: UpdatableUser, property: UserProperties, value: String) -> UpdatableUser {
        match property {
            UserProperties::NAME => user.name = Some(value),
            UserProperties::PASSWORD => user.password = Some(value),
            UserProperties::ICON => user.icon = Some(Some(value)),
            UserProperties::STATUS => user.status = Some(UserStatus::from_string(value)),
            _ => {}
        }
        user
    }
}