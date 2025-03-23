# Documentation

## HttpRequest

1. get_body_raw - Method for getting raw data from the body, passed, for example, via the header
```
application/json
```

Example:
```
router.add_route("POST", "/article/create", |req, res| {
	let body = req.get_body_raw();
});
```

2. get_body - Method for getting data from the body. Intended exclusively for getting FormData data. If the form was submitted as:
```
application/x-www-form-urlencoded
```

Example:
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

3. get_method — get the name of the method that made the request. If the request to the server was sent by the GET method, it will return GET, if by the POST method, then POST, etc.

Example:
```
router.add_route("GET", "/", |req, res| {
	let method = req.get_method();
});
```

4. get_param - Method for getting parameterized dynamic values ​​from the route. Example:
```
Request: http://127.0.0.1:8080/articles/1

In the code:

router.add_route("GET", "/articles/:id", |req, res| {
	let id = req.get_param("id"); // <- Get the identifier, the value of which can be dynamic
});
```

5. get_query_param - Method for getting a parameter from the query string. Example:
```
Request: http://127.0.0.1:8080/search?q=Hello+World

In code:

router.add_route("GET", "/search", |req, res| {
	let search = req.get_query_param("q"); // <- Get value for q from query string
});
```

## HttpResponse

1. set_header — sets header in raw form.
```
res.set_header("HTTP/1.1 400 Bad Request\r\n\r\n400 Bad Request");
```

Example:

```
router.add_route("GET", "/page_not_found", |req, res| {
	res.set_header("HTTP/1.1 404 Not Found\r\n\r\n404 The requested page was not found");
});
```

2. add_header — sets the header in Key-Value format. Example:
```
res.add_header("Content-Type", "application/json; charset=utf-8");
```

Example:

```
router.add_route("GET", "/test", |req, res| {
	res.add_header("Content-Type", "text/html; charset=utf-8");
});
```

3. set_status_code — sets the response status code. Takes enum StatusCode as parameter. Example:
```
res.set_status_code(StatusCode::Forbidden); // to return 403 Forbidden response.
```

Example:

```
router.add_route("GET", "/test", |req, res| {
	res.set_status_code(StatusCode::Forbidden);
	res.write("Viewing this page is prohibited.");
});
```

4. write — returns response to client. Takes string as input (including &format! format). Examples:
```
res.write("Welcome to httpx!");
```

Example:
```
router.add_route("GET", "/", |req, res| {
	res.write(&format!("Welcome to {}", "httpx"));
});
```

##StatusCode

Supports all types of response codes of the StatusCode::STATUS_CODE_NAME type. Examples:
StatusCode::Created
StatusCode::OK
StatusCode::Accepted
StatusCode::Unauthorized
StatusCode::Forbidden

## Full list of StatusCodes

```
// 1xx
Continue
SwitchingProtocols
Processing
early hints,
```

```
// 2xx
OK,
Created
Accepted
NonAuthoritativeInformation,
NoContent
Reset contents
Partial contents
MultiStatus
Already reported,
IMUsed
```

```
// 3xx
MultiSelect
MovedPermanently
Found
Moved Temporarily
SeeOther,
NotModified
UseProxy
SwitchProxy
TemporaryRedirect
PermanentRedirect
```

```
// 4xx
BadRequest
Unauthorized
PaymentRequired
Forbidden,
NotFound,
MethodNotAllowed
NotAcceptable
ProxyAuthenticationRequired
RequestTimeout
conflict,
Gone,
LengthRequired
PreconditionFailed
PayloadTooLarge,
URITooLong,
UnsupportedMediaType
RangeNotSatisfiable
Expecta tionFailed
ImTeapot,
AuthenticationTimeout
MisdirectedRequest
UnprocessableEntity
Locked
FailedDependency
TooEarly
UpgradeRequired
PreconditionRequired
TooManyRequests
RequestHeaderFieldsTooLarge,
RetryWith,
UnavailableForLegalReasons,
ClientClosedRequest
```

```
// 5xx
InternalServerError
NotImplemented
BadGateway
ServiceUnavailable
GatewayTimeout
HTTPVersionNotSupported,
VariantAlsoNegotiates,
InsufficientStorage
LoopDetected
BandwidthLimitExceeded,
NotExtended
NetworkAuthenticationRequired
UnknownError
WebServerIsDown
ConnectionTimedOut
OriginIsUnavailable
TimeoutOccurred
SSLHandshakeFailed,
InvalidSSLCertificate,
```