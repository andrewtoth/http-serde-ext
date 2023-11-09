#![allow(clippy::redundant_closure_call, clippy::mutable_key_type)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Debug,
    io,
    str::FromStr,
};

use fake::{Fake, Faker};
use http::{
    uri::{Authority, PathAndQuery, Scheme},
    HeaderMap, HeaderName, HeaderValue, Method, Request, Response, StatusCode, Uri, Version,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

macro_rules! serde_json_roundtrip {
    ($ty:ty, $val:expr, $path:expr, $json:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val.clone());
        let ser = serde_json::to_string(&wrapper).expect("serialize json to string");
        assert_eq!(ser, $json.to_string());

        let de: Wrapper = serde_json::from_str(&ser).expect("deserialize json string");
        assert_eq!(de.0, $val);

        let de: Wrapper = serde_json::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize json string from reader");
        assert_eq!(de.0, $val);

        let ser = serde_json::to_value(wrapper).expect("serialize json to value");
        assert_eq!(ser, $json);

        let de: Wrapper = serde_json::from_value(ser).expect("deserialize json value");
        assert_eq!(de.0, $val);
    }};
}

macro_rules! serde_json_no_intermediate_compare_roundtrip {
    ($ty:ty, $val:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val.clone());
        let ser = serde_json::to_string(&wrapper).expect("serialize json to string");

        let de: Wrapper = serde_json::from_str(&ser).expect("deserialize json string");
        assert_eq!(de.0, $val);

        let de: Wrapper = serde_json::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize json string from reader");
        assert_eq!(de.0, $val);

        let ser = serde_json::to_value(wrapper).expect("serialize json to value");
        let de: Wrapper = serde_json::from_value(ser).expect("deserialize json value");
        assert_eq!(de.0, $val);
    }};
}

macro_rules! serde_yaml_roundtrip {
    ($ty:ty, $val:expr, $equate:expr, $path:expr, $yaml:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = serde_yaml::to_string(&wrapper).expect("serialize yaml");
        let mut compare = ser.clone();
        compare.retain(|c| !c.is_whitespace());
        let mut yaml = $yaml.to_string();
        yaml.retain(|c| !c.is_whitespace());
        assert_eq!(compare, yaml);

        let de: Wrapper = serde_yaml::from_str(&ser).expect("deserialize yaml");
        $equate(&de.0, &$val);

        let de: Wrapper = serde_yaml::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize yaml from reader");
        $equate(&de.0, &$val);
    }};
}

macro_rules! serde_yaml_no_intermediate_compare_roundtrip {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = serde_yaml::to_string(&wrapper).expect("serialize yaml");

        let de: Wrapper = serde_yaml::from_str(&ser).expect("deserialize yaml");
        $equate(&de.0, &$val);

        let de: Wrapper = serde_yaml::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize yaml from reader");
        $equate(&de.0, &$val);
    }};
}

macro_rules! serde_cbor_roundtrip {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = serde_cbor::to_vec(&wrapper).expect("serialize cbor");
        let de: Wrapper = serde_cbor::from_slice(&ser).expect("deserialize cbor");
        $equate(&de.0, &$val);
    }};
}

macro_rules! bincode_roundtrip {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = bincode::serialize(&wrapper).expect("serialize cbor");
        let de: Wrapper = bincode::deserialize(&ser).expect("deserialize cbor");
        $equate(&de.0, &$val);
    }};
}

macro_rules! postcard_roundtrip {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = postcard::to_allocvec(&wrapper).expect("serialize postcard");
        let de: Wrapper = postcard::from_bytes(&ser).expect("deserialize postcard");
        $equate(&de.0, &$val);
    }};
}

macro_rules! roundtrip {
    ($ty:ty, $val:expr, $path:expr, $json:expr, $yaml:expr) => {{
        serde_json_roundtrip!($ty, $val, $path, $json);

        fn equate(a: &$ty, b: &$ty) {
            assert_eq!(a, b);
        }

        serde_yaml_roundtrip!($ty, $val, equate, $path, $yaml);
        serde_cbor_roundtrip!($ty, $val, equate, $path);
        bincode_roundtrip!($ty, $val, equate, $path);
        postcard_roundtrip!($ty, $val, equate, $path);
    }};
}

macro_rules! no_intermediate_compare_roundtrip {
    ($ty:ty, $val:expr, $path:expr) => {{
        serde_json_no_intermediate_compare_roundtrip!($ty, $val, $path);

        fn equate(a: &$ty, b: &$ty) {
            assert_eq!(a, b);
        }

        serde_yaml_no_intermediate_compare_roundtrip!($ty, $val, equate, $path);
        serde_cbor_roundtrip!($ty, $val, equate, $path);
        bincode_roundtrip!($ty, $val, equate, $path);
        postcard_roundtrip!($ty, $val, equate, $path);
    }};
}

