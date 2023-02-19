use chrono::Utc;

pub type DateTime = chrono::DateTime<Utc>;

static FORMAT_STR: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub mod format {

    use chrono::TimeZone;

    use super::{DateTime, FORMAT_STR};
    use std::fmt;

    struct Visitor;

    pub fn deserialize<'de, D>(des: D) -> Result<DateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        des.deserialize_str(Visitor)
    }

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = DateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a datestring")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let dt = chrono::NaiveDateTime::parse_from_str(s, FORMAT_STR).map_err(E::custom)?;
            Ok(chrono::Utc.from_utc_datetime(&dt))
        }
    }

    pub fn serialize<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = dt.format(FORMAT_STR).to_string();
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod tests {

    use chrono::{Datelike, TimeZone, Timelike, Utc};

    use super::*;

    // micros:

    #[test]
    fn parse_easee_format() {
        chrono::NaiveDateTime::parse_from_str("2021-11-01T12:00:18.044574", FORMAT_STR)
            .expect("parsing dt");

        chrono::NaiveDateTime::parse_from_str("2023-01-20T19:35:25+00:00", FORMAT_STR)
            .expect("parsing dt");
        chrono::NaiveDateTime::parse_from_str("2023-01-20T19:31:50+00:00", FORMAT_STR)
            .expect("parsing dt");

        {
            let dt =
                chrono::NaiveDateTime::parse_from_str("2022-03-31T09:22:30.319709", FORMAT_STR)
                    .expect("parsing dt");

            let utc_dt = Utc.from_utc_datetime(&dt);

            assert_eq!(utc_dt.year(), 2022);
            assert_eq!(utc_dt.month(), 3);
            assert_eq!(utc_dt.day(), 31);

            assert_eq!(utc_dt.hour(), 9);
            assert_eq!(utc_dt.minute(), 22);
            assert_eq!(utc_dt.second(), 30);

            assert_eq!(utc_dt.timestamp_subsec_micros(), 319709);
        }
    }

    //#[test]
    //fn deserialize_dts() {
    //    println!("{}", chrono::Utc::now());
    //    "2022-03-31T09:22:30.319709"
    //        .parse::<DateTime>()
    //        .expect("parsing dt string");
    //}
}
