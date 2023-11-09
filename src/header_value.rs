use serde::{ser, Deserializer, Serializer};

type Type = http::HeaderValue;
const EXPECT_MESSAGE: &str = "a header value";

pub fn serialize<S: Serializer>(val: &Type, ser: S) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
        use ser::Error;
        ser.serialize_str(val.to_str().map_err(Error::custom)?)
    } else {
        ser.serialize_bytes(val.as_ref())
    }
}

create_visitor!(
    Visitor,
    Type,
    EXPECT_MESSAGE,
    (visit_str, &str),
    (visit_string, String),
    (visit_bytes, &[u8]),
    (visit_byte_buf, Vec<u8>)
);

pub fn deserialize<'de, D>(de: D) -> Result<Type, D::Error>
where
    D: Deserializer<'de>,
{
    if de.is_human_readable() {
        de.deserialize_string(Visitor)
    } else {
        de.deserialize_byte_buf(Visitor)
    }
}

derive_extension_types!(super::Type);
derive_hash_types!(super::Type);
derive_ord_types!(super::Type);