macro_rules! test_all {
    ($ty:ty, $val:expr, $json:expr, $yaml:expr, $path:literal, $option:literal, $result:literal, $vec:literal, $vec_deque:literal, $linked_list:literal, $hash_map:literal, $btree_map:literal) => {{
        roundtrip!($ty, $val, $path, json!($json), format!("{}\n", $yaml));

        roundtrip!(
            Option<$ty>,
            Some($val),
            $option,
            json!($json),
            format!("{}\n", $yaml)
        );
        roundtrip!(Option<$ty>, None, $option, json!(null), "null\n");
        roundtrip!(
            Result<$ty, String>,
            Ok($val),
            $result,
            json!({"Ok": $json}),
            format!("!Ok{}\n", $yaml)
        );
        roundtrip!(Result<$ty, String>, Err(String::default()), $result, json!({"Err": ""}), "!Err''\n");
        roundtrip!(
            Vec<$ty>,
            vec![$val],
            $vec,
            json!([$json]),
            format!("- {}\n", $yaml)
        );
        roundtrip!(
            VecDeque<$ty>,
            VecDeque::from([$val]),
            $vec_deque,
            json!([$json]),
            format!("- {}\n", $yaml)
        );
        roundtrip!(
            LinkedList<$ty>,
            LinkedList::from([$val]),
            $linked_list,
            json!([$json]),
            format!("- {}\n", $yaml)
        );
        roundtrip!(
            HashMap<String, $ty>,
            HashMap::from([("foo".to_string(), $val)]),
            $hash_map,
            json!({"foo": $json}),
            format!("foo: {}\n", $yaml)
        );
        roundtrip!(
            BTreeMap<String, $ty>,
            BTreeMap::from([("foo".to_string(), $val)]),
            $btree_map,
            json!({"foo": $json}),
            format!("foo: {}\n", $yaml)
        );
    }};
}

macro_rules! test_all_no_intermediate_compare {
    ($ty:ty, $val:expr, $path:literal, $option:literal, $result:literal, $vec:literal, $vec_deque:literal, $linked_list:literal, $hash_map:literal, $btree_map:literal) => {{
        no_intermediate_compare_roundtrip!($ty, $val, $path);

        no_intermediate_compare_roundtrip!(
            Option<$ty>,
            Some($val),
            $option
        );
        no_intermediate_compare_roundtrip!(Option<$ty>, None, $option);
        no_intermediate_compare_roundtrip!(
            Result<$ty, String>,
            Ok($val),
            $result
        );
        no_intermediate_compare_roundtrip!(Result<$ty, String>, Err(String::default()), $result);
        no_intermediate_compare_roundtrip!(
            Vec<$ty>,
            vec![$val],
            $vec
        );
        no_intermediate_compare_roundtrip!(
            VecDeque<$ty>,
            VecDeque::from([$val]),
            $vec_deque
        );
        no_intermediate_compare_roundtrip!(
            LinkedList<$ty>,
            LinkedList::from([$val]),
            $linked_list
        );
        no_intermediate_compare_roundtrip!(
            HashMap<String, $ty>,
            HashMap::from([("foo".to_string(), $val)]),
            $hash_map
        );
        no_intermediate_compare_roundtrip!(
            BTreeMap<String, $ty>,
            BTreeMap::from([("foo".to_string(), $val)]),
            $btree_map
        );
    }};
}

macro_rules! test_hash {
    ($ty:ty, $val:expr, $json:expr, $yaml:expr, $hash_map_key:literal, $hash_set:literal) => {{
        roundtrip!(
            HashMap<$ty, String>,
            HashMap::from([($val, String::default())]),
            $hash_map_key,
            json!({$json.as_str().unwrap(): ""}),
            format!("{}: ''\n", $yaml));
        roundtrip!(
            HashSet<$ty>,
            HashSet::from([$val]),
            $hash_set,
            json!([$json]),
            format!("- {}\n", $yaml)
        );
    }};
}

macro_rules! test_hash_no_intermediate_compare {
    ($ty:ty, $val:expr, $hash_map_key:literal, $hash_set:literal) => {{
        no_intermediate_compare_roundtrip!(
            HashMap<$ty, String>,
            HashMap::from([($val, String::default())]),
            $hash_map_key
        );
        no_intermediate_compare_roundtrip!(
            HashSet<$ty>,
            HashSet::from([$val]),
            $hash_set
        );
    }};
}

macro_rules! test_ord {
    ($ty:ty, $val:expr, $json:expr, $yaml:expr, $btree_map_key:literal, $btree_set:literal) => {{
        roundtrip!(
            BTreeMap<$ty, String>,
            BTreeMap::from([($val, String::default())]),
            $btree_map_key,
            json!({$json.as_str().unwrap(): ""}),
            format!("{}: ''\n", $yaml)
        );
        roundtrip!(
            BTreeSet<$ty>,
            BTreeSet::from([$val]),
            $btree_set,
            json!([$json]),
            format!("- {}\n", $yaml)
        );
    }};
}

macro_rules! test_ord_no_intermediate_compare {
    ($ty:ty, $val:expr, $btree_map_key:literal, $btree_set:literal) => {{
        no_intermediate_compare_roundtrip!(
            BTreeMap<$ty, String>,
            BTreeMap::from([($val, String::default())]),
            $btree_map_key
        );
        no_intermediate_compare_roundtrip!(
            BTreeSet<$ty>,
            BTreeSet::from([$val]),
            $btree_set
        );
    }};
}

#[test]
fn test_flattened_option() {
    #[derive(Deserialize, Serialize)]
    struct MyStruct {
        #[serde(flatten)]
        field: MyEnum,
    }

    #[derive(Deserialize, Serialize)]
    struct A {
        #[serde(with = "http_serde_ext::header_map::option")]
        field: Option<HeaderMap>,
    }

    #[derive(Deserialize, Serialize)]
    struct B;

    #[derive(Deserialize, Serialize)]
    #[serde(untagged)]
    enum MyEnum {
        A(A),
        B(B),
    }

    let json = json!({
        "field": null
    });

    let de: MyStruct = serde_json::from_value(json).unwrap();
    assert!(matches!(de.field, MyEnum::A(A { field: None })));
}

