## [`serde`](https://github.com/serde-rs/serde) extensions for the [`http`](https://github.com/hyperium/http) crate types

Allows serializing and deserializing the following types from [`http`](https://github.com/hyperium/http):

- [`Response`](src/response.rs)
- [`Request`](src/request.rs)
- [`HeaderMap`](src/header_map.rs)
- [`StatusCode`](src/status_code.rs)
- [`Uri`](src/uri.rs)
- [`Method`](src/method.rs)
- [`HeaderName`](src/header_name.rs)
- [`HeaderValue`](src/header_value.rs)
- [`uri::Authority`](src/authority.rs)
- [`Version`](src/version.rs)
- Generic [`HeaderMap<T>`](src/header_map_generic.rs) where the item is not a `HeaderValue`

Allows serializing and deserializing the above types wrapped in the following `std` container types:

- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [`LinkedList`](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html) in the `Value` position
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) in the `Value` position

# Usage

This library is intended to be used with `serde`'s `derive` feature.
Fields should use the appropriate `#[serde(with = "...")]` annotation for that
type. Full examples are provided in each module section of these docs.

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
