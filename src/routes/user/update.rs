use crate::models::user::User;
use crate::util::{get_header_value, get_header_value_simple};
use crate::{models::etc::DefaultResponse, placeholder_route};
use actix_web::{body::Body, HttpRequest, HttpResponse};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub async fn update_user(req: HttpRequest) -> HttpResponse {
    //TODO: Implement session check instead of username/passwd check
    // TODO: update user
    placeholder_route(req)
}

/*pub fn generate_updated_user(req: &HttpRequest) -> UpdatableUser {
    /*let mut update: HashMap<UserProperties, String> = HashMap::new();
    for property in UserProperties::iter() {
        UserProperties::get_default_header_name(&property);
        match get_header_value(
            req,
            format!("new_{}", UserProperties::get_default_header_name(&property)).as_str(),
        ) {
            Some(header) => update.insert(property, header),
            _ => None,
        };
    }*/
    let mut user = UpdatableUser::new();
    /*for property in update {
        user = UpdatableUser::update_property(user, property.0, property.1);
    }*/
    user
}

#[derive(Clone, Debug)]
pub struct UpdatableUser {
    pub id: Option<u64>,
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
    fn update_property(
        mut user: UpdatableUser,
        property: UserProperties,
        value: String,
    ) -> UpdatableUser {
        match property {
            UserProperties::NAME => user.name = Some(value),
            UserProperties::PASSWORD => user.password = Some(value),
            UserProperties::ICON => user.icon = Some(Some(value)),
            UserProperties::STATUS => user.status = Some(UserStatus::from_string(value)),
            _ => {}
        }
        user
    }
}*/
