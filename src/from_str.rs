use std::{fmt, marker::PhantomData, str::FromStr};

use serde::de::{self, Visitor};

pub fn deserialize<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    de.deserialize_str(FromStrVisitor {
        _marker: PhantomData,
    })
}

#[derive(Default)]
struct FromOptStrVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    _marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for FromOptStrVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an optional FromStr str")
    }

    fn visit_some<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de.deserialize_str(FromStrVisitor {
            _marker: PhantomData,
        })
        .map(Some)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::from_str(value).map_err(E::custom).map(Some)
    }
}

pub fn deserialize_opt<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    de.deserialize_option(FromOptStrVisitor {
        _marker: PhantomData,
    })
}

#[derive(Default)]
struct FromStrVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    _marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for FromStrVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a FromStr str")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::from_str(value).map_err(E::custom)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn deserialize_u64() {
        #[derive(serde::Deserialize)]
        struct St {
            #[serde(deserialize_with = "deserialize")]
            strnum: u64,
        }

        let des = serde_json::from_str::<St>(r#" { "strnum": "2323"} "#).expect("deserializing");

        assert_eq!(des.strnum, 2323);
    }

    #[test]
    fn deserialize_opt_u64() {
        #[derive(serde::Deserialize)]
        struct St {
            #[serde(deserialize_with = "deserialize_opt")]
            strnum: Option<u64>,
        }

        let des = serde_json::from_str::<St>(r#" { "strnum": "2323"} "#).expect("deserializing");

        assert_eq!(des.strnum, Some(2323));
    }
}
