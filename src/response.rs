use http::{response::Builder, HeaderMap, StatusCode, Version};
use serde::{de, Deserialize, Serialize};

type Type<T> = http::Response<T>;
const STRUCT_NAME: &str = "Response";

#[derive(Serialize)]
struct BorrowedHead<'a> {
    #[serde(with = "crate::status_code")]
    status: StatusCode,
    #[serde(with = "crate::header_map")]
    headers: &'a HeaderMap,
    #[serde(with = "crate::version")]
    version: Version,
}

impl<'a, T> From<&'a Type<T>> for BorrowedHead<'a> {
    fn from(val: &'a Type<T>) -> Self {
        Self {
            status: val.status(),
            headers: val.headers(),
            version: val.version(),
        }
    }
}

#[derive(Deserialize)]
struct Head {
    #[serde(with = "crate::status_code")]
    status: StatusCode,
    #[serde(with = "crate::header_map")]
    headers: HeaderMap,
    #[serde(with = "crate::version")]
    version: Version,
}

impl Head {
    fn try_into<T, E>(self, body: T) -> Result<Type<T>, E>
    where
        E: de::Error,
    {
        let mut builder = Builder::new().status(self.status).version(self.version);

        if let Some(headers) = builder.headers_mut() {
            headers.reserve(self.headers.len());
            headers.extend(self.headers);
        } else {
            return Err(de::Error::custom("builder doesn't have headers"));
        }

        builder.body(body).map_err(de::Error::custom)
    }
}

serde_request_response!(Type<T>, STRUCT_NAME, Head, BorrowedHead);

derive_extension_types!(super::Type<T>, T);
