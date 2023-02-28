use crate::event::Kind;
use crate::time::{self, Seconds};
use crate::Hex;
use serde::{Deserialize, Serialize};

/// Request is a notes filter. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Request {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ids: Vec<Hex>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    authors: Vec<Hex>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    kinds: Vec<Kind>,
    #[serde(rename = "#e", skip_serializing_if = "Vec::is_empty", default)]
    e: Vec<Hex>,
    #[serde(rename = "#p", skip_serializing_if = "Vec::is_empty", default)]
    p: Vec<Hex>,
    #[serde(skip_serializing_if = "is_zero", default)]
    since: Seconds,
    #[serde(skip_serializing_if = "is_zero", default)]
    until: Seconds,
    limit: u16,
}

impl Request {
    pub fn new() -> Self {
        let until = time::since_epoch();
        Self {
            ids: vec![],
            authors: vec![],
            kinds: vec![],
            e: vec![],
            p: vec![],
            since: 0,
            until,
            limit: 100,
        }
    }

    pub fn set_ids(&mut self, ids: Vec<Hex>) -> &mut Self {
        self.ids = ids;
        self
    }

    pub fn add_id(&mut self, id: Hex) -> &mut Self {
        self.ids.push(id);
        self
    }

    pub fn set_authors(&mut self, authors: Vec<Hex>) -> &mut Self {
        self.authors = authors;
        self
    }

    pub fn add_author(&mut self, author: Hex) -> &mut Self {
        self.authors.push(author);
        self
    }

    pub fn set_kinds(&mut self, kinds: Vec<Kind>) -> &mut Self {
        self.kinds = kinds;
        self
    }

    pub fn add_kind(&mut self, kind: Kind) -> &mut Self {
        self.kinds.push(kind);
        self
    }

    pub fn set_events(&mut self, events: Vec<Hex>) -> &mut Self {
        self.e = events;
        self
    }

    pub fn add_event(&mut self, event: Hex) -> &mut Self {
        self.e.push(event);
        self
    }

    pub fn set_profiles(&mut self, profiles: Vec<Hex>) -> &mut Self {
        self.p = profiles;
        self
    }

    pub fn add_profilfe(&mut self, profile: Hex) -> &mut Self {
        self.p.push(profile);
        self
    }

    pub fn set_since(&mut self, since: Seconds) -> &mut Self {
        self.since = since;
        self
    }

    pub fn set_until(&mut self, until: Seconds) -> &mut Self {
        self.until = until;
        self
    }

    pub fn set_limit(&mut self, limit: u16) -> &mut Self {
        self.limit = limit;
        self
    }
}

fn is_zero(n: &Seconds) -> bool {
    *n == 0
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn new_request_has_limit() {
        let got = Request::new().limit;
        let want = 100;
        assert_eq!(got, want)
    }

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

    fn get_empty_request() -> Request {
        Request {
            ids: vec![],
            authors: vec![],
            kinds: vec![],
            e: vec![],
            p: vec![],
            since: 0,
            until: 0,
            limit: 0,
        }
    }

    fn get_empty_json() -> &'static str {
        r##"{"limit":0}"##
    }

    #[test]
    fn serialize_empty_works() -> serde_json::Result<()> {
        let request = get_empty_request();
        let got = to_string(&request)?;
        let want = get_empty_json();
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_empty_works() -> serde_json::Result<()> {
        let data = get_empty_json();
        let got: Request = from_str(data)?;
        let want = get_empty_request();
        assert_eq!(got, want);
        Ok(())
    }
}
