use crate::constant;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// HTTP状态码
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    NotFound,
    InternalServerError,
}

/// 实现 HttpStatus 的字符串表示方法
impl HttpStatus {
    fn to_str(&self) -> &str {
        match self {
            HttpStatus::Ok => "200 OK",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
        }
    }
}

/// HTTP响应
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: Cow<'a, str>,
    status: HttpStatus,
    headers: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    body: Option<Vec<u8>>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        let mut response = Self {
            version: Cow::Borrowed("HTTP/1.1"),
            status: HttpStatus::Ok,
            headers: BTreeMap::new(),
            body: None,
        };
        response.headers.insert(
            Cow::Borrowed("Content-Type"),
            Cow::Borrowed(constant::TEXT_PLAIN),
        );
        response.headers.insert(
            Cow::Borrowed("Server"),
            Cow::Borrowed("FlapyPan/my-http-server"),
        );
        response
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new<S>(
        status: HttpStatus,
        headers: Option<BTreeMap<S, S>>,
        body: Option<Vec<u8>>,
    ) -> HttpResponse<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        response.status = status;
        if let Some(hs) = headers {
            for (k, v) in hs {
                response.headers.insert(k.into(), v.into());
            }
        }
        response.body = body;
        response
    }

    pub fn not_found(body: Option<Vec<u8>>) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        response.status = HttpStatus::NotFound;
        response.headers.insert(
            Cow::Borrowed("Content-Type"),
            Cow::Borrowed(constant::TEXT_HTML),
        );
        response.body = body;
        response
    }

    fn headers(&self) -> String {
        let mut header_string = String::new();
        for (k, v) in &self.headers {
            header_string.push_str(&format!("{}: {}\r\n", k, v));
        }
        header_string
    }

    /// 转换为字节数组
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = format!(
            "{} {}\r\n{}Content-Length: {}\r\n\r\n",
            &self.version,
            &self.status.to_str(),
            &self.headers(),
            self.body.as_ref().map_or(0, |b| b.len()),
        )
        .as_bytes()
        .to_vec();

        if let Some(b) = &self.body {
            vec.extend_from_slice(b);
        }

        vec
    }
}
