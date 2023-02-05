use crate::{Epoch, Hex, Kind};
use serde::{Deserialize, Serialize};

/// Request is a notes filter
#[derive(Serialize, Deserialize)]
pub struct Request {
    ids: Vec<Hex>,
    authors: Vec<Hex>,
    kinds: Vec<Kind>,
    e: Vec<Hex>,
    p: Vec<Hex>,
    since: Vec<Epoch>,
    until: Vec<Epoch>,
    limit: Vec<u16>,
}
