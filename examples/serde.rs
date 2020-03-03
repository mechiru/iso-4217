use iso_4217::CurrencyCode;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Object {
    #[serde(with = "str_curr_code")]
    str_code: CurrencyCode,

    #[serde(with = "num_curr_code")]
    num_code: CurrencyCode,
}

mod str_curr_code {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::convert::TryFrom;

    pub fn serialize<S>(v: &CurrencyCode, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str((*v).into())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<CurrencyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        CurrencyCode::try_from(s.as_ref()).map_err(serde::de::Error::custom)
    }
}

mod num_curr_code {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::convert::TryFrom;

    pub fn serialize<S>(v: &CurrencyCode, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u32((*v).into())
    }

    pub fn deserialize<'de, D>(d: D) -> Result<CurrencyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = u32::deserialize(d)?;
        CurrencyCode::try_from(v).map_err(serde::de::Error::custom)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json: &[u8] = br#"{"str_code":"USD","num_code":392}"#;
    let obj = Object {
        str_code: CurrencyCode::USD,
        num_code: CurrencyCode::JPY,
    };

    assert_eq!(serde_json::from_slice::<Object>(json)?, obj);
    assert_eq!(serde_json::to_vec(&obj)?, json);

    Ok(())
}
