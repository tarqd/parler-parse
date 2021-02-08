use super::prelude::*;
use unhtml::{Error};
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromText)]
pub struct ApproxRelTimestampOffset(i64);

impl std::str::FromStr for ApproxRelTimestampOffset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let num = split.next().ok_or(())?.parse::<i64>().map_err(|_| ())?;
        let unit = split.next().ok_or(())?;
        let mult: i64 = match unit {
            "second" | "seconds" => 1,
            "minute" | "minutes" => 60,
            "hour" | "hours" => 60 * 60,
            "day" | "days" => 60 * 60 * 24,
            "week" | "weeks" => 60 * 60 * 24 * 7,
            // fuzzy
            "month" | "months" => 60 * 60 * 24 * 30,
            "year" | "years" => 60 * 60 * 24 * 30 * 365,
            _ => Err(())?,
        };

        Ok(Self(num * mult * -1))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromHtml)]
pub struct RelTimestamp {
    #[html(attr = "inner")]
    rel_ts: Option<String>,
    #[html(attr = "inner")]
    approx_ts_offset: Option<ApproxRelTimestampOffset>,
}