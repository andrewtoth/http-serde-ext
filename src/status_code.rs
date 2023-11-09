use std::fmt;

use serde::{de, Deserializer, Serializer};

type Type = http::StatusCode;
const EXPECT_MESSAGE: &str = "a status code";

pub fn serialize<S: Serializer>(status: &Type, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u16(status.as_u16())
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECT_MESSAGE)
    }

    fn visit_i32<E: de::Error>(self, val: i32) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_i16<E: de::Error>(self, val: i16) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_u8<E: de::Error>(self, val: u8) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_u32<E: de::Error>(self, val: u32) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_i64<E: de::Error>(self, val: i64) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_u64<E: de::Error>(self, val: u64) -> Result<Self::Value, E> {
        self.visit_u16(val as u16)
    }

    fn visit_u16<E: de::Error>(self, val: u16) -> Result<Self::Value, E> {
        val.try_into().map_err(de::Error::custom)
    }
}

pub fn deserialize<'de, D>(de: D) -> Result<Type, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_u16(Visitor)
}

derive_extension_types!(super::Type);
derive_hash_types!(super::Type);
derive_ord_types!(super::Type);