#[test]
fn test_authority_roundtrip() {
    test_all!(
        Authority,
        Authority::from_static("example.com:8080"),
        json!("example.com:8080"),
        "example.com:8080",
        "http_serde_ext::authority",
        "http_serde_ext::authority::option",
        "http_serde_ext::authority::result",
        "http_serde_ext::authority::vec",
        "http_serde_ext::authority::vec_deque",
        "http_serde_ext::authority::linked_list",
        "http_serde_ext::authority::hash_map",
        "http_serde_ext::authority::btree_map"
    );

    test_hash!(
        Authority,
        Authority::from_static("example.com:8080"),
        json!("example.com:8080"),
        "example.com:8080",
        "http_serde_ext::authority::hash_map_key",
        "http_serde_ext::authority::hash_set"
    );

    let fake: Authority = Faker.fake();
    test_all_no_intermediate_compare!(
        Authority,
        fake.clone(),
        "http_serde_ext::authority",
        "http_serde_ext::authority::option",
        "http_serde_ext::authority::result",
        "http_serde_ext::authority::vec",
        "http_serde_ext::authority::vec_deque",
        "http_serde_ext::authority::linked_list",
        "http_serde_ext::authority::hash_map",
        "http_serde_ext::authority::btree_map"
    );

    test_hash_no_intermediate_compare!(
        Authority,
        fake.clone(),
        "http_serde_ext::authority::hash_map_key",
        "http_serde_ext::authority::hash_set"
    );
}

#[test]
fn test_scheme_roundtrip() {
    test_all!(
        Scheme,
        Scheme::from_str("https").unwrap(),
        json!("https"),
        "https",
        "http_serde_ext::scheme",
        "http_serde_ext::scheme::option",
        "http_serde_ext::scheme::result",
        "http_serde_ext::scheme::vec",
        "http_serde_ext::scheme::vec_deque",
        "http_serde_ext::scheme::linked_list",
        "http_serde_ext::scheme::hash_map",
        "http_serde_ext::scheme::btree_map"
    );

    test_hash!(
        Scheme,
        Scheme::from_str("https").unwrap(),
        json!("https"),
        "https",
        "http_serde_ext::scheme::hash_map_key",
        "http_serde_ext::scheme::hash_set"
    );

    let fake: Scheme = Faker.fake();
    test_all_no_intermediate_compare!(
        Scheme,
        fake.clone(),
        "http_serde_ext::scheme",
        "http_serde_ext::scheme::option",
        "http_serde_ext::scheme::result",
        "http_serde_ext::scheme::vec",
        "http_serde_ext::scheme::vec_deque",
        "http_serde_ext::scheme::linked_list",
        "http_serde_ext::scheme::hash_map",
        "http_serde_ext::scheme::btree_map"
    );

    test_hash_no_intermediate_compare!(
        Scheme,
        fake.clone(),
        "http_serde_ext::scheme::hash_map_key",
        "http_serde_ext::scheme::hash_set"
    );
}

#[test]
fn test_path_and_query_roundtrip() {
    test_all!(
        PathAndQuery,
        PathAndQuery::from_static("/"),
        json!("/"),
        "/",
        "http_serde_ext::path_and_query",
        "http_serde_ext::path_and_query::option",
        "http_serde_ext::path_and_query::result",
        "http_serde_ext::path_and_query::vec",
        "http_serde_ext::path_and_query::vec_deque",
        "http_serde_ext::path_and_query::linked_list",
        "http_serde_ext::path_and_query::hash_map",
        "http_serde_ext::path_and_query::btree_map"
    );

    test_hash!(
        PathAndQuery,
        PathAndQuery::from_static("/"),
        json!("/"),
        "/",
        "http_serde_ext::path_and_query::hash_map_key",
        "http_serde_ext::path_and_query::hash_set"
    );

    let fake: PathAndQuery = Faker.fake();
    test_all_no_intermediate_compare!(
        PathAndQuery,
        fake.clone(),
        "http_serde_ext::path_and_query",
        "http_serde_ext::path_and_query::option",
        "http_serde_ext::path_and_query::result",
        "http_serde_ext::path_and_query::vec",
        "http_serde_ext::path_and_query::vec_deque",
        "http_serde_ext::path_and_query::linked_list",
        "http_serde_ext::path_and_query::hash_map",
        "http_serde_ext::path_and_query::btree_map"
    );

    test_hash_no_intermediate_compare!(
        PathAndQuery,
        fake.clone(),
        "http_serde_ext::path_and_query::hash_map_key",
        "http_serde_ext::path_and_query::hash_set"
    );
}

