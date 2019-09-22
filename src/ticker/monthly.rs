use crate::ticker::Tick;
use chrono::Datelike;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MonthlyTicker {
    pub ticks: BTreeMap<i32, BTreeMap<u32, Vec<Tick>>>,
}

impl MonthlyTicker {
    pub fn new(ticks: &Vec<Tick>) -> Self {
        Self {
            ticks: {
                let mut month_ticks: BTreeMap<i32, BTreeMap<u32, Vec<Tick>>> = BTreeMap::new();

                for tick in ticks.iter() {
                    let date = tick.date();

                    month_ticks
                        .entry(date.year())
                        .and_modify(|months| {
                            months
                                .entry(date.month())
                                .and_modify(|ticks| {
                                    //
                                    ticks.push(tick.clone())
                                })
                                .or_insert(Vec::new());
                        })
                        .or_insert(BTreeMap::new());
                }

                month_ticks
            },
        }
    }

    pub fn middle_of_month(self) -> Vec<Tick> {
        let mut output_ticks: Vec<Tick> = Vec::new();

        for (_year, mut months) in self.ticks {
            for (_month, ticks) in months.iter_mut() {
                let middle = ticks.len() / 2;

                // println!("------------");
                // println!("TI {:?} LEN {}", ticks[middle - 1], ticks.len());
                // println!("TI {:?} LEN {}", ticks[middle], ticks.len());
                // println!("TI {:?} LEN {}", ticks[middle + 1], ticks.len());
                // println!("------------");

                let tick = ticks.remove(middle);
                output_ticks.push(tick);
            }
        }

        output_ticks
    }

    pub fn first_day_of_month(self) -> Vec<Tick> {
        let mut output_ticks: Vec<Tick> = Vec::new();

        for (_year, mut months) in self.ticks {
            for (_month, ticks) in months.iter_mut() {
                output_ticks.push(ticks.remove(0));
            }
        }

        output_ticks
    }

    pub fn last_day_of_month(self) -> Vec<Tick> {
        let mut output_ticks: Vec<Tick> = Vec::new();

        for (_year, mut months) in self.ticks {
            for (_month, ticks) in months.iter_mut() {
                output_ticks.push(ticks.remove(ticks.len() - 1));
            }
        }

        output_ticks
    }
}
