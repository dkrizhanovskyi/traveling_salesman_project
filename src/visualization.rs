use plotters::prelude::*;
use std::error::Error;
use crate::tsp::TSP;

pub fn visualize_route(
    tsp: &TSP,
    route: &Vec<usize>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(file_path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = tsp.coordinates.iter().map(|coord| coord.0).fold(f64::INFINITY, f64::min);
    let x_max = tsp.coordinates.iter().map(|coord| coord.0).fold(f64::NEG_INFINITY, f64::max);
    let y_min = tsp.coordinates.iter().map(|coord| coord.1).fold(f64::INFINITY, f64::min);
    let y_max = tsp.coordinates.iter().map(|coord| coord.1).fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(
            format!("TSP Route - Total Distance: {:.2}", tsp.calculate_total_distance(route)),
            ("sans-serif", 30).into_font(),
        )
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    let colors = [RED, BLUE, GREEN, CYAN, MAGENTA, YELLOW, BLACK];

    for (i, &city1) in route.iter().enumerate() {
        if i == route.len() - 1 {
            break;
        }
        let city2 = route[i + 1];
        let distance = tsp.distances[city1][city2];

        let color = &colors[i % colors.len()];

        chart.draw_series(LineSeries::new(
            vec![
                (tsp.coordinates[city1].0, tsp.coordinates[city1].1),
                (tsp.coordinates[city2].0, tsp.coordinates[city2].1),
            ],
            ShapeStyle {
                color: color.to_rgba(),
                filled: true,
                stroke_width: 2,
            },
        ))?;

        let midpoint = (
            (tsp.coordinates[city1].0 + tsp.coordinates[city2].0) / 2.0,
            (tsp.coordinates[city1].1 + tsp.coordinates[city2].1) / 2.0,
        );

        let text = format!("{:.2}", distance);
        chart.draw_series(std::iter::once(EmptyElement::at(midpoint)
            + Rectangle::new(
                [(0, 0), (text.len() as i32 * 10, 20)],
                ShapeStyle {
                    color: WHITE.mix(0.8).to_rgba(),
                    filled: true,
                    stroke_width: 0,
                },
            )
            + Text::new(text, (5, -10), ("sans-serif", 15).into_font().color(color))))?;
    }

    chart.draw_series(
        tsp.coordinates
            .iter()
            .enumerate()
            .map(|(index, &(x, y))| {
                let color = if index == 0 {
                    &GREEN
                } else if index == route[route.len() - 1] {
                    &RED
                } else {
                    &BLUE
                };

                EmptyElement::at((x, y))
                    + Circle::new((0, 0), 7, ShapeStyle::from(color).filled())
                    + Text::new(format!("{}", index), (10, 0), ("sans-serif", 15).into_font())
            }),
    )?;

    Ok(())
}
