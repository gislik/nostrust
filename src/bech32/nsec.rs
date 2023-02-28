use super::FromBech32;
use crate::bech32;
use crate::key::SecretKey;

pub(crate) const SECRET_PREFIX: &str = "nsec";

impl FromBech32 for SecretKey {
    type Err = bech32::Error;

    fn from_bech32(nsec: &str) -> Result<Self> {
        let raw = bech32::decode(SECRET_PREFIX, nsec)?;
        let key = Self::try_from(raw.as_slice())?;
        Ok(key)
    }
}

type Result<T> = std::result::Result<T, bech32::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_key_from_nsec() -> Result<()> {
        let nsec = "nsec1pu2zjemwmu0l3ew2sgpvsaquk62lc08zfmp6ms8u7g6pzmcglpysymcg0m";
        let sk = SecretKey::from_bech32(nsec)?;
        let got = sk.display_secret();
        let want = "0f1429676edf1ff8e5ca8202c8741cb695fc3ce24ec3adc0fcf234116f08f849";
        assert_eq!(got, want);
        Ok(())
    }
}
