use csv::{ReaderBuilder, StringRecord, Trim};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, Write};

fn read() -> std::io::Result<()> {
    let mut csv_string = String::new();
    let rdr = File::open("layers.csv").unwrap();
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\n')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    for record in reader.records() {
        if let Ok(rec) = record {
            for layers in &rec {
                let split_layer: Vec<&str> = layers.split(", ").collect();
                if split_layer.len() > 1 {
                    let no_namecolor = &split_layer[2..];
                    let n = 3;
                    let radius: Vec<_> = no_namecolor
                        .iter()
                        .skip(n - 1)
                        .step_by(n)
                        .copied()
                        .collect();
                    let mut total_area: f64 = 0.;
                    for r in &radius {
                        let area = r.parse::<f64>().unwrap().powf(2.) * std::f32::consts::PI as f64;
                        total_area += area;
                    }
                    let average_area = total_area / radius.len() as f64;
                    let formatted_string = format!("{}, {}\n", split_layer[0], average_area);
                    csv_string += &formatted_string;
                }
            }
        }
    }
    let mut file = File::create("avgarea.csv")?;
    file.write_all(csv_string.as_bytes())?;
    Ok(())
}

fn main() {
    let _ = read();
}