#[test]
fn test_header_map_roundtrip() {
    test_all!(
        HeaderMap,
        HeaderMap::default(),
        json!({}),
        "{}",
        "http_serde_ext::header_map",
        "http_serde_ext::header_map::option",
        "http_serde_ext::header_map::result",
        "http_serde_ext::header_map::vec",
        "http_serde_ext::header_map::vec_deque",
        "http_serde_ext::header_map::linked_list",
        "http_serde_ext::header_map::hash_map",
        "http_serde_ext::header_map::btree_map"
    );

    let mut map = HeaderMap::new();
    map.insert("baz", HeaderValue::from_static("qux"));
    map.insert("foo", HeaderValue::from_static("bar"));
    map.append("two", HeaderValue::from_static("one"));
    map.append("two", HeaderValue::from_static("two"));

    test_all!(
        HeaderMap,
        map.clone(),
        json!({
            "foo": "bar",
            "baz": "qux",
            "two": ["one", "two"]
        }),
        "baz: qux\nfoo: bar\ntwo:\n- one\n- two",
        "http_serde_ext::header_map",
        "http_serde_ext::header_map::option",
        "http_serde_ext::header_map::result",
        "http_serde_ext::header_map::vec",
        "http_serde_ext::header_map::vec_deque",
        "http_serde_ext::header_map::linked_list",
        "http_serde_ext::header_map::hash_map",
        "http_serde_ext::header_map::btree_map"
    );

    let fake: HeaderMap = Faker.fake();
    test_all_no_intermediate_compare!(
        HeaderMap,
        fake.clone(),
        "http_serde_ext::header_map",
        "http_serde_ext::header_map::option",
        "http_serde_ext::header_map::result",
        "http_serde_ext::header_map::vec",
        "http_serde_ext::header_map::vec_deque",
        "http_serde_ext::header_map::linked_list",
        "http_serde_ext::header_map::hash_map",
        "http_serde_ext::header_map::btree_map"
    );
}

#[test]
fn test_header_map_generic_roundtrip() {
    test_all!(
        HeaderMap<String>,
        HeaderMap::default(),
        json!({}),
        "{}",
        "http_serde_ext::header_map_generic",
        "http_serde_ext::header_map_generic::option",
        "http_serde_ext::header_map_generic::result",
        "http_serde_ext::header_map_generic::vec",
        "http_serde_ext::header_map_generic::vec_deque",
        "http_serde_ext::header_map_generic::linked_list",
        "http_serde_ext::header_map_generic::hash_map",
        "http_serde_ext::header_map_generic::btree_map"
    );

    let fake: HeaderMap<String> = Faker.fake();
    test_all_no_intermediate_compare!(
        HeaderMap<String>,
        fake.clone(),
        "http_serde_ext::header_map_generic",
        "http_serde_ext::header_map_generic::option",
        "http_serde_ext::header_map_generic::result",
        "http_serde_ext::header_map_generic::vec",
        "http_serde_ext::header_map_generic::vec_deque",
        "http_serde_ext::header_map_generic::linked_list",
        "http_serde_ext::header_map_generic::hash_map",
        "http_serde_ext::header_map_generic::btree_map"
    );
}

#[test]
fn test_header_name_roundtrip() {
    test_all!(
        HeaderName,
        HeaderName::from_static("foo"),
        json!("foo"),
        "foo",
        "http_serde_ext::header_name",
        "http_serde_ext::header_name::option",
        "http_serde_ext::header_name::result",
        "http_serde_ext::header_name::vec",
        "http_serde_ext::header_name::vec_deque",
        "http_serde_ext::header_name::linked_list",
        "http_serde_ext::header_name::hash_map",
        "http_serde_ext::header_name::btree_map"
    );

    test_hash!(
        HeaderName,
        HeaderName::from_static("foo"),
        json!("foo"),
        "foo",
        "http_serde_ext::header_name::hash_map_key",
        "http_serde_ext::header_name::hash_set"
    );

    let fake: HeaderName = Faker.fake();
    test_all_no_intermediate_compare!(
        HeaderName,
        fake.clone(),
        "http_serde_ext::header_name",
        "http_serde_ext::header_name::option",
        "http_serde_ext::header_name::result",
        "http_serde_ext::header_name::vec",
        "http_serde_ext::header_name::vec_deque",
        "http_serde_ext::header_name::linked_list",
        "http_serde_ext::header_name::hash_map",
        "http_serde_ext::header_name::btree_map"
    );

    test_hash_no_intermediate_compare!(
        HeaderName,
        fake.clone(),
        "http_serde_ext::header_name::hash_map_key",
        "http_serde_ext::header_name::hash_set"
    );
}

#[test]
fn test_header_value_roundtrip() {
    test_all!(
        HeaderValue,
        HeaderValue::from_static("foo"),
        json!("foo"),
        "foo",
        "http_serde_ext::header_value",
        "http_serde_ext::header_value::option",
        "http_serde_ext::header_value::result",
        "http_serde_ext::header_value::vec",
        "http_serde_ext::header_value::vec_deque",
        "http_serde_ext::header_value::linked_list",
        "http_serde_ext::header_value::hash_map",
        "http_serde_ext::header_value::btree_map"
    );

    test_hash!(
        HeaderValue,
        HeaderValue::from_static("foo"),
        json!("foo"),
        "foo",
        "http_serde_ext::header_value::hash_map_key",
        "http_serde_ext::header_value::hash_set"
    );

    test_ord!(
        HeaderValue,
        HeaderValue::from_static("foo"),
        json!("foo"),
        "foo",
        "http_serde_ext::header_value::btree_map_key",
        "http_serde_ext::header_value::btree_set"
    );

    let fake: HeaderValue = Faker.fake();
    test_all_no_intermediate_compare!(
        HeaderValue,
        fake.clone(),
        "http_serde_ext::header_value",
        "http_serde_ext::header_value::option",
        "http_serde_ext::header_value::result",
        "http_serde_ext::header_value::vec",
        "http_serde_ext::header_value::vec_deque",
        "http_serde_ext::header_value::linked_list",
        "http_serde_ext::header_value::hash_map",
        "http_serde_ext::header_value::btree_map"
    );

    test_hash_no_intermediate_compare!(
        HeaderValue,
        fake.clone(),
        "http_serde_ext::header_value::hash_map_key",
        "http_serde_ext::header_value::hash_set"
    );

    test_ord_no_intermediate_compare!(
        HeaderValue,
        fake.clone(),
        "http_serde_ext::header_value::btree_map_key",
        "http_serde_ext::header_value::btree_set"
    );
}

