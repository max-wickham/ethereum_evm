use hex::FromHex;
use serde::{
    de::{Error, Visitor}, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer
};
use primitive_types::{H256, U256};
use std::fmt;


pub fn u256_to_h256(v: U256) -> H256 {
    let mut r = H256::default();
    v.to_big_endian(&mut r[..]);
    r
}


// TODO reimplement;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Hex(#[serde(deserialize_with = "deserialize_hex_bytes", serialize_with = "serialize_hex_bytes")] pub Vec<u8>);

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

fn serialize_hex_bytes<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string: String = data.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    let hex_string = "0x".to_string() + &hex_string;

    serializer.serialize_str(&hex_string)
}
