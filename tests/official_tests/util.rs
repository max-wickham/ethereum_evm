use ethereum_evm::runtime::Runtime;
use hex::FromHex;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;





// TODO reimplement;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Hex(#[serde(deserialize_with = "deserialize_hex_bytes")] pub Vec<u8>);

fn deserialize_hex_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct HexStrVisitor;

    impl<'de> Visitor<'de> for HexStrVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a hex encoded string")
        }

        fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if &data[0..2] != "0x" {
                return Err(Error::custom("should start with 0x"));
            }

            FromHex::from_hex(&data[2..]).map_err(Error::custom)
        }

        fn visit_borrowed_str<E>(self, data: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if &data[0..2] != "0x" {
                return Err(Error::custom("should start with 0x"));
            }

            FromHex::from_hex(&data[2..]).map_err(Error::custom)
        }
    }

    deserializer.deserialize_str(HexStrVisitor)
}