#[test]
fn test_method_roundtrip() {
    test_all!(
        Method,
        Method::default(),
        json!("GET"),
        "GET",
        "http_serde_ext::method",
        "http_serde_ext::method::option",
        "http_serde_ext::method::result",
        "http_serde_ext::method::vec",
        "http_serde_ext::method::vec_deque",
        "http_serde_ext::method::linked_list",
        "http_serde_ext::method::hash_map",
        "http_serde_ext::method::btree_map"
    );

    test_hash!(
        Method,
        Method::default(),
        json!("GET"),
        "GET",
        "http_serde_ext::method::hash_map_key",
        "http_serde_ext::method::hash_set"
    );

    let fake: Method = Faker.fake();
    test_all_no_intermediate_compare!(
        Method,
        fake.clone(),
        "http_serde_ext::method",
        "http_serde_ext::method::option",
        "http_serde_ext::method::result",
        "http_serde_ext::method::vec",
        "http_serde_ext::method::vec_deque",
        "http_serde_ext::method::linked_list",
        "http_serde_ext::method::hash_map",
        "http_serde_ext::method::btree_map"
    );

    test_hash_no_intermediate_compare!(
        Method,
        fake.clone(),
        "http_serde_ext::method::hash_map_key",
        "http_serde_ext::method::hash_set"
    );
}

macro_rules! serde_json_roundtrip_res_req {
    ($ty:ty, $val:expr, $equate:expr, $path:expr, $json:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = serde_json::to_string(&wrapper).expect("serialize json to string");

        let de: Wrapper = serde_json::from_str(&ser).expect("deserialize json string");
        $equate(&de.0, &$val);

        let de: Wrapper = serde_json::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize json string from reader");
        $equate(&de.0, &$val);

        let ser = serde_json::to_value(wrapper).expect("serialize json to value");
        assert_eq!(ser, $json);

        let de: Wrapper = serde_json::from_value(ser).expect("deserialize json value");
        $equate(&de.0, &$val);
    }};
}

macro_rules! serde_json_no_intermediate_compare_roundtrip_res_req {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        #[derive(Serialize, Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper($val);
        let ser = serde_json::to_string(&wrapper).expect("serialize json to string");

        let de: Wrapper = serde_json::from_str(&ser).expect("deserialize json string");
        $equate(&de.0, &$val);

        let de: Wrapper = serde_json::from_reader(io::Cursor::new(ser.as_bytes()))
            .expect("deserialize json string from reader");
        $equate(&de.0, &$val);

        let ser = serde_json::to_value(wrapper).expect("serialize json to value");
        let de: Wrapper = serde_json::from_value(ser).expect("deserialize json value");
        $equate(&de.0, &$val);
    }};
}

macro_rules! roundtrip_res_req {
    ($ty:ty, $val:expr, $equate:expr, $path:expr, $json:expr, $yaml:expr) => {{
        serde_json_roundtrip_res_req!($ty, $val, $equate, $path, $json);
        serde_yaml_roundtrip!($ty, $val, $equate, $path, $yaml);
        serde_cbor_roundtrip!($ty, $val, $equate, $path);
        bincode_roundtrip!($ty, $val, $equate, $path);
        postcard_roundtrip!($ty, $val, $equate, $path);
    }};
}

macro_rules! no_intermediate_compare_roundtrip_res_req {
    ($ty:ty, $val:expr, $equate:expr, $path:expr) => {{
        serde_json_no_intermediate_compare_roundtrip_res_req!($ty, $val, $equate, $path);
        serde_yaml_no_intermediate_compare_roundtrip!($ty, $val, $equate, $path);
        serde_cbor_roundtrip!($ty, $val, $equate, $path);
        bincode_roundtrip!($ty, $val, $equate, $path);
        postcard_roundtrip!($ty, $val, $equate, $path);
    }};
}

