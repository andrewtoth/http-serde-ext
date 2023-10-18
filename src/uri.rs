use serde::Serializer;

type Type = http::Uri;
const EXPECT_MESSAGE: &str = "a uri string";

pub fn serialize<S: Serializer>(val: &Type, ser: S) -> Result<S::Ok, S::Error> {
    ser.collect_str(val)
}

create_visitor!(
    Visitor,
    Type,
    EXPECT_MESSAGE,
    (visit_str, &str),
    (visit_string, String)
);
deserialize_string!(Type, Visitor);

derive_extension_types!(super::Type);
