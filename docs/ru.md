# Документация

## HttpRequest

1. get_body_raw - Метод получения необработанных данных из тела, переданного, например, через заголовок
```
application/json
```

Пример:
```
router.add_route("POST", "/article/create", |req, res| {
	let body = req.get_body_raw();
});
```

2. get_body - Метод получения данных из тела. Предназначен исключительно для получения данных FormData. Если форма была отправлена ​​как:
```
application/x-www-form-urlencoded
```

Пример:
```
router.add_route("POST", "/article/create", |req, res| {
	let form_data = req.get_body();

	let title = match form_data.get("title") {
        Some(value) => value.to_string(),
        None => String::new(),
    };

    let text = match form_data.get("text") {
        Some(value) => value.to_string(),
        None => String::new(),
    };

    println!("Title: {}", title);
    println!("Text: {}", text);
});
```

3. get_method - Получить имя метода, который сделал запрос. Если запрос на сервер был отправлен методом GET, вернет GET, если POST, тогда POST и так далее.

Пример:
```
router.add_route("GET", "/", |req, res| {
	let method = req.get_method();
});
```

4. get_param - Метод получения параметризованных динамических значений из маршрута. Пример:
```
Запрос: http://127.0.0.1:8080/articles/1

В коде:

router.add_route("GET", "/articles/:id", |req, res| {
	let id = req.get_param("id"); // <- Получить идентификатор, значение которого может быть динамическим
});
```

5. get_query_param - Метод получения параметра из строки запроса. Пример:
```
Запрос: http://127.0.0.1:8080/search?q=Hello+World

В коде:

router.add_route("GET", "/search", |req, res| {
	let search = req.get_query_param("q"); // <- Получить значение для q из строки запроса
});
```

## HttpResponse

1. set_header — устанавливает заголовок в необработанном виде.
```
res.set_header("HTTP/1.1 400 Bad Request\r\n\r\n400 Bad Request");
```

Пример:

```
router.add_route("GET", "/page_not_found", |req, res| {
	res.set_header("HTTP/1.1 404 Not Found\r\n\r\n404 Requested page not found");
});
```

2. add_header — устанавливает заголовок в формате Key-Value. Пример:
```
res.add_header("Content-Type", "application/json; charset=utf-8");
```

Пример:

```
router.add_route("GET", "/test", |req, res| {
	res.add_header("Content-Type", "text/html; charset=utf-8");
});
```

3. set_status_code — устанавливает код статуса ответа. Принимает enum StatusCode в качестве параметра. Пример:
```
res.set_status_code(StatusCode::Forbidden); // для возврата ответа 403 Forbidden.
```

Пример:

```
router.add_route("GET", "/test", |req, res| {
	res.set_status_code(StatusCode::Forbidden);
	res.write("Viewing this page is prohibited.");
});
```

4. write — возвращает ответ клиенту. Принимает строку в качестве входных данных (включая формат &format!). Примеры:
```
res.write("Добро пожаловать в httpx!");
```

Пример:
```
router.add_route("GET", "/", |req, res| {
	res.write(&format!("Добро пожаловать в {}", "httpx"));
});
```

## StatusCode

Поддерживает всевозможные коды ответов типа StatusCode::НАЗВАНИЕ_СТАТУС_КОДА. Примеры:
StatusCode::Created
StatusCode::OK
StatusCode::Accepted
StatusCode::Unauthorized
StatusCode::Forbidden


## Полный список StatusCode

```
// 1xx
Continue,
SwitchingProtocols,
Processing,
EarlyHints,
```

```
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
```

```
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
```

```
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
```

```
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
```