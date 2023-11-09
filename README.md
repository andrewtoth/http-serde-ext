## [`serde`](https://github.com/serde-rs/serde) extensions for the [`http`](https://github.com/hyperium/http) crate types

Allows serializing and deserializing the following types from [`http`](https://github.com/hyperium/http):

- [`Response`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/response)
- [`Request`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/request)
- [`HeaderMap`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/header_map)
- [`StatusCode`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/status_code)
- [`Uri`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/uri)
- [`Method`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/method)
- [`HeaderName`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/header_name)
- [`HeaderValue`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/header_value)
- [`uri::Authority`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/authority)
- [`uri::Scheme`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/scheme)
- [`uri::PathAndQuery`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/path_and_query)
- [`Version`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/version)
- Generic [`HeaderMap<T>`](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext/header_map_generic) where the item is not a `HeaderValue`

Allows serializing and deserializing the above types wrapped in the following `std` container types:

- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) in the `Ok` position
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html) as the `Key` for all except `HeaderMap`, `Request`, and `Response`. As the `Value` for all types.
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) as the `Key` only for `HeaderValue`, `StatusCode`, and `Version`. As the `Value` for all types.
- [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) for all except `HeaderMap`, `Request`, and `Response`
- [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) only for `HeaderValue`, `StatusCode`, and `Version`

### Installation

Run the following Cargo command in your project directory:

```bash
cargo add http-serde-ext
```

Or add the following line to your Cargo.toml:

```toml
http-serde-ext = "0.1.8"
```

### Usage

This library is intended to be used with `serde`'s `derive` feature.
Fields should use the appropriate `#[serde(with = "...")]` annotation for that
type. Full examples are provided in each module section of the [docs](https://docs.rs/http-serde-ext/0.1.8/http_serde_ext).

```rust
use std::collections::*;

use http::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "http_serde_ext::response")]
    base: Response<Vec<u8>>,

    #[serde(with = "http_serde_ext::request::option")]
    option: Option<Request<String>>,

    #[serde(with = "http_serde_ext::method::vec")]
    vec: Vec<Method>,

    #[serde(with = "http_serde_ext::uri::vec_deque")]
    vec_deque: VecDeque<Uri>,

    #[serde(with = "http_serde_ext::header_map::linked_list")]
    linked_list: LinkedList<HeaderMap>,

    #[serde(with = "http_serde_ext::header_map_generic::hash_map")]
    hash_map: HashMap<String, HeaderMap<String>>,

    #[serde(with = "http_serde_ext::status_code::btree_map_key")]
    btree_map: BTreeMap<StatusCode, i32>,

    #[serde(with = "http_serde_ext::authority::hash_set")]
    hash_set: HashSet<uri::Authority>,
}
```

This library can also be used to manually `De`/`Serialize` types if given a
`De`/`Serializer`. For example, when using `serde_json`:

```rust
let uri = http::Uri::default();
let serialized = http_serde_ext::uri::serialize(&uri, serde_json::value::Serializer).unwrap();
let deserialized = http_serde_ext::uri::deserialize(serialized).unwrap();
assert_eq!(uri, deserialized);

let responses: Vec<http::Response<()>> = vec![http::Response::default()];
let serialized =
    http_serde_ext::response::vec::serialize(&responses, serde_json::value::Serializer)
        .unwrap();
let deserialized: Vec<http::Response<()>> =
    http_serde_ext::response::vec::deserialize(serialized).unwrap();
```

### Acknowledgements

This crate is heavily inspired by [Kornel's](https://github.com/kornelski) [`http-serde`](https://crates.io/crates/http-serde).
