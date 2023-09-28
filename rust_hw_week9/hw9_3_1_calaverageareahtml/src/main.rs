use csv::{ReaderBuilder, StringRecord, Trim};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, Write};

fn read() -> std::io::Result<()> {
    let mut html_string = "<style>
    table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
<table>
  <tr>
    <th>Layer Name</th>
    <th>Average Area</th>
  </tr>".to_string();
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
                    let formatted_string = format!("\n\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>", split_layer[0], average_area);
                    html_string += &formatted_string;
                }
            }
        }
    }
    html_string += "\n</table>";
    let mut file = File::create("avgarea.html")?;
    file.write_all(html_string.as_bytes())?;
    Ok(())
}

fn main() {
    let _ = read();
}