macro_rules! test_all_res_req {
        ($ty:ty, $val:expr, $json:expr, $yaml:expr, $equate:ident, $path:literal, $option:literal, $result:literal, $vec:literal, $vec_deque:literal, $linked_list:literal, $hash_map:literal, $btree_map:literal) => {{
            roundtrip_res_req!($ty, $val, |a, b| $equate(a, b), $path, json!($json), format!("{}\n", $yaml));

            roundtrip_res_req!(
                Option<$ty>,
                Some($val),
                |a: &Option<$ty>, b: &Option<$ty>| $equate(a.as_ref().unwrap(), b.as_ref().unwrap()),
                $option,
                json!($json), format!("{}\n", $yaml)
            );

            roundtrip_res_req!(Option<$ty>, None, |a: &Option<$ty>, b: &Option<$ty>| { assert!(a.is_none()); assert!(b.is_none()); }, $option, json!(null), "null\n");

            roundtrip_res_req!(
                Result<$ty, String>,
                Ok($val),
                |a: &Result<$ty, String>, b: &Result<$ty, String>| $equate(a.as_ref().unwrap(), b.as_ref().unwrap()),
                $result,
                json!({"Ok": $json}), format!("!Ok{}\n", $yaml)
            );

            roundtrip_res_req!(
                Result<$ty, String>,
                Err(String::default()),
                |a: &Result<$ty, String>, b: &Result<$ty, String>| { a.as_ref().err().unwrap() == b.as_ref().err().unwrap() },
                $result,
                json!({"Err": ""}), "!Err''\n"
            );

            roundtrip_res_req!(Vec<$ty>, vec![$val], |a: &Vec<$ty>, b: &Vec<$ty>| $equate(&a[0], &b[0]), $vec, json!([$json]), format!("- {}", $yaml));
            roundtrip_res_req!(
                VecDeque<$ty>,
                VecDeque::from([$val]),
                |a: &VecDeque<$ty>, b: &VecDeque<$ty>| $equate(&a[0], &b[0]),
                $vec_deque,
                json!([$json]), format!("- {}", $yaml)
            );
            roundtrip_res_req!(
                LinkedList<$ty>,
                LinkedList::from([$val]),
                |a: &LinkedList<$ty>, b: &LinkedList<$ty>| $equate(a.front().unwrap(), b.front().unwrap()),
                $linked_list,
                json!([$json]), format!("- {}", $yaml)
            );
            roundtrip_res_req!(
                HashMap<String, $ty>,
                HashMap::from([("foo".to_string(), $val)]),
                |a: &HashMap<String, $ty>, b: &HashMap<String, $ty>| $equate(&a["foo"], &b["foo"]),
                $hash_map,
                json!({"foo": $json}), format!("foo:\n  {}", $yaml)
            );
            roundtrip_res_req!(
                BTreeMap<String, $ty>,
                BTreeMap::from([("foo".to_string(), $val)]),
                |a: &BTreeMap<String, $ty>, b: &BTreeMap<String, $ty>| $equate(&a["foo"], &b["foo"]),
                $btree_map,
                json!({"foo": $json}), format!("foo:\n  {}", $yaml)
            );
        }};
    }

macro_rules! test_all_no_intermediate_compare_res_req {
        ($ty:ty, $val:expr, $equate:ident, $path:literal, $option:literal, $result:literal, $vec:literal, $vec_deque:literal, $linked_list:literal, $hash_map:literal, $btree_map:literal) => {{
            no_intermediate_compare_roundtrip_res_req!($ty, $val, |a, b| $equate(a, b), $path);

            no_intermediate_compare_roundtrip_res_req!(
                Option<$ty>,
                Some($val),
                |a: &Option<$ty>, b: &Option<$ty>| $equate(a.as_ref().unwrap(), b.as_ref().unwrap()),
                $option
            );

            no_intermediate_compare_roundtrip_res_req!(Option<$ty>, None, |a: &Option<$ty>, b: &Option<$ty>| { assert!(a.is_none()); assert!(b.is_none()); }, $option);

            no_intermediate_compare_roundtrip_res_req!(
                Result<$ty, String>,
                Ok($val),
                |a: &Result<$ty, String>, b: &Result<$ty, String>| $equate(a.as_ref().unwrap(), b.as_ref().unwrap()),
                $result
            );

            no_intermediate_compare_roundtrip_res_req!(
                Result<$ty, String>,
                Err(String::default()),
                |a: &Result<$ty, String>, b: &Result<$ty, String>|  { a.as_ref().err().unwrap() == b.as_ref().err().unwrap() },
                $result
            );


            no_intermediate_compare_roundtrip_res_req!(Vec<$ty>, vec![$val], |a: &Vec<$ty>, b: &Vec<$ty>| $equate(&a[0], &b[0]), $vec);
            no_intermediate_compare_roundtrip_res_req!(
                VecDeque<$ty>,
                VecDeque::from([$val]),
                |a: &VecDeque<$ty>, b: &VecDeque<$ty>| $equate(&a[0], &b[0]),
                $vec_deque
            );
            no_intermediate_compare_roundtrip_res_req!(
                LinkedList<$ty>,
                LinkedList::from([$val]),
                |a: &LinkedList<$ty>, b: &LinkedList<$ty>| $equate(a.front().unwrap(), b.front().unwrap()),
                $linked_list
            );
            no_intermediate_compare_roundtrip_res_req!(
                HashMap<String, $ty>,
                HashMap::from([("foo".to_string(), $val)]),
                |a: &HashMap<String, $ty>, b: &HashMap<String, $ty>| $equate(&a["foo"], &b["foo"]),
                $hash_map
            );
            no_intermediate_compare_roundtrip_res_req!(
                BTreeMap<String, $ty>,
                BTreeMap::from([("foo".to_string(), $val)]),
                |a: &BTreeMap<String, $ty>, b: &BTreeMap<String, $ty>| $equate(&a["foo"], &b["foo"]),
                $btree_map
            );
        }};
    }

#[test]
fn test_response_roundtrip() {
    fn equate<T: Debug + Eq>(a: &Response<T>, b: &Response<T>) {
        assert_eq!(a.body(), b.body());
        assert_eq!(a.status(), b.status());
        assert_eq!(a.headers(), b.headers());
        assert_eq!(a.version(), b.version());
        assert!(a.extensions().is_empty());
        assert!(b.extensions().is_empty());
    }

    test_all_res_req!(
        Response<()>,
        Response::default(),
        json!({
            "head": {
                "status": 200,
                "headers": {},
                "version": "HTTP/1.1"
            },
            "body": null
        }),
        "head:\n  status: 200\n  headers: {}\n  version: HTTP/1.1\nbody: null",
        equate,
        "http_serde_ext::response",
        "http_serde_ext::response::option",
        "http_serde_ext::response::result",
        "http_serde_ext::response::vec",
        "http_serde_ext::response::vec_deque",
        "http_serde_ext::response::linked_list",
        "http_serde_ext::response::hash_map",
        "http_serde_ext::response::btree_map"
    );

    let response: Response<String> = Faker.fake();
    let status = response.status();
    let headers = response.headers().clone();
    let version = response.version();
    let body = response.body();

    let response = || {
        let mut builder = Response::builder().status(status).version(version);
        std::mem::swap(builder.headers_mut().unwrap(), &mut headers.clone());
        builder.body(body.clone()).unwrap()
    };
    test_all_no_intermediate_compare_res_req!(
        Response<String>,
        response(),
        equate,
        "http_serde_ext::response",
        "http_serde_ext::response::option",
        "http_serde_ext::response::result",
        "http_serde_ext::response::vec",
        "http_serde_ext::response::vec_deque",
        "http_serde_ext::response::linked_list",
        "http_serde_ext::response::hash_map",
        "http_serde_ext::response::btree_map"
    );
}

