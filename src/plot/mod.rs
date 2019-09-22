use crate::ticker::{Symbol, Ticker};
use plotters::coord::IntoMonthly;
use plotters::prelude::*;

pub fn draw(symbol: Symbol, ticker: Ticker) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("stock.svg", (1024 * 5, 768 * 2)).into_drawing_area();
    root.fill(&WHITE)?;

    let ticks = &ticker.ticks;

    let (from, to) = (&ticks[0], &ticks[ticks.len() - 1]);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(
            format!("{} stock price", symbol),
            ("Arial", 30.0).into_font(),
        )
        .build_ranged(
            (from.date()..to.date_plus_days(1)).monthly(),
            from.price()..(to.price() + (to.price() * 0.1)),
        )?;

    chart.draw_series(LineSeries::new(
        ticks.iter().map(|ticker| *ticker.tuple()),
        &GREEN,
    ))?;

    chart.draw_series(LineSeries::new(
        ticks.iter().map(|ticker| (ticker.date(), 100f32)),
        &BLUE,
    ))?;

    chart
        .configure_mesh()
        .x_labels(50)
        .y_labels(30)
        .line_style_2(&WHITE)
        .draw()?;

    Ok(())
}
