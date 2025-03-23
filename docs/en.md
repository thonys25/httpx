# Documentation

## HttpRequest

1. get_body_raw - Method to get raw data from the body that was passed, for example, via the header
```
application/json
```

2. get_body - Method to get data from the body. Intended exclusively for getting FormData data. If the form was submitted as:
```
application/x-www-form-urlencoded
```

3. get_method - Get the name of the method that made the request

4. get_param - Method to get parameterized, dynamic values ​​from the route. Example:
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

In the code:
router.add_route("GET", "/search", |req, res| {
let search = req.get_query_param("q"); // <- Get the value for q from the query string
});
```

## HttpResponse

1. set_header — sets the header in raw form. Example:
```
res.set_header("HTTP/1.1 400 Bad Request\r\n\r\n400 Bad Request");
```

2. add_header — sets the header in Key-Value format. Example:
```
res.add_header("Content-Type", "application/json; charset=utf-8");
```

3. set_status_code — sets the response status code. Accepts enum StatusCode as a parameter. Example:
```
res.set_status_code(StatusCode::Forbidden); // to return 403 Forbidden response.
```

4. write — returns the response to the client. Accepts a string as input (including &format! format). Examples:
```
res.write("Welcome to httpx!");
```
```
res.write(&format!("Welcome to {}", "httpx"));
```

## StatusCode

Supports all kinds of response codes of the type StatusCode::STATUS_CODE_NAME. Examples:
StatusCode::Created
StatusCode::OK
StatusCode::Accepted
StatusCode::NoContent
StatusCode::Unauthorized
StatusCode::Forbidden

and so on..

## Full list of StatusCode

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