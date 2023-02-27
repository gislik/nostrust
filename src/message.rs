use crate::event::Event;
use crate::request::Request;
use serde::ser::SerializeSeq;
use serde::Serialize;

/// Messages are sent from clients to relays. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
pub enum Message {
    Event(Event),
    Request(String, Request),
    Close(String),
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Message::Event(event) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&"EVENT".to_string())?;
                seq.serialize_element(event)?;
                seq.end()
            }
            Message::Request(subscription_id, request) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&"REQ".to_string())?;
                seq.serialize_element(subscription_id)?;
                seq.serialize_element(request)?;
                seq.end()
            }
            Message::Close(subscription_id) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&"CLOSE".to_string())?;
                seq.serialize_element(subscription_id)?;
                seq.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event;
    use crate::request;
    use serde_json::to_string;

    #[test]
    fn serialize_event_works() -> serde_json::Result<()> {
        let event = event::tests::get_simple_event();
        let message = Message::Event(event);
        let got = to_string(&message)?;
        let json = event::tests::get_json();
        let want = format!(r#"["EVENT",{}]"#, json);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_request_works() -> serde_json::Result<()> {
        let id = "subid".to_string();
        let request = request::tests::get_simple_request();
        let message = Message::Request(id.clone(), request);
        let got = to_string(&message)?;
        let json = request::tests::get_json();
        let want = format!(r#"["REQ","{}",{}]"#, id, json);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_close_works() -> serde_json::Result<()> {
        let message = Message::Close("subid".to_string());
        let got = to_string(&message)?;
        let want = r#"["CLOSE","subid"]"#;
        assert_eq!(got, want);
        Ok(())
    }
}
