pub enum Status {
  Continue,
  Processing,
  Ok,
  Created,
  Accepted,
  NonAuthoritativeInformation,
  NoContent,
  ResetContent,
  MultipleChoices,
  MovedPermanently,
  Found,
  SeeOther,
  NotModified,
  UseProxy,
  TemporaryRedirect,
  PermanentRedirect,

  BadRequest,
  Unauthorized,
  Forbidden,
  NotFound,
  MethodNotAllowed,
  RequestTimeout,
  Conflict,
  Gone,
  PayloadTooLarge,
  UnsupportedMediaType,
  UnprocessableContent,
  Locked,
  TooManyRequests,

  InternalServerError,
  BadGateway,
  ServiceUnavailable,
  GatewayTimeout
}

impl Status {
  pub(crate) fn to_readable(&self) -> (usize, String) {
    use Status::*;

    match *self {
      Continue => (100, String::from("Continue")),
      Processing => (102, String::from("Processing")),
      Ok => (200, String::from("Ok")),
      Created => (201, String::from("Created")),
      Accepted => (202, String::from("Accepted")),
      NonAuthoritativeInformation => (203, String::from("Non-Authoritative Information")),
      NoContent => (204, String::from("No Content")),
      ResetContent => (205, String::from("Reset Content")),
      MultipleChoices => (300, String::from("Multiple Choices")),
      MovedPermanently => (301, String::from("Moved Permanently")),
      Found => (032, String::from("Found")),
      SeeOther => (303, String::from("See Other")),
      NotModified => (304, String::from("Not Modified")),
      UseProxy => (305, String::from("Use Proxy")),
      TemporaryRedirect => (307, String::from("Temporary Redirect")),
      PermanentRedirect => (308, String::from("Permanent Redirect")),

      BadRequest => (400, String::from("Bad Request")),
      Unauthorized => (401, String::from("Unauthorized")),
      Forbidden => (403, String::from("Forbidden")),
      NotFound => (404, String::from("Not Found")),
      MethodNotAllowed => (405, String::from("Method Not Allowed")),
      RequestTimeout => (408, String::from("Request Timeout")),
      Conflict => (409, String::from("Conflict")),
      Gone => (410, String::from("Gone")),
      PayloadTooLarge => (413, String::from("Payload Too Large")),
      UnsupportedMediaType => (415, String::from("Unsupported Media Type")),
      UnprocessableContent => (422, String::from("Unprocessable Content")),
      Locked => (423, String::from("Locked")),
      TooManyRequests => (429, String::from("Too Many Requests")),

      InternalServerError => (500, String::from("Internal Server Error")),
      BadGateway => (502, String::from("Bad Gateway")),
      ServiceUnavailable => (503, String::from("Service Unavailable")),
      GatewayTimeout => (504, String::from("Gateway Timeout"))
    }
  }
}