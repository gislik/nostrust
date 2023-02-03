use serde::{Deserialize, Serialize};

/// Event is at the heart of nostr
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Event {
    id: String,
    pubkey: String,
    tags: Vec<String>,
    content: String,
    sig: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string, Result};

    fn get_event() -> Event {
        Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        }
    }

    #[test]
    fn serialize_works() -> Result<()> {
        let event = get_event();
        let got = to_string(&event)?;
        let want = r#"{"id":"id","pubkey":"pubkey","tags":[],"content":"content","sig":"sig"}"#;
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_works() -> Result<()> {
        let data = r#"{"id":"id","pubkey":"pubkey","tags":[],"content":"content","sig":"sig"}"#;
        let want = get_event();
        let got: Event = from_str(data)?;
        assert_eq!(got, want);

        Ok(())
    }
}
