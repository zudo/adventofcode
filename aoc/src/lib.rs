pub mod get;
pub mod post;
use chrono::FixedOffset;
use lazy_static::lazy_static;
lazy_static! {
    static ref TZ: FixedOffset = FixedOffset::west_opt(5 * 3600).unwrap();
}
