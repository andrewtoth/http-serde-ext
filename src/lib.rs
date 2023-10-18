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
    ($ty:ty, $path:ident$(, $generic:tt)?$(; $extra:expr)?) => {
        paste::paste! {
            #[doc = " [`Serialize`](serde::Serialize)/[`Deserialize`](serde::Deserialize) for [`http::" $($extra)? $ty "`](http::" $($extra)? $ty ")"]
            ///
            /// ```
            /// use std::{cmp::Ord, collections::*, hash::Hash};
            ///
            #[doc = " use http::" $($extra)? $ty ";"]
            /// use serde::{Serialize, Deserialize};
            ///
            /// #[derive(Serialize, Deserialize)]
            #[doc = " struct MyStruct<T"$(", " $generic )?">"]
            /// where
            ///     T: Serialize + for<'a> Deserialize<'a> + Hash + Ord,
            $(#[doc = "     " $generic ": Serialize + for<'a> Deserialize<'a>," ])?
            /// {
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "\")]"]
            #[doc = "    base: " $ty $("<" $generic ">")? ","]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::option\")]"]
            #[doc = "    option: Option<" $ty $("<" $generic ">")? ">,"]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::vec\")]"]
            #[doc = "    vec: Vec<" $ty $("<" $generic ">")? ">,"]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::vec_deque\")]"]
            #[doc = "    vec_deque: VecDeque<" $ty $("<" $generic ">")? ">,"]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::linked_list\")]"]
            #[doc = "    linked_list: LinkedList<" $ty $("<" $generic ">")? ">,"]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::hash_map\")]"]
            #[doc = "    hash_map: HashMap<T, " $ty $("<" $generic ">")? ">,"]
            ///
            #[doc = "    #[serde(with = \"http_serde_ext::" $path "::btree_map\")]"]
            #[doc = "    btree_map: BTreeMap<T, " $ty $("<" $generic ">")? ">,"]
            /// }
            /// ```
            pub mod $path;
        }
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
