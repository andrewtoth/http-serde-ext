type Type = http::HeaderName;
const EXPECT_MESSAGE: &str = "a header name";

serialize_as_str!(Type);
create_visitor!(Visitor, Type, EXPECT_MESSAGE, (visit_str, &str));
deserialize_from_str!(Visitor, Type);

derive_extension_types!(super::Type);