#[test]
fn test_request_roundtrip() {
    fn equate<T: std::fmt::Debug + Eq>(a: &Request<T>, b: &Request<T>) {
        assert_eq!(a.body(), b.body());
        assert_eq!(a.method(), b.method());
        assert_eq!(a.uri(), b.uri());
        assert_eq!(a.headers(), b.headers());
        assert_eq!(a.version(), b.version());
        assert!(a.extensions().is_empty());
        assert!(b.extensions().is_empty());
    }

    test_all_res_req!(
        Request<()>,
        Request::default(),
        json!({
            "head": {
                "method": "GET",
                "uri": "/",
                "headers": {},
                "version": "HTTP/1.1"
            },
            "body": null
        }),
        "head:\n  method: GET\n  uri: /\n  headers: {}\n  version: HTTP/1.1\nbody: null",
        equate,
        "http_serde_ext::request",
        "http_serde_ext::request::option",
        "http_serde_ext::request::result",
        "http_serde_ext::request::vec",
        "http_serde_ext::request::vec_deque",
        "http_serde_ext::request::linked_list",
        "http_serde_ext::request::hash_map",
        "http_serde_ext::request::btree_map"
    );

    let request: Request<String> = Faker.fake();
    let method = request.method();
    let uri = request.uri();
    let headers = request.headers().clone();
    let version = request.version();
    let body = request.body();

    let request = || {
        let mut builder = Request::builder().method(method).uri(uri).version(version);
        std::mem::swap(builder.headers_mut().unwrap(), &mut headers.clone());
        builder.body(body.clone()).unwrap()
    };

    test_all_no_intermediate_compare_res_req!(
        Request<String>,
        request(),
        equate,
        "http_serde_ext::request",
        "http_serde_ext::request::option",
        "http_serde_ext::request::result",
        "http_serde_ext::request::vec",
        "http_serde_ext::request::vec_deque",
        "http_serde_ext::request::linked_list",
        "http_serde_ext::request::hash_map",
        "http_serde_ext::request::btree_map"
    );
}

#[test]
fn test_status_code_roundtrip() {
    test_all!(
        StatusCode,
        StatusCode::default(),
        json!(200),
        "200",
        "http_serde_ext::status_code",
        "http_serde_ext::status_code::option",
        "http_serde_ext::status_code::result",
        "http_serde_ext::status_code::vec",
        "http_serde_ext::status_code::vec_deque",
        "http_serde_ext::status_code::linked_list",
        "http_serde_ext::status_code::hash_map",
        "http_serde_ext::status_code::btree_map"
    );

    test_all!(
        StatusCode,
        StatusCode::NOT_MODIFIED,
        json!(304),
        "304",
        "http_serde_ext::status_code",
        "http_serde_ext::status_code::option",
        "http_serde_ext::status_code::result",
        "http_serde_ext::status_code::vec",
        "http_serde_ext::status_code::vec_deque",
        "http_serde_ext::status_code::linked_list",
        "http_serde_ext::status_code::hash_map",
        "http_serde_ext::status_code::btree_map"
    );

    let fake: StatusCode = Faker.fake();
    test_all_no_intermediate_compare!(
        StatusCode,
        fake,
        "http_serde_ext::status_code",
        "http_serde_ext::status_code::option",
        "http_serde_ext::status_code::result",
        "http_serde_ext::status_code::vec",
        "http_serde_ext::status_code::vec_deque",
        "http_serde_ext::status_code::linked_list",
        "http_serde_ext::status_code::hash_map",
        "http_serde_ext::status_code::btree_map"
    );

    test_hash_no_intermediate_compare!(
        StatusCode,
        fake,
        "http_serde_ext::status_code::hash_map_key",
        "http_serde_ext::status_code::hash_set"
    );

    test_ord_no_intermediate_compare!(
        StatusCode,
        fake,
        "http_serde_ext::status_code::btree_map_key",
        "http_serde_ext::status_code::btree_set"
    );
}

