type Type = http::Method;
const EXPECT_MESSAGE: &str = "valid method name";

serialize_str!(Type);
create_visitor!(Visitor, Type, EXPECT_MESSAGE, (visit_str, &str));
deserialize_str!(Visitor, Type);

derive_extension_types!(super::Type);
derive_hash_types!(super::Type);
