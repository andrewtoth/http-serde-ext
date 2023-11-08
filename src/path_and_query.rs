type Type = http::uri::PathAndQuery;
const EXPECT_MESSAGE: &str = "valid path and query";

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
serde_seq!(
    std::collections::HashSet<super::Type>,
    super::Type,
    std::collections::HashSet::with_capacity,
    insert,
    hash_set
);
