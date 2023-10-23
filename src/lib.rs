//! ## [`serde`] extensions for the [`http`] crate types
//!
//! Allows serializing and deserializing the following types from [`http`]:
//! - [`Response`](response)
//! - [`Request`](request)
//! - [`HeaderMap`](header_map)
//! - [`StatusCode`](status_code)
//! - [`Uri`](uri)
//! - [`Method`](method)
//! - [`HeaderName`](header_name)
//! - [`HeaderValue`](header_value)
//! - [`uri::Authority`](authority)
//! - [`Version`](version)
//! - Generic [`HeaderMap<T>`](header_map_generic) where the item is not a `HeaderValue`
//!
//! Allows serializing and deserializing the above types wrapped in the following `std` container types:
//! - [`Option`]
//! - [`Vec`]
//! - [`VecDeque`](std::collections::VecDeque)
//! - [`LinkedList`](std::collections::LinkedList)
//! - [`HashMap`](std::collections::HashMap) in the `Value` position
//! - [`BTreeMap`](std::collections::BTreeMap) in the `Value` position
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
//!    #[serde(with = "http_serde_ext::status_code::btree_map")]
//!    btree_map: BTreeMap<i32, StatusCode>,
//! }
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
    ($ty:ty, $path:ident$(, $generic:ident)?$(; $extra:expr)?) => {
        #[doc = concat!(" [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize) for [`http::"$(, stringify!($extra))?, stringify!($ty), "`]")]
        ///
        /// ```
        /// use std::{cmp::Ord, collections::*, hash::Hash};
        ///
        #[doc = concat!("use http::", $(stringify!($extra),)? stringify!($ty), ";")]
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
    };
}

doc_mod!(Authority, authority; "uri::");
doc_mod!(HeaderMap, header_map);
doc_mod!(HeaderMap, header_map_generic, U);
doc_mod!(HeaderName, header_name);
doc_mod!(HeaderValue, header_value);
doc_mod!(Method, method);
doc_mod!(Request, request, U);
doc_mod!(Response, response, U);
doc_mod!(StatusCode, status_code);
doc_mod!(Uri, uri);
doc_mod!(Version, version);
