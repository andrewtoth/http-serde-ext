type Type = http::uri::Authority;
const EXPECT_MESSAGE: &str = "valid authority";

serialize_as_str!(Type);
create_visitor!(
    Visitor,
    Type,
    EXPECT_MESSAGE,
    (visit_str, &str),
    (visit_string, String)
);
deserialize_string!(Type, Visitor);

derive_extension_types!(super::Type);
