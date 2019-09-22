use crate::invest::{Amount, Invest};

use crate::ticker::{Symbol, Ticker};
use chrono::Datelike;
use plotters::coord::IntoMonthly;
use plotters::prelude::*;

pub fn draw(symbol: Symbol, ticker: Ticker) -> Result<(), Box<dyn std::error::Error>> {
    // let root = BitMapBackend::new("stock.png", (1024 * 5, 768 * 2)).into_drawing_area();
    let root = SVGBackend::new("stock.svg", (1024 * 5, 768 * 2)).into_drawing_area();
    root.fill(&WHITE)?;

    let year = 2000;
    let amount = 1000.0;

    let ticker_from = ticker.ticks_from_year(year);

    // let invest_first = Invest::first_day_of_month(&mut Amount::new(amount), year, &ticker);
    // let invest_last = Invest::last_day_of_month(&mut Amount::new(amount), year, &ticker);
    let invest_first = Invest::each_day(
        &mut Amount::new(amount),
        year,
        &ticker_from.ticks_first_of_month(),
    );
    let invest_last = Invest::each_day(
        &mut Amount::new(amount),
        year,
        &ticker_from.ticks_last_of_month(),
    );

    let invest_middle = Invest::each_day(
        &mut Amount::new(amount),
        year,
        &ticker_from.ticks_middle_of_month(),
    );

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .caption(
            format!("{} stock price", symbol),
            ("Arial", 30.0).into_font(),
        )
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Right, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_ranged(
            (ticker_from.date_from()..ticker.date_to()).monthly(),
            ticker.price_from()..(ticker.price_to() + (ticker.price_to() * 0.1)),
        )?
        .set_secondary_coord(
            (invest_first.invested.date_from()..invest_last.invested.date_to()).monthly(),
            invest_first.invested.price_from()
                ..(invest_first.invested.price_to() + (invest_last.invested.price_to() * 0.1)),
        );

    chart
        .configure_mesh()
        .x_labels(70)
        .y_labels(50)
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|d| format!("{}-{:02}-{:02}", d.year(), d.month(), d.day()))
        .y_desc("Price")
        .line_style_2(&RGBColor(112, 128, 144))
        .draw()?;

    chart
        .configure_secondary_axes()
        .x_labels(10)
        .y_labels(50)
        .y_desc("Total")
        .draw()?;

    ///////////////////////// Daily
    chart
        .draw_series(LineSeries::new(
            ticker_from.ticks.iter().map(|ticker| *ticker.tuple()),
            &BLACK,
        ))?
        .label(format!("X: {} daily from {}", symbol, year))
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart.draw_series(
        ticker_from
            .ticks_first_of_month()
            .ticks
            .iter()
            .map(|ticker| Circle::new(*ticker.tuple(), 2, YELLOW.filled())),
    )?;

    chart.draw_series(
        ticker_from
            .ticks_middle_of_month()
            .ticks
            .iter()
            .map(|ticker| Circle::new(*ticker.tuple(), 1, GREEN.filled())),
    )?;

    chart.draw_series(
        ticker_from
            .ticks_last_of_month()
            .ticks
            .iter()
            .map(|ticker| Circle::new(*ticker.tuple(), 2, RED.filled())),
    )?;
    ///////////////////////// Daily

    ///////////////////////// KEEP
    chart
        .draw_secondary_series(LineSeries::new(
            invest_first.kept.ticks.iter().map(|ticker| *ticker.tuple()),
            &MAGENTA,
        ))?
        .label(format!(
            "Y: keep cash, total {:.0}",
            invest_first.kept.price_to()
        ))
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &MAGENTA));
    ///////////////////////// KEEP

    ///////////////////////// FIRST DAY
    chart
        .draw_secondary_series(LineSeries::new(
            invest_first
                .invested
                .ticks
                .iter()
                .map(|ticker| *ticker.tuple()),
            &YELLOW,
        ))?
        .label(format!(
            "Y: first market day of month, total {:.0}",
            invest_first.invested.price_to()
        ))
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &YELLOW));
    ///////////////////////// FIRST DAY

    ///////////////////////// MIDDLE DAY
    chart
        .draw_secondary_series(LineSeries::new(
            invest_middle
                .invested
                .ticks
                .iter()
                .map(|ticker| *ticker.tuple()),
            &GREEN,
        ))?
        .label(format!(
            "Y: middle market day of month, total {:.0}",
            invest_middle.invested.price_to()
        ))
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &GREEN));
    ///////////////////////// MIDDLE DAY

    ///////////////////////// LAST DAY
    chart
        .draw_secondary_series(LineSeries::new(
            invest_last
                .invested
                .ticks
                .iter()
                .map(|ticker| *ticker.tuple()),
            &RED,
        ))?
        .label(format!(
            "Y: last market day of month, total {:.0}",
            invest_last.invested.price_to()
        ))
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &RED));
    ///////////////////////// LAST DAY

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .margin(30)
        .background_style(&RGBColor(128, 128, 128))
        .draw()?;

    Ok(())
}
