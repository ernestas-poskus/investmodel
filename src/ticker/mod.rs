use chrono::offset::TimeZone;
use chrono::offset::Utc;
use chrono::{Date, Datelike, Duration, NaiveDate};
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Ticker {
    pub symbol: Arc<Symbol>,
    pub date: NaiveDate,

    pub price: f32,
}

impl Ticker {
    /// ```
    /// use investmodel::ticker::Ticker;
    /// use investmodel::ticker::Symbol;
    /// use investmodel::chrono::NaiveDate;
    /// use std::sync::Arc;
    ///
    /// let tick = Ticker {
    ///     symbol: Arc::new(Symbol::new("s".to_string())),
    ///     date: NaiveDate::from_ymd(2019, 6, 20),
    ///     price: 120.0
    /// };
    ///
    /// assert_eq!(tick.percent_diff(141.0), 17.5);
    /// assert_eq!(tick.percent_diff(204.0), 70.0);
    /// assert_eq!(tick.percent_diff(360.0), 200.0);
    /// assert_eq!(tick.percent_diff(60.0), -50.0);
    /// assert_eq!(tick.percent_diff(30.0), -75.0);
    /// assert_eq!(tick.percent_diff(90.0), -25.0);
    /// assert_eq!(tick.percent_diff(132.0), 10.0);
    /// assert_eq!(tick.percent_diff(108.0), -10.0);
    ///
    /// ```
    pub fn percent_diff(&self, price: f32) -> f32 {
        let diff = ((self.price - price) / self.price) * 100.0;
        if self.price > price {
            -diff
        } else {
            diff.abs()
        }
    }

    pub fn day_utc(&self) -> Date<Utc> {
        Utc.ymd(self.date.year(), self.date.month(), self.date.day())
    }

    pub fn day_utc_plus_days(&self, days: i64) -> Date<Utc> {
        Utc.ymd(self.date.year(), self.date.month(), self.date.day()) + Duration::days(days)
    }

    pub fn percent_diff_less_than(&self, price: f32, less_than: f32) -> bool {
        self.percent_diff(price) < less_than
    }

    pub fn percent_diff_more_than(&self, price: f32, more_than: f32) -> bool {
        self.percent_diff(price) > more_than
    }
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}: {:.2}", self.symbol, self.date, self.price)
    }
}
