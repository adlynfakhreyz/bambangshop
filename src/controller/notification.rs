use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscribers>")]
pub fn subscribe(product_type: &str, subscribers: Json<Subscribers>) -> Result<Created<Json<Subscribers>>> {
    return match NotificationService::subscribe(product_type, subscribers.into_inner()) {
        Ok(f) => Ok(Created::new("/").body(Json::from(f))),
        Err(e) => Err(e)
    };
}
