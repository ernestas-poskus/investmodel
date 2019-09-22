extern crate chrono;
extern crate investmodel;

use std::collections::BTreeMap;
use std::env;
use std::path::Path;

use investmodel::sources::yahoo;
use investmodel::ticker::Symbol;

fn arg() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("Enter CSV file name to load");
    }

    let file = format!("{}.csv", args[1]);

    if !Path::new(&file).exists() {
        panic!("File not found: {}", file);
    }

    args[1].clone()
}

fn main() {
    let symbol = Symbol::new(arg());

    let data = yahoo::deserialize_from_csv(symbol.clone());

    investmodel::plot::draw(symbol, data);
}
