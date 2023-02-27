use crate::event::Event;
use crate::request::Request;
use serde::ser::SerializeSeq;
use serde::Serialize;

/// Messages are sent from clients to relays. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
pub enum MessageRequest {
    Event(Event),
    Request(String, Request),
    Close(String),
}

impl Serialize for MessageRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageRequest::Event(event) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&"EVENT".to_string())?;
                seq.serialize_element(event)?;
                seq.end()
            }
            MessageRequest::Request(subscription_id, request) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&"REQ".to_string())?;
                seq.serialize_element(subscription_id)?;
                seq.serialize_element(request)?;
                seq.end()
            }
            MessageRequest::Close(subscription_id) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&"CLOSE".to_string())?;
                seq.serialize_element(subscription_id)?;
                seq.end()
            }
        }
    }
}

pub enum MessageResponse {
    Event(String, Event),
    Notice(String),
}

impl Serialize for MessageResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageResponse::Event(subscription_id, event) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(&"EVENT".to_string())?;
                seq.serialize_element(&subscription_id)?;
                seq.serialize_element(event)?;
                seq.end()
            }
            MessageResponse::Notice(message) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&"NOTICE".to_string())?;
                seq.serialize_element(message)?;
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
    fn serialize_event_request_works() -> serde_json::Result<()> {
        let event = event::tests::get_simple_event();
        let message = MessageRequest::Event(event);
        let got = to_string(&message)?;
        let json = event::tests::get_json();
        let want = format!(r#"["EVENT",{}]"#, json);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_request_request_works() -> serde_json::Result<()> {
        let id = "subid".to_string();
        let request = request::tests::get_simple_request();
        let message = MessageRequest::Request(id.clone(), request);
        let got = to_string(&message)?;
        let json = request::tests::get_json();
        let want = format!(r#"["REQ","{}",{}]"#, id, json);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_close_request_works() -> serde_json::Result<()> {
        let message = MessageRequest::Close("subid".to_string());
        let got = to_string(&message)?;
        let want = r#"["CLOSE","subid"]"#;
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_event_response_works() -> serde_json::Result<()> {
        let event = event::tests::get_simple_event();
        let subscription_id = "subid".to_string();
        let message = MessageResponse::Event(subscription_id, event);
        let got = to_string(&message)?;
        let json = event::tests::get_json();
        let want = format!(r#"["EVENT","subid",{}]"#, json);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn serialize_notice_response_works() -> serde_json::Result<()> {
        let notice = "notice".to_string();
        let message = MessageResponse::Notice(notice);
        let got = to_string(&message)?;
        let want = r#"["NOTICE","notice"]"#;
        assert_eq!(got, want);
        Ok(())
    }
}
