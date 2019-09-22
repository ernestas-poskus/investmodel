use chrono::offset::TimeZone;
use chrono::offset::Utc;
use chrono::{Date, Datelike, Duration, NaiveDate};
use std::fmt;

#[derive(Debug)]
pub struct Ticker {
    pub symbol: Symbol,
    pub ticks: Vec<Tick>,
}

impl Ticker {
    pub fn new(symbol: Symbol, ticks: Vec<Tick>) -> Self {
        Self {
            symbol: symbol,
            ticks: ticks,
        }
    }
}

#[derive(Debug, Clone)]
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
pub struct Tick((Date<Utc>, f32));

impl Tick {
    pub fn new_from_naive_date(date: NaiveDate, price: f32) -> Self {
        Self((Utc.ymd(date.year(), date.month(), date.day()), price))
    }

    pub fn tuple(&self) -> &(Date<Utc>, f32) {
        &self.0
    }

    pub fn date(&self) -> Date<Utc> {
        self.tuple().0
    }

    pub fn price(&self) -> f32 {
        self.tuple().1
    }

    /// ```
    /// use investmodel::Tick::Tick;
    /// use investmodel::Tick::Symbol;
    /// use investmodel::chrono::NaiveDate;
    ///
    /// let tick = Tick(NaiveDate::from_ymd(2019, 6, 20), 120);
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
        let diff = ((self.price() - price) / self.price()) * 100.0;
        if self.price() > price {
            -diff
        } else {
            diff.abs()
        }
    }

    pub fn date_plus_days(&self, days: i64) -> Date<Utc> {
        let date = self.date();
        Utc.ymd(date.year(), date.month(), date.day()) + Duration::days(days)
    }

    pub fn percent_diff_less_than(&self, price: f32, less_than: f32) -> bool {
        self.percent_diff(price) < less_than
    }

    pub fn percent_diff_more_than(&self, price: f32, more_than: f32) -> bool {
        self.percent_diff(price) > more_than
    }
}

impl fmt::Display for Tick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:.2}", self.date(), self.price())
    }
}
