use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::error::{Fail, Result};
use crate::request::HttpRequest;
use crate::response::{HttpResponse, HttpStatus};
use crate::router::Router;

#[derive(Clone, Debug)]
pub struct HttpSettings {
    pub max_header_size: usize,
    pub max_body_size: usize,
    pub header_buffer: usize,
    pub body_buffer: usize,
    pub header_read_attempts: usize,
    pub body_read_attempts: usize,
}

impl HttpSettings {
    pub fn new() -> Self {
        Self {
            max_header_size: 8192,
            max_body_size: 8192 * 1024,
            header_buffer: 8192,
            body_buffer: 8192,
            header_read_attempts: 3,
            body_read_attempts: 3,
        }
    }
}

pub struct Server {
    socket_addr: SocketAddr,
    http_settings: Arc<HttpSettings>,
}

impl Server {
    pub fn new(addr: &str, http_settings: HttpSettings) -> Self {
        let socket_addr = addr.parse().unwrap();
        let http_settings = Arc::new(http_settings);
        Self { socket_addr, http_settings }
    }

    pub async fn run(&self) -> Result<()> {
        let conn_listener = TcpListener::bind(self.socket_addr).await?;
        println!("Running on {}", self.socket_addr);
        loop {
            if let Ok((stream, address)) = conn_listener.accept().await {
                let http_settings = self.http_settings.clone();
                tokio::spawn(async move {
                    let mut stream = stream;
                    match handle_conn(&http_settings, &mut stream, address).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                            write_stream(&mut stream, HttpResponse::new(HttpStatus::BadRequest, None, Some(err.to_string().as_bytes().to_vec())).to_vec()).await;
                        }
                    };
                });
            }
        }
    }
}

async fn handle_conn(http_settings: &HttpSettings,
                     mut stream: &mut TcpStream,
                     addr: SocketAddr) -> Result<()> {
    let (header, mut body) = read_head(http_settings, &mut stream).await?;
    let content_length = get_content_length(header.as_str());
    if content_length > 0 {
        read_body(&http_settings, &mut stream, &mut body, content_length).await?;
    }
    let ip = addr.ip().to_string();
    let request = HttpRequest::from(&header, body, &ip[..])?;
    let response = Router::route(request);
    write_stream(stream, response.to_vec()).await;
    Ok(())
}

async fn write_stream(stream: &mut TcpStream, content: Vec<u8>) {
    match stream.write_all(&content).await {
        Ok(_) => {
            match stream.flush().await {
                Ok(_) => {}
                Err(err) => { println!("{}", err); }
            }
        }
        Err(err) => { println!("{}", err); }
    };
}

async fn read_head(http_settings: &HttpSettings, stream: &mut TcpStream) -> Result<(String, Vec<u8>)> {
    let mut header = Vec::new();
    let mut body = Vec::new();
    let mut buf = vec![0u8; http_settings.header_buffer];
    let mut read_fails = 0;

    while let Ok(length) = stream.read(&mut buf).await {
        if length == 0 {
            break;
        }

        if header.len() + length > http_settings.max_header_size {
            return Fail::from("请求头大小超出限制");
        }

        let buf = &buf[..length];
        header.extend_from_slice(buf);

        if let Some(pos) = header.windows(4).position(|window| window == b"\r\n\r\n") {
            let (head, rest) = header.split_at(pos + 4);
            body.extend_from_slice(rest);
            return Ok((String::from_utf8(head.to_vec())?, body));
        }

        if length < http_settings.header_buffer {
            read_fails += 1;
            if read_fails > http_settings.header_read_attempts {
                return Fail::from("读取请求头失败");
            }
        }
    }

    Fail::from("请求头读取失败")
}

fn get_content_length(head: &str) -> usize {
    let mut size: usize = 0;
    for hl in head.lines() {
        let mut split_hl = hl.splitn(2, ":");
        if let (Some(key), Some(value)) = (split_hl.next(), split_hl.next()) {
            if key.trim().to_lowercase().eq("content-length") {
                size = match value.parse::<usize>() {
                    Ok(s) => s,
                    Err(_) => 0
                };
            }
        }
    }
    size
}

async fn read_body(http_settings: &HttpSettings,
                   stream: &mut TcpStream,
                   body: &mut Vec<u8>,
                   content_len: usize) -> Result<()> {
    if content_len > http_settings.max_body_size {
        return Err(Fail::new("请求体大小超出限制"));
    }
    let mut read_fails = 0;
    while body.len() < content_len {
        let rest_len = content_len - body.len();
        let buf_len = if rest_len > http_settings.body_buffer {
            http_settings.body_buffer
        } else { rest_len };
        let mut buf = vec![0u8; buf_len];
        let length = match stream.read(&mut buf).await {
            Ok(len) => { len }
            Err(_) => { return Err(Fail::new("请求体读取失败")); }
        };
        buf.truncate(length);
        body.append(&mut buf);
        if length < http_settings.body_buffer {
            read_fails += 1;
            if read_fails > http_settings.body_read_attempts {
                return Fail::from("请求体读取失败");
            }
        }
    }
    Ok(())
}
