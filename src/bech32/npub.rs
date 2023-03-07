use std::result;

use crate::bech32::{self, FromBech32, ToBech32};
use crate::key::PublicKey;

const PUBLIC_PREFIX: &str = "npub";

impl ToBech32 for PublicKey {
    fn to_bech32(&self) -> String {
        bech32::encode(PUBLIC_PREFIX, self.0.serialize().into()).unwrap() // never results in an error
    }
}

impl FromBech32 for PublicKey {
    type Error = bech32::Error;

    fn from_bech32(s: &str) -> Result<Self> {
        let bytes = bech32::decode(PUBLIC_PREFIX, &s)?;
        let key = Self::try_from(bytes.as_slice())?;
        Ok(key)
    }
}

type Result<T> = result::Result<T, bech32::Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::tests::get_public_key;

    #[test]
    fn public_key_from_npub() -> Result<()> {
        let npub = "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6";
        let got = PublicKey::from_bech32(npub)?;
        let want = get_public_key();
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn public_key_to_npub() -> Result<()> {
        let got = get_public_key().to_bech32();
        let want = "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6";
        assert_eq!(got, want);
        Ok(())
    }
}
