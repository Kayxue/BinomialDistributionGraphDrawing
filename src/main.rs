use plotters::prelude::*;
use rand_distr::{Binomial, Distribution};

fn main() {
    let bin = Binomial::new(10000, 0.01).unwrap();

    let testResult: Vec<i32> = (0..10000)
        .into_iter()
        .map(|_| bin.sample(&mut rand::thread_rng()) as i32)
        .collect();

    let root = BitMapBackend::new("./output.png", (1280, 720)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Binomial", ("sans-serif", 50.0))
        .build_cartesian_2d(60..140, 0..500)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Success Count")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.5).filled())
                .data(testResult.iter().map(|x: &i32| (*x, 1))),
        )
        .unwrap();

    root.present().unwrap();
    println!("Chart output complete.")
}
