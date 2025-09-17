//! # Status Module
//!
//! Provides HTTP status codes and utilities for working with them.
//!
//! The [`HttpStatus`] enum represents standard HTTP status codes (1xx–5xx) with
//! associated methods to get numeric codes and reason phrases.

/// Standard HTTP status codes.
///
/// # Example
/// ```
/// use rxpress::HttpStatus;
///
/// // Get numeric code
/// assert_eq!(HttpStatus::OK.code(), 200);
///
/// // Get reason phrase
/// assert_eq!(HttpStatus::reason(404), "Not Found");
/// ```

/// Represents Standard HTTP status Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpStatus {
    // 1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // 2xx Success
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    // 3xx Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx Client Errors
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // 5xx Server Errors
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HttpStatus {
    //// Returns the reason phrase for a status code.
    ///
    /// # Example
    /// ```
    /// use rxpress::HttpStatus;
    /// assert_eq!(HttpStatus::reason(200), "OK");
    /// assert_eq!(HttpStatus::reason(404), "Not Found");
    /// assert_eq!(HttpStatus::reason(418), "I'm a Teapot");
    /// ```
    pub fn reason(code: u16) -> &'static str {
        match code {
            // 1xx
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",

            // 2xx
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",

            // 3xx
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",

            // 4xx
            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a Teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            425 => "Too Early",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",

            // 5xx
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => "",
        }
    }

    /// Returns the numeric code for an enum variant.
    ///
    /// # Example
    /// ```
    /// use rxpress::HttpStatus;
    /// assert_eq!(HttpStatus::OK.code(), 200);
    /// assert_eq!(HttpStatus::NotFound.code(), 404);
    /// ```
    pub fn code(&self) -> u16 {
        *self as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST - http status codes
    #[test]
    fn test_http_status_codes() {
        assert_eq!(HttpStatus::OK.code(), 200);
        assert_eq!(HttpStatus::NotFound.code(), 404);
        assert_eq!(HttpStatus::InternalServerError.code(), 500);
    }

    // TEST - http status code reasons
    #[test]
    fn test_http_status_reason_lookup() {
        assert_eq!(HttpStatus::reason(200), "OK");
        assert_eq!(HttpStatus::reason(404), "Not Found");
        assert_eq!(HttpStatus::reason(418), "I'm a Teapot");
        assert_ne!(HttpStatus::reason(418), "I'm a teapot");
    }
}
