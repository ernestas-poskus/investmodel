use crate::ticker::{Symbol, Tick, Ticker};
use chrono::NaiveDate;
use csv::{Error as CsvError, Reader};

#[derive(Debug, Serialize, Deserialize)]
struct YahooHistoricalData {
    #[serde(rename = "Date")]
    date: NaiveDate,
    #[serde(rename = "Open")]
    open: f32,
    #[serde(rename = "High")]
    high: f32,
    #[serde(rename = "Low")]
    low: f32,
    #[serde(rename = "Close")]
    close: f32,
    #[serde(rename = "Adj Close")]
    adjclose: f32,
    #[serde(rename = "Volume")]
    volume: f32,
}

pub fn deserialize_from_csv(symbol: Symbol) -> Ticker {
    let mut rdr = Reader::from_path(format!("{}.csv", symbol))
        .expect(&format!("File not found: {}.csv", symbol));

    let iter = rdr.deserialize();

    let mut ticks: Vec<Tick> = if let Some(size) = iter.size_hint().1 {
        Vec::with_capacity(size)
    } else {
        Vec::new()
    };

    for result in iter {
        let data: Result<YahooHistoricalData, CsvError> = result;

        match data {
            Ok(historical_data) => ticks.push(Tick::new_from_naive_date(
                historical_data.date,
                historical_data.adjclose,
            )),
            Err(e) => {
                println!("Error deserialize: {}", e);
            }
        }
    }

    Ticker::new(symbol, ticks)
}
