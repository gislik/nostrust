use crate::time::Seconds;
use crate::{Hex, Kind};
use serde::{Deserialize, Serialize};

/// Request is a notes filter
#[derive(Serialize, Deserialize)]
pub struct Request {
    ids: Vec<Hex>,
    authors: Vec<Hex>,
    kinds: Vec<Kind>,
    e: Vec<Hex>,
    p: Vec<Hex>,
    since: Vec<Seconds>,
    until: Vec<Seconds>,
    limit: Vec<u16>,
}