#[test]
fn test_uri_roundtrip() {
    test_all!(
        Uri,
        Uri::default(),
        json!("/"),
        "/",
        "http_serde_ext::uri",
        "http_serde_ext::uri::option",
        "http_serde_ext::uri::result",
        "http_serde_ext::uri::vec",
        "http_serde_ext::uri::vec_deque",
        "http_serde_ext::uri::linked_list",
        "http_serde_ext::uri::hash_map",
        "http_serde_ext::uri::btree_map"
    );

    test_all!(
        Uri,
        Uri::try_from("https://example.com").unwrap(),
        json!("https://example.com/"),
        "https://example.com/",
        "http_serde_ext::uri",
        "http_serde_ext::uri::option",
        "http_serde_ext::uri::result",
        "http_serde_ext::uri::vec",
        "http_serde_ext::uri::vec_deque",
        "http_serde_ext::uri::linked_list",
        "http_serde_ext::uri::hash_map",
        "http_serde_ext::uri::btree_map"
    );

    test_hash!(
        Uri,
        Uri::try_from("https://example.com").unwrap(),
        json!("https://example.com/"),
        "https://example.com/",
        "http_serde_ext::uri::hash_map_key",
        "http_serde_ext::uri::hash_set"
    );

    let fake: Uri = Faker.fake();
    test_all_no_intermediate_compare!(
        Uri,
        fake.clone(),
        "http_serde_ext::uri",
        "http_serde_ext::uri::option",
        "http_serde_ext::uri::result",
        "http_serde_ext::uri::vec",
        "http_serde_ext::uri::vec_deque",
        "http_serde_ext::uri::linked_list",
        "http_serde_ext::uri::hash_map",
        "http_serde_ext::uri::btree_map"
    );

    test_hash_no_intermediate_compare!(
        Uri,
        fake.clone(),
        "http_serde_ext::uri::hash_map_key",
        "http_serde_ext::uri::hash_set"
    );
}

#[test]
fn test_version_roundtrip() {
    test_all!(
        Version,
        Version::default(),
        json!("HTTP/1.1"),
        "HTTP/1.1",
        "http_serde_ext::version",
        "http_serde_ext::version::option",
        "http_serde_ext::version::result",
        "http_serde_ext::version::vec",
        "http_serde_ext::version::vec_deque",
        "http_serde_ext::version::linked_list",
        "http_serde_ext::version::hash_map",
        "http_serde_ext::version::btree_map"
    );

    test_hash!(
        Version,
        Version::default(),
        json!("HTTP/1.1"),
        "HTTP/1.1",
        "http_serde_ext::version::hash_map_key",
        "http_serde_ext::version::hash_set"
    );

    test_ord!(
        Version,
        Version::default(),
        json!("HTTP/1.1"),
        "HTTP/1.1",
        "http_serde_ext::version::btree_map_key",
        "http_serde_ext::version::btree_set"
    );

    let fake: Version = Faker.fake();
    test_all_no_intermediate_compare!(
        Version,
        fake,
        "http_serde_ext::version",
        "http_serde_ext::version::option",
        "http_serde_ext::version::result",
        "http_serde_ext::version::vec",
        "http_serde_ext::version::vec_deque",
        "http_serde_ext::version::linked_list",
        "http_serde_ext::version::hash_map",
        "http_serde_ext::version::btree_map"
    );

    test_hash_no_intermediate_compare!(
        Version,
        fake,
        "http_serde_ext::version::hash_map_key",
        "http_serde_ext::version::hash_set"
    );

    test_ord_no_intermediate_compare!(
        Version,
        fake,
        "http_serde_ext::version::btree_map_key",
        "http_serde_ext::version::btree_set"
    );
}

macro_rules! invalid_deserialize {
    ($ty:ty, $json:expr, $path:literal, $msg:tt) => {{
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let res = serde_json::from_value::<Wrapper>($json);
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().to_string(), $msg);
    }};
}

macro_rules! serde_json_res_req_invalid {
    ($ty:ty, $path:literal, $msg:expr) => {{
        let mut val = <$ty>::default();
        val.extensions_mut().insert(true);

        #[derive(Serialize)]
        struct Wrapper(#[serde(with = $path)] $ty);

        let wrapper = Wrapper(val);
        let result = serde_json::to_value(&wrapper);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), $msg);
    }};
}

#[test]
fn test_invalid() {
    invalid_deserialize!(
        Authority,
        json!("\\"),
        "http_serde_ext::authority",
        "invalid uri character"
    );

    let invalid_str = unsafe { std::str::from_utf8_unchecked(&[127]) };
    invalid_deserialize!(
        HeaderMap<String>,
        json!({invalid_str: "hello"}),
        "http_serde_ext::header_map_generic",
        "invalid HTTP header name"
    );
    invalid_deserialize!(
        HeaderMap,
        json!(""),
        "http_serde_ext::header_map",
        "invalid type: string \"\", expected a header map"
    );

    invalid_deserialize!(
        HeaderName,
        json!(invalid_str),
        "http_serde_ext::header_name",
        "invalid HTTP header name"
    );
    invalid_deserialize!(
        HeaderValue,
        json!(invalid_str),
        "http_serde_ext::header_value",
        "failed to parse header value"
    );

    invalid_deserialize!(
        Response<()>,
        json!({}),
        "http_serde_ext::response",
        "missing field `head`"
    );
    invalid_deserialize!(
        Request<()>,
        json!({"head": {}}),
        "http_serde_ext::request",
        "missing field `method`"
    );
    serde_json_res_req_invalid!(
        Response::<()>,
        "http_serde_ext::response",
        "extensions is not empty"
    );
    serde_json_res_req_invalid!(
        Request::<()>,
        "http_serde_ext::request",
        "extensions is not empty"
    );

    invalid_deserialize!(
        StatusCode,
        json!(1000),
        "http_serde_ext::status_code",
        "invalid status code"
    );
    invalid_deserialize!(Uri, json!(""), "http_serde_ext::uri", "empty string");
    invalid_deserialize!(
        Version,
        json!("HTTP/0.0"),
        "http_serde_ext::version",
        "invalid value: string \"HTTP/0.0\", expected a version string"
    );
}
