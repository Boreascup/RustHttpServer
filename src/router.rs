use super::handler::{Handler, StaticHandler};
use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub struct Router;

impl Router {
    pub fn route<'a>(req: HttpRequest) -> HttpResponse<'a> {
        StaticHandler::handle(&req)
    }
}
