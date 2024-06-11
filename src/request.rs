use std::collections::{BTreeMap, HashMap};
use crate::constant;
use crate::error::{Fail, Result};
use crate::utils::split;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Unknown,
    Options,
    Get,
    Post,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s {
            "OPTIONS" => HttpMethod::Options,
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => HttpMethod::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    Unknown,
    V1_1,
    V2_0,
}

impl From<&str> for HttpVersion {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => HttpVersion::V1_1,
            "HTTP/2.0" => HttpVersion::V2_0,
            _ => HttpVersion::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    url: &'a str,
    version: HttpVersion,
    ip: &'a str,
    headers: HashMap<String, &'a str>,
    search_params: HashMap<String, &'a str>,
    body: HashMap<String, Vec<u8>>,
}

impl<'a> HttpRequest<'a> {
    pub fn from(raw_header: &'a str, raw_body: Vec<u8>, ip: &'a str) -> Result<Self> {
        let mut header_lines = raw_header.lines();
        let request_line = header_lines.next().ok_or_else(|| Fail::new("获取请求行失败"))?;
        let (method, url, version) = parse_request_line(request_line)?;
        let headers = parse_headers(header_lines)?;
        let search_params = parse_parameters(url)?;
        let body = parse_body(&headers, &raw_body)?;
        Ok(Self {
            method,
            url,
            version,
            ip,
            headers,
            search_params,
            body,
        })
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn version(&self) -> &HttpVersion {
        &self.version
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn headers(&self) -> &HashMap<String, &'a str> {
        &self.headers
    }

    pub fn search_params(&self) -> &HashMap<String, &'a str> {
        &self.search_params
    }

    pub fn body(&self) -> &HashMap<String, Vec<u8>> {
        &self.body
    }

    pub fn body_utf8(&self) -> HashMap<String, String> {
        self.body.iter()
            .map(|(k, v)| (k.clone(), String::from_utf8_lossy(v).to_string()))
            .collect()
    }
}

fn parse_request_line(line: &str) -> Result<(HttpMethod, &str, HttpVersion)> {
    let mut parts = line.split_whitespace();
    let method = parts.next().ok_or_else(|| Fail::new("无法解析请求方法"))?.into();
    let url = parts.next().ok_or_else(|| Fail::new("无法解析请求地址"))?;
    let version = parts.next().ok_or_else(|| Fail::new("无法解析http协议版本"))?.into();
    Ok((method, url, version))
}

fn parse_headers<'a, I>(lines: I) -> Result<HashMap<String, &'a str>>
where
    I: Iterator<Item = &'a str>,
{
    let mut headers = HashMap::new();
    for line in lines {
        let mut parts = line.splitn(2, ':');
        let key = parts.next().ok_or_else(|| Fail::new("损坏的请求头"))?.trim().to_lowercase();
        let value = parts.next().ok_or_else(|| Fail::new("损坏的请求头"))?.trim();
        headers.insert(key, value);
    }
    Ok(headers)
}

fn parse_parameters(url: &str) -> Result<HashMap<String, &str>> {
    let mut params = HashMap::new();
    if let Some(query) = url.split('?').nth(1) {
        for param in query.split('&') {
            let mut parts = param.splitn(2, '=');
            let key = parts.next().ok_or_else(|| Fail::new("损坏的参数"))?.trim().to_lowercase();
            let value = parts.next().unwrap_or("").trim();
            params.insert(key, value);
        }
    }
    Ok(params)
}

fn parse_body(headers: &HashMap<String, &str>, body: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    let mut boundary = None;
    let content_type = headers.get("content-type").map_or(constant::TEXT_PLAIN, |s| {
        for part in s.split(';') {
            let part = part.trim();
            if part.starts_with("boundary=") {
                boundary = Some(part.split('=').nth(1).unwrap());
            }
        }
        s.trim()
    });

    if content_type.starts_with(constant::APPLICATION_X_WWW_FORM_URLENCODED) {
        parse_parameters(&String::from_utf8_lossy(body))
            .map(|params| params.into_iter().map(|(k, v)| (k, v.as_bytes().to_vec())).collect())
    } else if content_type.starts_with(constant::MULTIPART_FORM_DATA) {
        parse_multipart_form(body, boundary.ok_or_else(|| Fail::new("没有有效的boundary"))?)
    } else {
        let mut map = HashMap::new();
        map.insert(String::from("__raw"), body.to_vec());
        Ok(map)
    }
}

fn parse_multipart_form(body: &[u8], boundary: &str) -> Result<HashMap<String, Vec<u8>>> {
    let mut params = HashMap::new();
    let sections = split(body, format!("--{}\r\n", boundary).as_bytes());
    let last_sep = format!("--{}--\r\n", boundary);

    for section in sections {
        if section.ends_with(last_sep.as_bytes()) {
            continue;
        }
        let lines = split(&section, b"\r\n");
        let name = String::from_utf8_lossy(lines[0])
            .split(';')
            .find_map(|s| {
                if s.trim().starts_with("name=") {
                    Some(s.split('=').nth(1).unwrap().trim_matches('"').to_lowercase())
                } else {
                    None
                }
            })
            .ok_or_else(|| Fail::new("表单内容没有name属性"))?;
        
        let data_line = lines.iter().position(|&l| l.is_empty())
            .and_then(|idx| lines.get(idx + 1))
            .ok_or_else(|| Fail::new("表单内容损坏"))?;
        
        params.insert(name, data_line.to_vec());
    }
    Ok(params)
}
