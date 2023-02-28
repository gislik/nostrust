use crate::event::Event;
use crate::request::Request;
use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

/// Messages are sent from clients to relays. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
#[derive(Debug, PartialEq)]
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

struct MessageRequestVisitor;

impl<'de> Visitor<'de> for MessageRequestVisitor {
    type Value = MessageRequest;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("message request array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        if let Some(topic) = seq.next_element()? {
            match topic {
                "EVENT" => {
                    let event = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(1, &self))?;
                    Ok(MessageRequest::Event(event))
                }
                "REQ" => {
                    let sequence_id = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(1, &self))?;
                    let request = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(2, &self))?;
                    Ok(MessageRequest::Request(sequence_id, request))
                }
                "CLOSE" => {
                    let sequence_id = seq
                        .next_element()?
                        .ok_or(serde::de::Error::invalid_length(1, &self))?;
                    Ok(MessageRequest::Close(sequence_id))
                }
                other => Err(serde::de::Error::unknown_variant(
                    other,
                    &["EVENT", "REQ", "CLOSE"],
                )),
            }
        } else {
            Err(serde::de::Error::invalid_length(0, &self))
        }
    }
}

impl<'de> Deserialize<'de> for MessageRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(MessageRequestVisitor)
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
    use serde_json::{from_str, to_string};

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
    fn deserialize_event_request_works() -> serde_json::Result<()> {
        let data = format!(r#"["EVENT",{}]"#, event::tests::get_json());
        let got: MessageRequest = from_str(&data)?;
        let event = event::tests::get_simple_event();
        let want = MessageRequest::Event(event);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_request_request_works() -> serde_json::Result<()> {
        let data = format!(r#"["REQ","subid",{}]"#, request::tests::get_json());
        let got: MessageRequest = from_str(&data)?;
        let request = request::tests::get_simple_request();
        let want = MessageRequest::Request("subid".to_string(), request);
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_close_request_works() -> serde_json::Result<()> {
        let data = r#"["CLOSE","subid"]"#;
        let got: MessageRequest = from_str(&data)?;
        let want = MessageRequest::Close("subid".to_string());
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
