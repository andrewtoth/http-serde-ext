use std::{fmt, iter, vec::IntoIter};

use http::{
    header::{Entry, GetAll},
    HeaderName, HeaderValue,
};
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{header_value, BorrowedNameWrapper, Either, NameWrapper};

type Type = http::HeaderMap;
const EXPECT_MESSAGE: &str = "a header map";

#[derive(Serialize)]
struct BorrowedValueWrapper<'a>(#[serde(with = "crate::header_value")] &'a HeaderValue);

struct GetAllWrapper<'a>(GetAll<'a, HeaderValue>);

impl<'a> Serialize for GetAllWrapper<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            if iter.next().is_none() {
                if ser.is_human_readable() {
                    return header_value::serialize(first, ser);
                } else {
                    return ser.collect_seq(iter::once(BorrowedValueWrapper(first)));
                }
            };

            let count = iter.count() + 2;
            let mut seq = ser.serialize_seq(Some(count))?;
            for v in self.0.iter() {
                seq.serialize_element(&BorrowedValueWrapper(v))?;
            }
            seq.end()
        } else {
            Err(ser::Error::custom("header has no values"))
        }
    }
}

pub fn serialize<S>(headers: &Type, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.collect_map(
        headers
            .keys()
            .map(|k| (BorrowedNameWrapper(k), GetAllWrapper(headers.get_all(k)))),
    )
}

#[derive(Deserialize)]
struct ValueWrapper(#[serde(with = "crate::header_value")] HeaderValue);

#[inline]
fn insert_header_values(map: &mut Type, key: HeaderName, mut values: IntoIter<ValueWrapper>) {
    if let Entry::Vacant(e) = map.entry(key) {
        if let Some(first) = values.next() {
            let mut e = e.insert_entry(first.0);

            for val in values {
                e.append(val.0);
            }
        }
    }
}

struct Visitor {
    is_human_readable: bool,
}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Type;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECT_MESSAGE)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map = Type::with_capacity(access.size_hint().unwrap_or(0));

        if self.is_human_readable {
            while let Some((key, val)) = access.next_entry::<NameWrapper, Either<ValueWrapper>>()? {
                match val {
                    Either::One(val) => {
                        map.insert(key.0, val.0);
                    }
                    Either::Many(values) => {
                        insert_header_values(&mut map, key.0, values.into_iter());
                    }
                };
            }
        } else {
            while let Some((key, values)) = access.next_entry::<NameWrapper, Vec<ValueWrapper>>()? {
                insert_header_values(&mut map, key.0, values.into_iter());
            }
        }
        Ok(map)
    }
}

pub fn deserialize<'de, D>(de: D) -> Result<Type, D::Error>
where
    D: Deserializer<'de>,
{
    let is_human_readable = de.is_human_readable();
    de.deserialize_map(Visitor { is_human_readable })
}

derive_extension_types!(super::Type);
