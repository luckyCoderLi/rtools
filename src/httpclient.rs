use std::collections::HashMap;
use std::time::Duration;
use crate::{RtoolsResult, RtoolsError};

/// HTTP请求方法
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "HEAD" => Some(Self::HEAD),
            "OPTIONS" => Some(Self::OPTIONS),
            _ => None,
        }
    }
}

/// HTTP请求配置
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: Option<Duration>,
    pub follow_redirects: bool,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::GET,
            url: String::new(),
            headers: HashMap::new(),
            body: None,
            timeout: Some(Duration::from_secs(30)),
            follow_redirects: true,
        }
    }
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            method,
            url,
            ..Default::default()
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    pub fn with_follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }
}

/// HTTP响应信息
#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub content_length: Option<usize>,
    pub content_type: Option<String>,
    pub response_time_ms: u128,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn print_response(&self) {
        println!("HTTP响应信息:");
        println!("- 状态码: {} {}", self.status_code, self.status_text);
        println!("- 响应时间: {} ms", self.response_time_ms);
        
        if let Some(content_type) = &self.content_type {
            println!("- 内容类型: {}", content_type);
        }
        
        if let Some(content_length) = self.content_length {
            println!("- 内容长度: {} 字节", content_length);
        }
        
        if !self.headers.is_empty() {
            println!("\n响应头:");
            for (key, value) in &self.headers {
                println!("  {}: {}", key, value);
            }
        }
        
        if !self.body.is_empty() {
            println!("\n响应体 (前500字符):");
            let preview = if self.body.len() > 500 {
                format!("{}...", &self.body[..500])
            } else {
                self.body.clone()
            };
            println!("{}", preview);
        }
    }
    
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
    
    pub fn is_redirect(&self) -> bool {
        self.status_code >= 300 && self.status_code < 400
    }
    
    pub fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }
    
    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500 && self.status_code < 600
    }
}

/// 发送HTTP请求
pub async fn send_request(request: HttpRequest) -> RtoolsResult<HttpResponse> {
    let start_time = std::time::Instant::now();
    
    // 创建HTTP客户端
    let mut client_builder = reqwest::Client::builder();
    
    if let Some(timeout) = request.timeout {
        client_builder = client_builder.timeout(timeout);
    }
    
    if !request.follow_redirects {
        client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
    }
    
    let client = client_builder
        .build()
        .map_err(|e| RtoolsError::NetworkError(format!("客户端创建失败: {}", e)))?;
    
    // 构建请求
    let mut req_builder = match request.method {
        HttpMethod::GET => client.get(&request.url),
        HttpMethod::POST => client.post(&request.url),
        HttpMethod::PUT => client.put(&request.url),
        HttpMethod::DELETE => client.delete(&request.url),
        HttpMethod::HEAD => client.head(&request.url),
        HttpMethod::OPTIONS => client.request(reqwest::Method::OPTIONS, &request.url),
    };
    
    // 添加请求头
    for (key, value) in request.headers {
        req_builder = req_builder.header(key, value);
    }
    
    // 添加请求体
    if let Some(body) = request.body {
        req_builder = req_builder.body(body);
    }
    
    // 发送请求
    let response = req_builder
        .send()
        .await
        .map_err(|e| RtoolsError::NetworkError(format!("请求失败: {}", e)))?;
    
    let response_time = start_time.elapsed();
    
    // 构建响应对象
    let mut http_response = HttpResponse::new();
    http_response.status_code = response.status().as_u16();
    http_response.status_text = response.status().canonical_reason().unwrap_or("").to_string();
    http_response.response_time_ms = response_time.as_millis();
    
    // 获取响应头
    for (key, value) in response.headers() {
        if let Ok(value_str) = value.to_str() {
            http_response.headers.insert(key.to_string(), value_str.to_string());
        }
    }
    
    // 获取内容类型和长度
    if let Some(content_type) = response.headers().get("content-type") {
        http_response.content_type = content_type.to_str().ok().map(|s| s.to_string());
    }
    
    if let Some(content_length) = response.headers().get("content-length") {
        if let Ok(length) = content_length.to_str().unwrap_or("0").parse::<usize>() {
            http_response.content_length = Some(length);
        }
    }
    
    // 获取响应体
    if request.method != HttpMethod::HEAD {
        http_response.body = response
            .text()
            .await
            .map_err(|e| RtoolsError::NetworkError(format!("读取响应体失败: {}", e)))?;
    }
    
    Ok(http_response)
}

/// 简单的GET请求
pub async fn get(url: &str) -> RtoolsResult<HttpResponse> {
    let request = HttpRequest::new(HttpMethod::GET, url.to_string());
    send_request(request).await
}

/// 简单的POST请求
pub async fn post(url: &str, body: &str) -> RtoolsResult<HttpResponse> {
    let request = HttpRequest::new(HttpMethod::POST, url.to_string())
        .with_body(body.to_string());
    send_request(request).await
}

/// 检查URL是否可访问
pub async fn check_url(url: &str) -> RtoolsResult<bool> {
    let request = HttpRequest::new(HttpMethod::HEAD, url.to_string())
        .with_timeout(Duration::from_secs(10));
    
    match send_request(request).await {
        Ok(response) => Ok(response.is_success()),
        Err(_) => Ok(false),
    }
} 