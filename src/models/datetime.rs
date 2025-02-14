use std::{fmt, str};

pub static FORMAT: &[time::format_description::FormatItem<'static>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second][optional [.[subsecond]]]"
);

pub static FORMAT_Z: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

pub static FORMAT_WITH_OFFSET: &[time::format_description::FormatItem<'static>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour]:[offset_minute]"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateTime(pub time::OffsetDateTime);

impl DateTime {
    pub fn now_utc() -> Self {
        Self(time::OffsetDateTime::now_utc())
    }

    pub fn from_unix_timestamp(epoch: i64) -> Result<Self, time::error::ComponentRange> {
        time::OffsetDateTime::from_unix_timestamp(epoch).map(Self)
    }
}

impl From<time::OffsetDateTime> for DateTime {
    fn from(dt: time::OffsetDateTime) -> Self {
        Self(dt)
    }
}

impl From<DateTime> for time::OffsetDateTime {
    fn from(dt: DateTime) -> time::OffsetDateTime {
        dt.0
    }
}

// static FORMAT_STR: &str = "%Y-%m-%dT%H:%M:%S%.f";

impl str::FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // So, easy uses multiple different formats. Just to have a laugh.
        // if it contains a +, it's:
        // 2023-01-20T19:31:50+00:00
        //
        // if it ends in Z it's
        // 2023-01-20T19:35:28Z
        //
        // otherwise it might be
        // 2021-11-01T12:00:18.044574

        if s.contains('+') {
            time::OffsetDateTime::parse(s, FORMAT_WITH_OFFSET)
                .map_err(|err| format!("parsing `{s}` as offset datetime: {err}"))
                .map(Self)
        } else if s.ends_with('Z') {
            time::PrimitiveDateTime::parse(s, FORMAT_Z)
                .map_err(|err| format!("parsing `{s}` as datetime zulu format: {err}"))
                .map(|dt| Self(dt.assume_utc()))
        } else {
            time::PrimitiveDateTime::parse(s, FORMAT)
                .map_err(|err| format!("parsing `{s}` as datetime simple: {err}"))
                .map(|dt| Self(dt.assume_utc()))
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.format(FORMAT).expect("Dateformat");
        f.write_str(&s)
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for DateTime {
    fn deserialize<D>(des: D) -> Result<DateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        des.deserialize_str(Visitor)
    }
}
struct Visitor;

impl serde::de::Visitor<'_> for Visitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a datestring")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse::<super::DateTime>().map_err(E::custom)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // micros:

    #[test]
    fn parse_easee_format() {
        time::PrimitiveDateTime::parse("2021-11-01T12:00:18.044574", FORMAT)
            .expect("parsing dt")
            .assume_utc();

        {
            let dt = time::PrimitiveDateTime::parse("2022-03-31T09:22:30.319709", FORMAT)
                .expect("parsing dt")
                .assume_utc();

            assert_eq!(dt.year(), 2022);
            assert_eq!(dt.month(), time::Month::March);
            assert_eq!(dt.day(), 31);

            assert_eq!(dt.hour(), 9);
            assert_eq!(dt.minute(), 22);
            assert_eq!(dt.second(), 30);

            assert_eq!(dt.microsecond(), 319709);
        }
    }

    #[test]
    fn parse_easee_offset_format() {
        "2023-01-20T19:31:50+00:00"
            .parse::<DateTime>()
            .expect("parsing");
        "2023-01-20T19:35:25+00:00"
            .parse::<DateTime>()
            .expect("parsing");
    }

    #[test]
    fn parse_easee_zulu_time() {
        "2023-01-20T19:31:46Z".parse::<DateTime>().expect("parsing");
        "2023-01-20T19:35:28Z".parse::<DateTime>().expect("parsing");
    }

    //#[test]
    //fn deserialize_dts() {
    //    println!("{}", chrono::Utc::now());
    //    "2022-03-31T09:22:30.319709"
    //        .parse::<DateTime>()
    //        .expect("parsing dt string");
    //}

    #[test]
    fn epoch_to_datetime() {
        DateTime::from_unix_timestamp(1682208130).expect("from unix");
    }
}
