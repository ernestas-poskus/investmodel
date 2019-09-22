use crate::ticker::{Tick, Ticker};
use chrono::{Date, Datelike, Utc};

#[derive(Debug, Clone)]
pub struct Amount {
    initial: f32,
    left: f32,
    lots: i64,

    // Options
    carry_leftover: bool,
}

impl Amount {
    pub fn new(initial: f32) -> Self {
        Self {
            initial: initial,
            left: 0f32,
            lots: 0,
            carry_leftover: true,
        }
    }

    fn amount(&self) -> f32 {
        if self.carry_leftover {
            self.initial + self.left
        } else {
            self.initial
        }
    }
}

pub struct Invest {
    pub kept: Ticker,
    pub invested: Ticker,
}

impl Invest {
    fn invest_on<F>(ticker: &Ticker, am: Amount, mut invest_func: F) -> Invest
    where
        F: FnMut(usize, Date<Utc>, f32, &mut Vec<Tick>) -> bool,
    {
        let mut kept_ticks: Vec<Tick> = Vec::new();
        let mut invested_ticks: Vec<Tick> = Vec::new();

        for (pos, tick) in ticker.ticks.iter().enumerate() {
            let date = tick.date();
            let price = tick.price();

            if invest_func(pos, date, price, &mut invested_ticks) {
                // Amount of money if we were to keep as cash
                kept_ticks.push(Tick::new(date, am.initial * invested_ticks.len() as f32));
            }
        }

        println!("INVESTED TIMES {}", invested_ticks.len());

        Invest {
            kept: Ticker::new(ticker.symbol.clone(), kept_ticks),
            invested: Ticker::new(ticker.symbol.clone(), invested_ticks),
        }
    }

    pub fn first_day_of_month(am: &mut Amount, from_year: i32, ticker: &Ticker) -> Invest {
        Self::invest_on(ticker, am.clone(), |pos, date, price, invested_ticks| {
            if date.year() < from_year {
                return false;
            }

            let prev_date = ticker
                .ticks
                .get(pos - 1)
                .map(|ticker| ticker.date())
                .unwrap_or(date);

            if prev_date.month() != date.month() {
                if price > am.amount() {
                    println!("Too high price: {} for amount: {}", price, am.amount());
                    return false;
                }

                let lots = (am.amount() / price).trunc() as i64;
                am.left = am.amount() - lots as f32 * price;
                am.lots += lots;

                invested_ticks.push(Tick::new(date, am.lots as f32 * price));
                return true;
            }
            false
        })
    }

    pub fn each_day(am: &mut Amount, from_year: i32, ticker: &Ticker) -> Invest {
        Self::invest_on(ticker, am.clone(), |_pos, date, price, invested_ticks| {
            if date.year() < from_year {
                return false;
            }

            if price > am.amount() {
                println!("Too high price: {} for amount: {}", price, am.amount());
                return false;
            }

            let lots = (am.amount() / price).trunc() as i64;
            am.left = am.amount() - lots as f32 * price;
            am.lots += lots;

            invested_ticks.push(Tick::new(date, am.lots as f32 * price));
            true
        })
    }

    pub fn last_day_of_month(am: &mut Amount, from_year: i32, ticker: &Ticker) -> Invest {
        let mut once = false;
        Self::invest_on(ticker, am.clone(), |pos, date, price, invested_ticks| {
            if date.year() < from_year {
                return false;
            }

            // First investment to land on first day of month
            // to fairly compare to `first_day_of_month`
            if !once {
                am.lots += (am.amount() / price).trunc() as i64;

                invested_ticks.push(Tick::new(date, am.lots as f32 * price));

                once = true;
                return true;
            }

            let next_date = ticker
                .ticks
                .get(pos + 1)
                .map(|ticker| ticker.date())
                .unwrap_or(date);

            if date.month() != next_date.month() {
                if price > am.amount() {
                    println!("Too high price: {} for amount: {}", price, am.amount());
                    return false;
                }

                let lots = (am.amount() / price).trunc() as i64;
                am.left = am.amount() - lots as f32 * price;
                am.lots += lots;

                invested_ticks.push(Tick::new(date, am.lots as f32 * price));
                return true;
            }
            false
        })
    }
}
