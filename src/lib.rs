//! ## [`serde`] extensions for the [`http`] crate types
//!
//! Allows serializing and deserializing the following types from [`http`]:
//! - [`Request`](request)
//! - [`Response`](response)
//! - [`HeaderMap`](header_map)
//! - [`StatusCode`](status_code)
//! - [`Uri`](uri)
//! - [`Method`](method)
//! - [`HeaderName`](header_name)
//! - [`HeaderValue`](header_value)
//! - [`uri::Authority`](authority)
//! - [`uri::Scheme`](scheme)
//! - [`uri::PathAndQuery`](path_and_query)
//! - [`Version`](version)
//! - Generic [`HeaderMap<T>`](header_map_generic) where the item is not a `HeaderValue`
//!
//! Allows serializing and deserializing the above types wrapped in the following `std` container types:
//! - [`Option`]
//! - [`Result`] in the `Ok` position
//! - [`Vec`]
//! - [`VecDeque`](std::collections::VecDeque)
//! - [`LinkedList`](std::collections::LinkedList)
//! - [`HashMap`](std::collections::HashMap) as the `Key` for all except `HeaderMap`, `Request`, and `Response`. As the `Value` for all types.
//! - [`BTreeMap`](std::collections::BTreeMap)  as the `Key` only for `HeaderValue`, `StatusCode`, and `Version`. As the `Value` for all types.
//! - [`HashSet`](std::collections::HashSet) for all except `HeaderMap`, `Request`, and `Response`
//! - [`BTreeSet`](std::collections::BTreeSet) only for `HeaderValue`, `StatusCode`, and `Version`
//!
//! ## Usage
//!
//! This library is intended to be used with `serde`'s `derive` feature.
//! Fields should use the appropriate `#[serde(with = "...")]` annotation for that
//! type. Full examples are provided in each module section of these docs.
//! ```
//! use std::collections::*;
//!
//! use http::*;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!    #[serde(with = "http_serde_ext::response")]
//!    base: Response<Vec<u8>>,
//!
//!    #[serde(with = "http_serde_ext::request::option")]
//!    option: Option<Request<String>>,
//!
//!    #[serde(with = "http_serde_ext::method::vec")]
//!    vec: Vec<Method>,
//!
//!    #[serde(with = "http_serde_ext::uri::vec_deque")]
//!    vec_deque: VecDeque<Uri>,
//!
//!    #[serde(with = "http_serde_ext::header_map::linked_list")]
//!    linked_list: LinkedList<HeaderMap>,
//!
//!    #[serde(with = "http_serde_ext::header_map_generic::hash_map")]
//!    hash_map: HashMap<String, HeaderMap<String>>,
//!
//!    #[serde(with = "http_serde_ext::status_code::btree_map_key")]
//!    btree_map_key: BTreeMap<StatusCode, i32>,
//!
//!    #[serde(with = "http_serde_ext::authority::hash_set")]
//!    hash_set: HashSet<uri::Authority>,
//! }
//! ```
//!
//! This library can also be used to manually `De`/`Serialize` types if given a
//! `De`/`Serializer`. For example, when using `serde_json`:
//!
//! ```rust
//! let uri = http::Uri::default();
//! let serialized = http_serde_ext::uri::serialize(&uri, serde_json::value::Serializer).unwrap();
//! let deserialized = http_serde_ext::uri::deserialize(serialized).unwrap();
//! assert_eq!(uri, deserialized);
//!
//! let responses: Vec<http::Response<()>> = vec![http::Response::default()];
//! let serialized =
//!     http_serde_ext::response::vec::serialize(&responses, serde_json::value::Serializer)
//!         .unwrap();
//! let deserialized: Vec<http::Response<()>> =
//!     http_serde_ext::response::vec::deserialize(serialized).unwrap();
//! ```

#[macro_use]
mod macros;

#[derive(serde::Serialize)]
struct BorrowedNameWrapper<'a>(#[serde(with = "crate::header_name")] &'a http::HeaderName);

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Either<T> {
    One(T),
    Many(Vec<T>),
}

macro_rules! doc_mod {
    { $ty:ty, $path:ident$(, $generic:ident)? } => {
        #[doc = concat!(" [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize) for [`http::", stringify!($ty), "`]")]
        ///
        /// ```
        /// use std::{cmp::Ord, collections::*, hash::Hash};
        ///
        #[doc = concat!("use http::", stringify!($ty), ";")]
        /// use serde::{Serialize, Deserialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        #[doc = concat!("struct MyStruct<T", $(", ", stringify!($generic), )?">")]
        /// where
        ///     T: Serialize + for<'a> Deserialize<'a> + Hash + Ord,
        $(#[doc = concat!("    ", stringify!($generic), ": Serialize + for<'a> Deserialize<'a>,") ])?
        /// {
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "\")]")]
        #[doc = concat!("    base: ", stringify!($ty), $("<", stringify!($generic), ">",)? ",")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::option\")]")]
        #[doc = concat!("    option: Option<", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::result\")]")]
        #[doc = concat!("    result: Result<", stringify!($ty), $("<", stringify!($generic), ">",)? ", T>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec\")]")]
        #[doc = concat!("    vec: Vec<", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec_deque\")]")]
        #[doc = concat!("    vec_deque: VecDeque<", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::linked_list\")]")]
        #[doc = concat!("    linked_list: LinkedList<", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_map\")]")]
        #[doc = concat!("    hash_map: HashMap<T, ", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::btree_map\")]")]
        #[doc = concat!("    btree_map: BTreeMap<T, ", stringify!($ty), $("<", stringify!($generic), ">",)? ">,")]
        /// }
        /// ```
        pub mod $path;
    }
}

