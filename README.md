## [`serde`](https://github.com/serde-rs/serde) extensions for the [`http`](https://github.com/hyperium/http) crate types

Allows serializing and deserializing the following types from [`http`](https://github.com/hyperium/http):

- [`Response`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/response)
- [`Request`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/request)
- [`HeaderMap`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/header_map)
- [`StatusCode`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/status_code)
- [`Uri`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/uri)
- [`Method`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/method)
- [`HeaderName`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/header_name)
- [`HeaderValue`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/header_value)
- [`uri::Authority`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/authority)
- [`Version`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/version)
- Generic [`HeaderMap<T>`](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext/header_map_generic) where the item is not a `HeaderValue`

Allows serializing and deserializing the above types wrapped in the following `std` container types:

- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html) in the `Value` position
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) in the `Value` position

### Installation

Run the following Cargo command in your project directory:

```bash
cargo add http-serde-ext
```

Or add the following line to your Cargo.toml:

```toml
http-serde-ext = "0.1.4"
```

### Usage

This library is intended to be used with `serde`'s `derive` feature.
Fields should use the appropriate `#[serde(with = "...")]` annotation for that
type. Full examples are provided in each module section of the [docs](https://docs.rs/http-serde-ext/0.1.4/http_serde_ext).

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

    #[serde(with = "http_serde_ext::status_code::btree_map")]
    btree_map: BTreeMap<i32, StatusCode>,
}
```

### Acknowledgements

This crate is heavily inspired by [Kornel's](https://github.com/kornelski) [`http-serde`](https://crates.io/crates/http-serde).
