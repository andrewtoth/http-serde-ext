use std::fmt;

use serde::{de, Serializer};

type Type = http::Version;
const EXPECT_MESSAGE: &str = "a version string";

pub fn serialize<S: Serializer>(val: &Type, ser: S) -> Result<S::Ok, S::Error> {
    let val = match *val {
        Type::HTTP_09 => "HTTP/0.9",
        Type::HTTP_10 => "HTTP/1.0",
        Type::HTTP_11 => "HTTP/1.1",
        Type::HTTP_2 => "HTTP/2.0",
        Type::HTTP_3 => "HTTP/3.0",
        _ => {
            return ser.serialize_str(format!("{val:?}").as_str());
        }
    };
    ser.serialize_str(val)
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECT_MESSAGE)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let version = match v {
            "HTTP/0.9" => Type::HTTP_09,
            "HTTP/1.0" => Type::HTTP_10,
            "HTTP/1.1" => Type::HTTP_11,
            "HTTP/2.0" => Type::HTTP_2,
            "HTTP/3.0" => Type::HTTP_3,
            _ => return Err(E::invalid_value(de::Unexpected::Str(v), &self)),
        };
        Ok(version)
    }
}

deserialize_str!(Visitor, Type);

derive_extension_types!(super::Type);
