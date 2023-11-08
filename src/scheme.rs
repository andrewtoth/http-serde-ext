type Type = http::uri::Scheme;
const EXPECT_MESSAGE: &str = "valid scheme";

serialize_str!(Type);
create_visitor!(Visitor, Type, EXPECT_MESSAGE, (visit_str, &str));
deserialize_str!(Visitor, Type);

derive_extension_types!(super::Type);
serde_seq!(
    std::collections::HashSet<super::Type>,
    super::Type,
    std::collections::HashSet::with_capacity,
    insert,
    hash_set
);
