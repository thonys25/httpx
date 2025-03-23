use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::collections::HashMap;

pub type RouteHandler = fn(HttpRequest, &mut HttpResponse);

pub struct Router {
    static_routes: HashMap<String, RouteHandler>,
    dynamic_routes: HashMap<String, RouteHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            static_routes: HashMap::new(),
            dynamic_routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: &str, path: &str, handler: RouteHandler) {
        let key = format!("{} {}", method, path);
        if path.contains(':') {
            // Динамический маршрут
            self.dynamic_routes.insert(key, handler);
        } else {
            // Статический маршрут
            self.static_routes.insert(key, handler);
        }
    }

    pub async fn listen(&self, address: &str) {
        let listener = TcpListener::bind(address).await.expect("Failed to bind address");
        println!("Listening on {}", address);

        while let Ok((stream, _)) = listener.accept().await {
            let static_routes = self.static_routes.clone();
            let dynamic_routes = self.dynamic_routes.clone();
            task::spawn(async move {
                let mut router = Router {
                    static_routes,
                    dynamic_routes,
                };
                router.handle_connection(stream).await;
            });
        }
    }

    async fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = vec![0; 1024];
        if stream.read(&mut buffer).await.is_err() {
            return;
        }

        let request_str = String::from_utf8_lossy(&buffer[..]);

        if let Some(mut request) = HttpRequest::parse(&request_str) {
            if let Some(handler) = self.find_route(&mut request) {
                let mut response = HttpResponse::new(&request.protocol);
                handler(request, &mut response);
                response.send(&mut stream).await;
            } else {
                //println!("Returning 404 for {} {}", request.method, request.path); // Отладочное сообщение
                self.not_found(&mut stream).await;
            }
        } else {
            self.bad_request(&mut stream).await;
        }
    }

    fn find_route(&self, request: &mut HttpRequest) -> Option<&RouteHandler> {
        // Сначала проверяем статические маршруты
        let static_key = format!("{} {}", request.method, request.path);
        //println!("Checking static route: {}", static_key); // Отладочное сообщение
        if let Some(handler) = self.static_routes.get(&static_key) {
            //println!("Static route found: {}", static_key); // Отладочное сообщение
            return Some(handler);
        }

        // Проверяем, есть ли статический маршрут для этого пути (даже если метод другой)
        for (key, _) in &self.static_routes {
            let parts: Vec<&str> = key.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == request.path {
                //println!("Static path exists, but method mismatch: {}", request.path); // Отладочное сообщение
                return None; // Возвращаем None, чтобы вызвать 404
            }
        }

        // Затем проверяем динамические маршруты
        //println!("Checking dynamic routes..."); // Отладочное сообщение
        for (route, handler) in &self.dynamic_routes {
            if self.match_route(route, request) {
                //println!("Dynamic route matched: {}", route); // Отладочное сообщение
                return Some(handler);
            }
        }

        // Если ничего не найдено
        //println!("No route found for {} {}", request.method, request.path); // Отладочное сообщение
        None
    }

    fn is_static_path(&self, route_path: &str, request_path: &str) -> bool {
        // Проверяем, есть ли статический маршрут, который соответствует пути
        let static_key = format!("GET {}", request_path); // Предполагаем GET для примера
        self.static_routes.contains_key(&static_key)
    }

    fn match_route(&self, route: &str, request: &mut HttpRequest) -> bool {
        let route_parts: Vec<&str> = route.split_whitespace().collect();
        if route_parts[0] == request.method
            && self.is_path_match(route_parts[1], &request.path, &mut request.params)
        {
            println!("Route matched: {}", route); // Отладочное сообщение
            return true;
        }
        false
    }

    fn is_path_match(&self, route_path: &str, request_path: &str, params: &mut HashMap<String, String>) -> bool {
        let route_segments: Vec<&str> = route_path.split('/').collect();
        let request_segments: Vec<&str> = request_path.split('/').collect();

        if route_segments.len() != request_segments.len() {
            println!("Path length mismatch: {} != {}", route_path, request_path); // Отладочное сообщение
            return false;
        }

        for (route_seg, req_seg) in route_segments.iter().zip(request_segments.iter()) {
            if route_seg.starts_with(':') {
                if req_seg.is_empty() {
                    println!("Empty segment for dynamic route: {}", route_path); // Отладочное сообщение
                    return false;
                }
                let param_name = &route_seg[1..];
                params.insert(param_name.to_string(), req_seg.to_string());
            } else if route_seg != req_seg {
                println!("Segment mismatch: {} != {}", route_seg, req_seg); // Отладочное сообщение
                return false;
            }
        }
        println!("Path matched: {} == {}", route_path, request_path); // Отладочное сообщение
        true
    }

    async fn not_found(&self, stream: &mut TcpStream) {
        let response = "HTTP/1.1 404 Not Found\r\nServer: httpx\r\n\r\nNot Found";
        stream.write_all(response.as_bytes()).await.unwrap();
    }

    async fn bad_request(&self, stream: &mut TcpStream) {
        let response = "HTTP/1.1 400 Bad Request\r\nServer: httpx\r\n\r\nBad Request";
        stream.write_all(response.as_bytes()).await.unwrap();
    }
}

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub query_params: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(request_str: &str) -> Option<Self> {
        let mut lines = request_str.lines();
        let first_line = lines.next()?;
        let mut parts = first_line.split_whitespace();
        let method = parts.next()?.to_string();
        let full_path = parts.next()?.to_string();
        let protocol = parts.next()?.to_string();

        let (path, query_string) = if let Some(index) = full_path.find('?') {
            (&full_path[..index], &full_path[index + 1..])
        } else {
            (&full_path[..], "")
        };

        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut is_body = false;

        for line in lines {
            if line.is_empty() {
                is_body = true;
                continue;
            }

            if is_body {
                body.push_str(line);
                body.push('\n');
            } else {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }

        let query_params = query_string
            .split('&')
            .filter_map(|pair| {
                let mut split = pair.splitn(2, '=');
                Some((
                    split.next()?.to_string(),
                    split.next().unwrap_or_default().to_string(),
                ))
            })
            .collect();

        Some(Self {
            method,
            path: path.to_string(),
            protocol,
            query_params,
            params: HashMap::new(),
            headers,
            body: body.trim().to_string(),
        })
    }

    // pub fn get_body(&self) -> &str {
    //     &self.body
    // }

    pub fn get_body_raw(&self) -> String {
        return self.read_until_null(&self.body)
    }

    fn read_until_null(&self, data: &String) -> String {
        let bytes = data.as_bytes(); // Преобразуем строку в байтовый срез
        let null_terminated_data = bytes.split(|&x| x == 0).next().unwrap_or(&[]);
        String::from_utf8_lossy(null_terminated_data).into_owned()
    }

    pub fn get_body(&self) -> HashMap<String, String> {
        let mut form_data = HashMap::new();
        if self
            .headers
            .get("Content-Type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false)
        {
            for pair in self.body.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    form_data.insert(key.to_string(), value.to_string());
                }
            }
        }
        form_data
    }

    pub fn get_method(&self) -> &str {
        return &self.method
    }

    pub fn get_param(&self, name: &str) -> &str {
        // Проверяем параметры пути
        if let Some(value) = self.params.get(name) {
            // Если параметр содержит "&", берём только часть до "&"
            return value.split('&').next().unwrap_or("");
        }

        // Проверяем параметры строки запроса
        if let Some(query) = self.query_params.get(name) {
            return query;
        }

        ""
    }

    pub fn get_query_param(&self, name: &str) -> &str {
        self.query_params.get(name).map(|s| s.as_str()).unwrap_or("")
    }
}

