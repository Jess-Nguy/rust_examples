use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, TimeZone};
// use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
// use serde::{Deserialize, Serialize};

fn main() {
    // // The input time zone as a string
    // let time_zone_str = "America/Los_Angeles";

    // let time_zone_str = "UTC";

    // Parse the input date time in UTC
    // let date_time_str = "2023-07-31 12:34:56";
    // let naive_date_time =
    //     chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S").unwrap();

    // // Convert the input date time to the desired time zone
    // let time_zone: Tz = time_zone_str.parse().unwrap();

    // let local_date_time = Tokyo.from_local_datetime(&naive_date_time).unwrap();

    // // Print the result
    // println!("UTC: {}", naive_date_time);
    // println!("time_zone {time_zone}");
    // println!("Local time in {}: {}", time_zone_str, local_date_time);

    // let my_tz = DefaultTimeZone {
    //     time_zone: String::from("America/New_York"),
    //     timestamp: naive_date_time,
    // };

    // println!("\nmy_tz: {my_tz:?}");
    // println!("\nconvert: {:?}", my_tz.convert_time_zone());

    // convert_tz_to_fixed_offset();

    example_local_result_ambiguous();
}

/// convert String time zone "America/New_York" to FixedOffset of a UTC timestamp
fn convert_tz_to_fixed_offset() {
    // CREATE the time zone
    let time_zone_str = "Asia/Tokyo";
    // let time_zone_str = "America/Toronto";
    // let time_zone_str = "America/New_York";
    let time_zone: Option<Tz> = match time_zone_str.parse() {
        Ok(tz) => Some(tz),
        Err(_) => None,
    };
    println!("\ntime_zone: {:?}", time_zone);

    // SET the time zone type as Tz
    let time_zone: Tz = time_zone_str.parse().unwrap();
    println!("\nunwrapped - time_zone: {:?}", time_zone);

    // CREATE the timestamp
    let date_time_str = "2023-07-31 12:34:56";
    let naive_date_time =
        chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S").unwrap();
    println!("\nnaive_date_time: {:?}", naive_date_time);

    // set the time zone to the timestamp
    // let tz_timestamp: DateTime<Tz> = time_zone.from_local_datetime(&naive_date_time).unwrap();
    println!(
        "\nLocalResults: {:?}",
        time_zone.from_local_datetime(&naive_date_time)
    );
    let tz_timestamp: DateTime<Tz> = time_zone.from_local_datetime(&naive_date_time).unwrap();

    // change the time zone Tz to a FixedOffset
    let fixed_offset: DateTime<FixedOffset> = tz_timestamp.fixed_offset();

    println!("\nfixed_offset: {:?}", fixed_offset);
}

/// this helped to me to understand the LocalResult::Ambiguous for when to use single, earliest, latest functions
fn example_local_result_ambiguous() {
    // Assume daylight saving time change on November 5st, 2023
    let local = Local.with_ymd_and_hms(2023, 11, 5, 1, 30, 0); // 01:30:00

    match local {
        LocalResult::Single(dt) => {
            println!("Parsed datetime: {}", dt);
        }
        LocalResult::Ambiguous(dt1, dt2) => {
            println!(
                "Ambiguous local time, two possible interpretations: {} and {}",
                dt1, dt2
            );
        }
        LocalResult::None => {
            println!("Failed to parse datetime");
        }
    }
}
#[derive(Debug)]
pub struct DefaultTimeZone {
    pub time_zone: String,
    pub timestamp: NaiveDateTime,
}

impl DefaultTimeZone {
    #[warn(dead_code)]
    pub fn convert_time_zone(&self) -> Option<DateTime<Tz>> {
        let time_zone_str = &self.time_zone;
        let timestamp = self.timestamp;
        let time_zone: chrono_tz::Tz = time_zone_str.parse().ok()?;

        match time_zone.from_local_datetime(&timestamp) {
            LocalResult::Single(datetime) => Some(datetime),
            LocalResult::Ambiguous(_, datetime) => Some(datetime),
            LocalResult::None => None,
        }
    }
}

fn to_tz(time_zone_str: &str) -> Option<Tz> {
    let time_zone: Option<Tz> = match time_zone_str.parse() {
        Ok(tz) => Some(tz),
        Err(_) => None,
    };
    return time_zone;
}
