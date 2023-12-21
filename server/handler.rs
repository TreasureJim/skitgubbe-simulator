use warp::reply::Reply;

use crate::{RegisterRequest, Clients};

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {

}