pub struct HttpResponse {
    header: String,
    headers: HashMap<String, String>,
    body: String,
    protocol: String,
}

pub enum StatusCode {
    // 1xx
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 2xx
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // 3xx
    MultipleChoices,
    MovedPermanently,
    Found,
    MovedTemporarily,
    SeeOther,
    NotModified,
    UseProxy,
    SwitchProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImTeapot,
    AuthenticationTimeout,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    RetryWith,
    UnavailableForLegalReasons,
    ClientClosedRequest,

    // 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
    UnknownError,
    WebServerIsDown,
    ConnectionTimedOut,
    OriginIsUnreachable,
    TimeoutOccurred,
    SSLHandshakeFailed,
    InvalidSSLCertificate,
}

impl StatusCode {
    fn value(&self) -> &str {
        match *self {
            // 1xx
            StatusCode::Continue => "100 Continue",
            StatusCode::SwitchingProtocols => "101 Switching Protocols",
            StatusCode::Processing => "102 Processing",
            StatusCode::EarlyHints => "103 Early Hints",

            // 2xx
            StatusCode::OK => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::Accepted => "202 Accepted",
            StatusCode::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            StatusCode::NoContent => "204 No Content",
            StatusCode::ResetContent => "205 Reset Content",
            StatusCode::PartialContent => "206 Partial Content",
            StatusCode::MultiStatus => "207 Multi-Status",
            StatusCode::AlreadyReported => "208 Already Reported",
            StatusCode::IMUsed => "226 IM Used",

            // 3xx
            StatusCode::MultipleChoices => "300 Multiple Choices",
            StatusCode::MovedPermanently => "301 Moved Permanently",
            StatusCode::Found => "302 Found",
            StatusCode::SeeOther => "303 See Other",
            StatusCode::NotModified => "304 Not Modified",
            StatusCode::UseProxy => "305 Use Proxy",
            StatusCode::SwitchProxy => "306 Switch Proxy",
            StatusCode::TemporaryRedirect => "307 Temporary Redirect",
            StatusCode::PermanentRedirect => "308 Permanent Redirect",

            // 4xx
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::Unauthorized => "401 Unauthorized",
            StatusCode::PaymentRequired => "402 Payment Required",
            StatusCode::Forbidden => "403 Forbidden",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::MethodNotAllowed => "405 Method Not Allowed",
            StatusCode::NotAcceptable => "406 Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            StatusCode::RequestTimeout => "408 Request Timeout",
            StatusCode::Conflict => "409 Conflict",
            StatusCode::Gone => "410 Gone",
            StatusCode::LengthRequired => "411 Length Required",
            StatusCode::PreconditionFailed => "412 Precondition Failed",
            StatusCode::PayloadTooLarge => "413 Payload Too Large",
            StatusCode::URITooLong => "414 URI Too Long",
            StatusCode::UnsupportedMediaType => "415 Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "416 Range Not Satisfiable",
            StatusCode::ExpectationFailed => "417 Expectation Failed",
            StatusCode::ImTeapot => "418 I'm a teapot ",
            StatusCode::AuthenticationTimeout => "419 Authentication Timeout",
            StatusCode::MisdirectedRequest => "421 Misdirected Request",
            StatusCode::UnprocessableEntity => "422 Unprocessable Entity",
            StatusCode::Locked => "423 Locked",
            StatusCode::FailedDependency => "424 Failed Dependency",
            StatusCode::TooEarly => "425 Too Early",
            StatusCode::UpgradeRequired => "426 Upgrade Required",
            StatusCode::PreconditionRequired => "428 Precondition Required",
            StatusCode::TooManyRequests => "429 Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            StatusCode::RetryWith => "449 Retry With",
            StatusCode::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
            StatusCode::ClientClosedRequest => "499 Client Closed Request",
            
            // 5xx
            StatusCode::InternalServerError => "500 Internal Server Error",
            StatusCode::NotImplemented => "501 Not Implemented",
            StatusCode::BadGateway => "502 Bad Gateway",
            StatusCode::ServiceUnavailable => "503 Service Unavailable",
            StatusCode::GatewayTimeout => "504 Gateway Timeout",
            StatusCode::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            StatusCode::InsufficientStorage => "507 Insufficient Storage",
            StatusCode::LoopDetected => "508 Loop Detected",
            StatusCode::BandwidthLimitExceeded => "509 Bandwidth Limit Exceeded",
            StatusCode::NotExtended => "510 Not Extended",
            StatusCode::NetworkAuthenticationRequired => "511 Network Authentication Required",
            StatusCode::UnknownError => "520 Unknown Error",
            StatusCode::WebServerIsDown => "521 Web Server Is Down",
            StatusCode::ConnectionTimedOut => "522 Connection Timed Out",
            StatusCode::OriginIsUnreachable => "523 Origin Is Unreachable",
            StatusCode::TimeoutOccurred => "524 A Timeout Occurred",
            StatusCode::SSLHandshakeFailed => "525 SSL Handshake Failed",
            StatusCode::InvalidSSLCertificate => "526 Invalid SSL Certificate",

            // Default value should always be
            _ => "200 OK",
        }
    }
}

impl HttpResponse {
    pub fn new(protocol: &str) -> Self {
        HttpResponse {
            header: "200 OK".to_string(),
            headers: HashMap::new(),
            body: String::new(),
            protocol: protocol.to_string(),
        }
    }

    pub fn set_header(&mut self, header: &str) {
        self.header = header.to_string();
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.header = status_code.value().to_string();
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn write(&mut self, content: &str) {
        self.body = content.to_string();
    }

    pub async fn send(self, stream: &mut TcpStream) {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        let response = format!(
            "{} {}\r\n{}\r\n\r\n{}",
            self.protocol, self.header, headers, self.body
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    }
}
