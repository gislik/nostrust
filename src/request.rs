use crate::Hex;
use crate::{event::Kind, time::Seconds};
use serde::{Deserialize, Serialize};

/// Request is a notes filter. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Request {
    ids: Vec<Hex>,
    authors: Vec<Hex>,
    kinds: Vec<Kind>,
    #[serde(rename = "#e")]
    e: Vec<Hex>,
    #[serde(rename = "#p")]
    p: Vec<Hex>,
    since: Seconds,
    until: Seconds,
    limit: u16,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    pub fn get_simple_request() -> Request {
        Request {
            ids: vec!["id".to_string()],
            authors: vec!["author".to_string()],
            kinds: vec![1, 2],
            e: vec!["e".to_string(), "event".to_string()],
            p: vec!["p".to_string(), "profile".to_string()],
            since: 1,
            until: 2,
            limit: 3,
        }
    }

    pub fn get_json() -> &'static str {
        r##"{"ids":["id"],"authors":["author"],"kinds":[1,2],"#e":["e","event"],"#p":["p","profile"],"since":1,"until":2,"limit":3}"##
    }

    #[test]
    fn serialize_works() -> serde_json::Result<()> {
        let request = get_simple_request();
        let got = to_string(&request)?;
        let want = get_json();
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_works() -> serde_json::Result<()> {
        let data = get_json();
        let got: Request = from_str(data)?;
        let want = get_simple_request();
        assert_eq!(got, want);
        Ok(())
    }
}
