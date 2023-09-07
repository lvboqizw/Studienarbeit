use std::{fs, collections::HashMap, io::{BufRead, BufReader}, path::{PathBuf}};
use serde_json;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Value {
    path: String,
    entropy: f32,
    chi_square: f32,
    mean: f32,
    monte_carlo_pi: f32,
    serial_correlation: f32
}

fn main() {
    let n = 10.0;

    let mut entropy_m: HashMap<String, f32> = HashMap::new();
    let mut chi_square_m: HashMap<String, f32> = HashMap::new();
    let mut mean_m: HashMap<String, f32> = HashMap::new();
    let mut monte_carlo_pi_m: HashMap<String, f32> = HashMap::new();
    let mut serial_correlation_m: HashMap<String, f32> = HashMap::new();

    // let mut entropy_o: Vec<(String, f32)> = Vec::new();
    // let mut chi_square_o: Vec<(String, f32)> = Vec::new();
    // let mut mean_o: Vec<(String, f32)> = Vec::new();
    // let mut monte_carlo_pi_o: Vec<(String, f32)> = Vec::new();
    // let mut serial_correlation_o: Vec<(String, f32)> = Vec::new();

    // Vec<(filename, original_value, encrypted_value)>
    let mut entropy: Vec<(String, f32, f32)> = Vec::new();
    let mut chi_square: Vec<(String, f32, f32)> = Vec::new();
    let mut mean: Vec<(String, f32, f32)> = Vec::new();
    let mut monte_carlo_pi: Vec<(String, f32, f32)> = Vec::new();
    let mut serial_correlation: Vec<(String, f32, f32)> = Vec::new();

    let entries = fs::read_dir("diff_lag/encrypted").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let file = fs::File::open(entry.path()).unwrap();
        let lines = BufReader::new(file).lines();
        for line in lines {
            let value: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
            let path_col: Vec<&str> = value.path.split("/").collect();
            let file_name = path_col[1].to_string();

            if !entropy_m.contains_key(&file_name) {
                entropy_m.insert(file_name.clone(), value.entropy);
            } else {
                entropy_m.insert(file_name.clone(), entropy_m.get(&file_name).unwrap() + value.entropy);
            }

            if !chi_square_m.contains_key(&file_name) {
                chi_square_m.insert(file_name.clone(), value.chi_square);
            } else {
                chi_square_m.insert(file_name.clone(), chi_square_m.get(&file_name).unwrap() + value.chi_square);
            }

            if !mean_m.contains_key(&file_name) {
                mean_m.insert(file_name.clone(), value.mean);
            } else {
                mean_m.insert(file_name.clone(), mean_m.get(&file_name).unwrap() + value.mean);
            }

            if !monte_carlo_pi_m.contains_key(&file_name) {
                monte_carlo_pi_m.insert(file_name.clone(), value.monte_carlo_pi);
            } else {
                monte_carlo_pi_m.insert(file_name.clone(), monte_carlo_pi_m.get(&file_name).unwrap() + value.monte_carlo_pi);
            }

            if !serial_correlation_m.contains_key(&file_name) {
                serial_correlation_m.insert(file_name.clone(), value.serial_correlation);
            } else {
                serial_correlation_m.insert(file_name.clone(), serial_correlation_m.get(&file_name).unwrap() + value.serial_correlation);
            }
        }
    }

    let file_org = fs::File::open("diff_lag/original").unwrap();
    let lines = BufReader::new(file_org).lines();
    for line in lines {
        let value: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
        let path_col: Vec<&str> = value.path.split("/").collect();
        let file_name = path_col[path_col.len() - 1].to_string();

        entropy.push((file_name.clone(), value.entropy, entropy_m.get(&file_name).unwrap()/n));
        chi_square.push((file_name.clone(), value.chi_square, chi_square_m.get(&file_name).unwrap()/n));
        mean.push((file_name.clone(), value.mean, mean_m.get(&file_name).unwrap()/n));
        monte_carlo_pi.push((file_name.clone(), value.monte_carlo_pi, monte_carlo_pi_m.get(&file_name).unwrap()/n));
        serial_correlation.push((file_name.clone(), value.serial_correlation, serial_correlation_m.get(&file_name).unwrap()/n));
    }


        draw(entropy, "Entropy.png");
        draw(chi_square, "Chi_square.png");
        draw(mean, "Mean.png");
        draw(monte_carlo_pi, "Monte_carlo_pi.png");
        draw(serial_correlation, "Serial_correlation.png");


}


fn draw(data: Vec<(String, f32, f32)>,  name: &str) {
    let mut path = PathBuf::new();
    path.push("threshold_files");
    path.push(name);

    if data.is_empty() {
        panic!("The data for drawing is empty");
    }

    let labels: Vec<&str> = data.iter().map(|(label, _, _)| label.as_str()).collect();

    let root_area = BitMapBackend::new(&path, (1200, 500))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let (x_min, x_max) = (0, data.len() as i32 - 1);
    let (data_min, data_max) = (
        *data.iter().map(|(_, _, y)| y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        *data.iter().map(|(_, _, y)| y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    );
    let (data_org_min, data_org_max) = (
        *data.iter().map(|(_, y, _)| y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        *data.iter().map(|(_, y, _)| y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    );

    let (y_min, y_max) = (
            // *data.iter().map(|(_, y)| y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(), 
        data_min.min(data_org_min),
        data_max.max(data_org_max)
    );

    // let title = &name[0..name.len() - 4];

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .margin(15)
        // .caption(title, ("sans-serif", 30))
        .build_cartesian_2d((x_min..x_max).into_segmented(), (y_min - y_min/7.0)..(y_max + y_max/2.0))
        .unwrap();

    ctx
        .configure_mesh()
        .x_desc("Language")
        .y_desc("Output")
        .x_label_formatter(&|x: &SegmentValue<i32>| {
            let tmp = match *x {
                SegmentValue::Exact(value) => value,
                SegmentValue::CenterOf(value) => value,
                SegmentValue::Last => 0,
            };
            labels[tmp as usize].to_string()
        })
        .draw()
        .unwrap();

    ctx
        .draw_series(
            Histogram::vertical(&ctx)
                .style(RED.mix(0.5).filled())
                .margin(15)
                .data(
                    data
                        .iter()
                        .enumerate()
                        .map(|(i, (_x, _y, z))| ((i as i32, *z)))),
        ).unwrap()
        .label("Entrypted")
        .legend(
            |(x,y)| Rectangle::new([(x - 15, y + 1), (x, y)], RED)
        );

    ctx
        .draw_series(
            Histogram::vertical(&ctx)
                .style(BLUE.mix(0.5).filled())
                .margin(15)
                .data(
                    data
                    .iter()
                    .enumerate()
                    .map(|(i, (_x, y, _z))| ((i as i32, *y)))),
        ).unwrap()
        .label("Original")
        .legend(
            |(x,y)| Rectangle::new([(x - 15, y + 1), (x, y)], BLUE)
        );

    ctx
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .margin(20)
        .legend_area_size(5)
        .border_style(&BLACK)
        .draw().unwrap();
}