macro_rules! doc_mod_hash {
    ($ty:ty, $path:ident$(, $extra:expr)?) => {
        #[doc = concat!(" [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize) for [`http::"$(, $extra)?, stringify!($ty), "`]")]
        ///
        /// ```
        /// use std::{cmp::Ord, collections::*, hash::Hash};
        ///
        #[doc = concat!("use http::", $($extra,)? stringify!($ty), ";")]
        /// use serde::{Serialize, Deserialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct MyStruct<T, U>
        /// where
        ///     U: Serialize + for<'a> Deserialize<'a>,
        ///     T: Serialize + for<'a> Deserialize<'a> + Hash + Ord,
        /// {
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "\")]")]
        #[doc = concat!("    base: ", stringify!($ty), ",")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::option\")]")]
        #[doc = concat!("    option: Option<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::result\")]")]
        #[doc = concat!("    result: Result<", stringify!($ty), ", U>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec\")]")]
        #[doc = concat!("    vec: Vec<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec_deque\")]")]
        #[doc = concat!("    vec_deque: VecDeque<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::linked_list\")]")]
        #[doc = concat!("    linked_list: LinkedList<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_map\")]")]
        #[doc = concat!("    hash_map: HashMap<T, ", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_map_key\")]")]
        #[doc = concat!("    hash_map_key: HashMap<", stringify!($ty), ", U>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::btree_map\")]")]
        #[doc = concat!("    btree_map: BTreeMap<T, ", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_set\")]")]
        #[doc = concat!("    hash_set: HashSet<", stringify!($ty), ">,")]
        /// }
        /// ```
        pub mod $path;
    };
}

macro_rules! doc_mod_ord_and_hash {
    ($ty:ty, $path:ident) => {
        #[doc = concat!(" [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize) for [`http::", stringify!($ty), "`]")]
        ///
        /// ```
        /// use std::{cmp::Ord, collections::*, hash::Hash};
        ///
        #[doc = concat!("use http::", stringify!($ty), ";")]
        /// use serde::{Serialize, Deserialize};
        ///
        /// #[derive(Serialize, Deserialize)]
        /// struct MyStruct<T, U>
        /// where
        ///     U: Serialize + for<'a> Deserialize<'a>,
        ///     T: Serialize + for<'a> Deserialize<'a> + Hash + Ord,
        /// {
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "\")]")]
        #[doc = concat!("    base: ", stringify!($ty), ",")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::option\")]")]
        #[doc = concat!("    option: Option<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::result\")]")]
        #[doc = concat!("    result: Result<", stringify!($ty), ", U>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec\")]")]
        #[doc = concat!("    vec: Vec<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::vec_deque\")]")]
        #[doc = concat!("    vec_deque: VecDeque<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::linked_list\")]")]
        #[doc = concat!("    linked_list: LinkedList<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_map\")]")]
        #[doc = concat!("    hash_map: HashMap<T, ", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_map_key\")]")]
        #[doc = concat!("    hash_map_key: HashMap<", stringify!($ty), ", U>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::btree_map\")]")]
        #[doc = concat!("    btree_map: BTreeMap<T, ", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::btree_map_key\")]")]
        #[doc = concat!("    btree_map_key: BTreeMap<", stringify!($ty), ", U>,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::hash_set\")]")]
        #[doc = concat!("    hash_set: HashSet<", stringify!($ty), ">,")]
        ///
        #[doc = concat!("    #[serde(with = \"http_serde_ext::", stringify!($path), "::btree_set\")]")]
        #[doc = concat!("    btree_set: BTreeSet<", stringify!($ty), ">,")]
        /// }
        /// ```
        pub mod $path;
    };
}

doc_mod_hash!(Authority, authority, "uri::");
doc_mod!(HeaderMap, header_map);
doc_mod!(HeaderMap, header_map_generic, U);
doc_mod_hash!(HeaderName, header_name);
doc_mod_ord_and_hash!(HeaderValue, header_value);
doc_mod_hash!(Method, method);
doc_mod_hash!(PathAndQuery, path_and_query, "uri::");
doc_mod!(Request, request, U);
doc_mod!(Response, response, U);
doc_mod_hash!(Scheme, scheme, "uri::");
doc_mod_ord_and_hash!(StatusCode, status_code);
doc_mod_hash!(Uri, uri);
doc_mod_ord_and_hash!(Version, version);
