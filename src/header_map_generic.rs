use std::{fmt, iter, marker::PhantomData, vec::IntoIter};

use http::{
    header::{Entry, GetAll},
    HeaderName,
};
use serde::{
    de,
    ser::{self, SerializeSeq},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{BorrowedNameWrapper, Either, NameWrapper};

type Type<T> = http::HeaderMap<T>;
const EXPECT_MESSAGE: &str = "a header map";

struct GetAllWrapper<'a, T: Serialize>(GetAll<'a, T>);

impl<'a, T: Serialize> Serialize for GetAllWrapper<'a, T> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            if iter.next().is_none() {
                if ser.is_human_readable() {
                    return first.serialize(ser);
                } else {
                    return ser.collect_seq(iter::once(first));
                }
            };

            let count = iter.count() + 2;
            let mut seq = ser.serialize_seq(Some(count))?;
            for v in self.0.iter() {
                seq.serialize_element(v)?;
            }
            seq.end()
        } else {
            Err(ser::Error::custom("header has no values"))
        }
    }
}

pub fn serialize<S, T>(headers: &Type<T>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    ser.collect_map(
        headers
            .keys()
            .map(|k| (BorrowedNameWrapper(k), GetAllWrapper(headers.get_all(k)))),
    )
}

#[inline]
fn insert_header_values<T>(map: &mut Type<T>, key: HeaderName, mut values: IntoIter<T>) {
    if let Entry::Vacant(e) = map.entry(key) {
        if let Some(first) = values.next() {
            let mut e = e.insert_entry(first);

            for val in values {
                e.append(val);
            }
        }
    }
}

struct Visitor<T>
where
    T: for<'a> Deserialize<'a>,
{
    is_human_readable: bool,
    _ph: PhantomData<T>,
}

impl<'de, T> de::Visitor<'de> for Visitor<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Value = Type<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECT_MESSAGE)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map = Type::<T>::with_capacity(access.size_hint().unwrap_or_default());

        if self.is_human_readable {
            while let Some((key, val)) = access.next_entry::<NameWrapper, Either<T>>()? {
                match val {
                    Either::One(val) => {
                        map.insert(key.0, val);
                    }
                    Either::Many(values) => {
                        insert_header_values(&mut map, key.0, values.into_iter());
                    }
                };
            }
        } else {
            while let Some((key, values)) = access.next_entry::<NameWrapper, Vec<T>>()? {
                insert_header_values(&mut map, key.0, values.into_iter());
            }
        }

        Ok(map)
    }
}

pub fn deserialize<'de, D, T>(de: D) -> Result<Type<T>, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> Deserialize<'a>,
{
    let is_human_readable = de.is_human_readable();
    de.deserialize_map(Visitor::<T> {
        is_human_readable,
        _ph: PhantomData,
    })
}

derive_extension_types!(super::Type<T>, T);
