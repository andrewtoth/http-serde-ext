type Type = http::uri::Authority;
const EXPECT_MESSAGE: &str = "valid authority";

serialize_str!(Type);
create_visitor!(
    Visitor,
    Type,
    EXPECT_MESSAGE,
    (visit_str, &str),
    (visit_string, String)
);
deserialize_string!(Visitor, Type);

derive_extension_types!(super::Type);
derive_hash_types!(super::Type);
