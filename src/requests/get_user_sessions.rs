use std::{io::Cursor, str::FromStr};

use crate::{BytesBody, Client, DateTime, Error, NoBody, Result};

pub struct GetUserSessions {
    site_id: i64,
    from: time::Date,
    to: time::Date,
}

impl GetUserSessions {
    /// Fetches sessions between [from - to[
    /// Notice that to is exclusive.
    ///
    /// So to fetch sessions ended on the 2nd of april 2023:
    /// * from:  2023-04-02
    /// * from:  2023-04-03
    pub fn new(site_id: i64, from_inclusive: time::Date, to_exclusive: time::Date) -> Self {
        Self {
            site_id,
            from: from_inclusive,
            to: to_exclusive,
        }
    }

    pub async fn send(&self, client: &Client) -> Result<Vec<UserSession>> {
        let df = time::macros::format_description!("[year]-[month]-[day]");

        let site_id = &self.site_id;
        let from_s = self.from.format(&df).unwrap();
        let to_s = self.to.format(&df).unwrap();

        let url = format!("api/sessions/export/{site_id}/1/{from_s}/{to_s}");

        let bs = client
            .req::<_, BytesBody>(http::Method::GET, &url, NoBody)
            .await?;

        parse_body(&bs)
    }
}

#[derive(serde::Deserialize)]
pub struct UserSession {
    #[serde(rename = "User")]
    pub user: String,

    #[serde(rename = "Phone")]
    pub phone: String,

    #[serde(rename = "Email")]
    pub email: String,

    #[serde(rename = "Charger Serial")]
    pub charger_serial: String,

    #[serde(rename = "Car Connected (Coordinated Universal Time)")]
    pub car_connected: DateTime,

    #[serde(rename = "Car Disconnected (Coordinated Universal Time)")]
    pub car_disconnected: DateTime,

    #[serde(rename = "Energy (kWh)")]
    pub kwh: f64,

    #[serde(rename = "Amount (ex.VAT) (SEK)")]
    pub amount_excl_vat: f64,

    #[serde(rename = "VAT (%)")]
    pub vat_percent: u8,

    #[serde(
        rename = "Total (inc.VAT) (SEK),",
        deserialize_with = "fuckedup_fucking_easee_shit"
    )]
    pub amount_incl_vat: f64,
}

fn fuckedup_fucking_easee_shit<'de, D, T>(des: D) -> std::result::Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    des.deserialize_str(LastFuckingCellVisitor(std::marker::PhantomData))
}

struct LastFuckingCellVisitor<T>(std::marker::PhantomData<T>);

impl<T> serde::de::Visitor<'_> for LastFuckingCellVisitor<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a fucked up last cell")
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.trim_end_matches(',').parse::<T>().map_err(E::custom)
    }
}

fn parse_body(bs: &[u8]) -> Result<Vec<UserSession>> {
    let s = std::str::from_utf8(bs).map_err(|err| crate::Error::InvalidUtf8(err.to_string()))?;

    let split = s.split(
        r#";;;,
;;;,
"#
        .trim(),
    );

    let csv = split
        .into_iter()
        .nth(3)
        .ok_or_else(|| Error::DeserializingCsv("searching for 4th csv block".into()))?
        .trim();

    // For a laugh, once removed the bullshit before the actual data.
    // we're faced with a special kind of csv:
    // A kind that mixes ';' and ',' as separators.
    // Each row ends in a comma instead of a semicolon.

    println!("PARSING `{csv}`");
    let mut res = Vec::with_capacity(csv.lines().count());

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .quoting(false)
        .has_headers(true)
        .from_reader(Cursor::new(csv));

    for row_res in rdr.deserialize::<UserSession>() {
        match row_res {
            Ok(row) => res.push(row),
            Err(err) => {
                return Err(Error::DeserializingCsv(format!(
                    "deserializing rows: {err}"
                )));
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {

    #[test]
    fn deserialize_session() {
        let s = r#"Easee;;Export Period;
;;Date from:;2025-03-01
User Detailed Report;;Date to:;2025-03-31,
;;;,
;;;,
Site Info;;;,
Site Key:;E6US-N222;;,
Site Name:;Brf Ryssjan 222;;,
Address:;Blecktornsstigen;;,
ZIP-code:;116 66;;,
City:;Stockholm;;,
Country:;Sweden;;,
Contact Person:;EcoVision AB;;,
Contact Phone:;+46703564597;;,
;;;,
;;;,
Total Consumption;;;,
Total Energy (kWh):;147.6;;,
Total Amount (ex.VAT) (SEK):;258.60;;
Total Amount (inc.VAT) (SEK):;323.25;;
;;;,
;;;,
User;Phone;Email;Charger Serial;Car Connected (Coordinated Universal Time);Car Disconnected (Coordinated Universal Time);Energy (kWh);Amount (ex.VAT) (SEK);VAT (%);Total (inc.VAT) (SEK),
Jens Fredin;+46739760709;jens.fredin@gmail.com;EC32KXFN;2025-03-03 15:52;2025-03-04 07:58;4.38;7.68;25;9.60,
Jens Fredin;+46739760709;jens.fredin@gmail.com;ECS7ZRJY;2025-03-04 16:05;2025-03-05 08:07;9.6;16.82;25;21.02,
Lars Olof Smedman;+46762227048;lars.smedman@paravida.se;EC32KXFN;2025-03-04 12:15;2025-03-05 08:32;54.1;94.78;25;118.48,
Magnus Cederäng;+46706501882;magnus.cederang@infralogic.se;ECS7ZRJY;2025-03-05 19:12;2025-03-06 06:01;54.06;94.71;25;118.39,
Jens Fredin;+46739760709;jens.fredin@gmail.com;EC32KXFN;2025-03-05 16:04;2025-03-06 08:09;9.56;16.75;25;20.94,
Jens Fredin;+46739760709;jens.fredin@gmail.com;EC32KXFN;2025-03-06 22:08;2025-03-07 13:50;8.92;15.63;25;19.53,
Jens Fredin;+46739760709;jens.fredin@gmail.com;EC32KXFN;2025-03-07 17:14;2025-03-08 07:45;4.27;7.48;25;9.35,
Jens Fredin;+46739760709;jens.fredin@gmail.com;EC32KXFN;2025-03-08 10:31;2025-03-08 16:24;2.71;4.75;25;5.94,

"#;

        let rows = super::parse_body(s.as_bytes()).expect("parsing body");

        assert_eq!(8, rows.len());
        assert_eq!(9.60, rows[0].amount_incl_vat);
        assert_eq!(5.94, rows[7].amount_incl_vat);
    }
}
