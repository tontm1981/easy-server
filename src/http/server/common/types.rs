use std::collections::HashMap;
use std::io::Error;
use super::Request;

pub type ApplicationMap = HashMap<String, RouteMap>;

pub type RouteMap = HashMap<String, ApplicationHandler>;

pub type ApplicationHandler = fn(&Request) -> Result<Option<String>, Error>;

pub type MiddlewareFunctionsVec = Vec<fn(&Request) -> Result<(), Error>>;