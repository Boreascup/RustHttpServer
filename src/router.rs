use crate::request::HttpRequest;
use crate::response::HttpResponse;
use super::handler::{Handler, StaticHandler};

pub struct Router;

impl Router {
    pub fn route<'a>(req: HttpRequest) -> HttpResponse<'a> {
        StaticHandler::handle(&req)
    }
}