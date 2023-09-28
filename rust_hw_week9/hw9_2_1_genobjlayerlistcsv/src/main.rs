use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}
#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    circles: Vec<Circle>,
}

fn hexcode_gen<R: Rng>(rng: &mut R) -> String {
    let mut hexcode = "#".to_string();
    for _ in 0..8 {
        let digit = rng.gen_range(0..=15);
        match digit {
            10 => {
                hexcode += "A";
            }
            11 => {
                hexcode += "B";
            }
            12 => {
                hexcode += "C";
            }
            13 => {
                hexcode += "D";
            }
            14 => {
                hexcode += "E";
            }
            15 => {
                hexcode += "F";
            }
            _ => {
                hexcode += &digit.to_string();
            }
        }
    }
    hexcode
}

fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: i64) -> Vec<Layer>{
    let mut return_vec = Vec::new();
    for layer in 0..n {
        let circle_amount = rng.gen_range(20 ..=50);
        let mut circles = Vec::new();
        for circle in 0..circle_amount{
            let x = rng.gen_range(-100. ..=100.);
            let y = rng.gen_range(-100. ..=100.);
            let r = rng.gen_range(-10. ..=20.);
            circles.push(Circle{
                x,
                y,
                r,
            })
        }
        let name =  format!("Layer {layer}");
        let color = hexcode_gen(rng);
        return_vec.push(Layer{
            name,
            color,
            circles,
        })
    }
    return_vec
}

fn writecsv(n: i64) -> std::io::Result<()>{
    let mut rng = rand::thread_rng();
    let mut file = File::create("layers.csv")?;
    let gened_list = gen_obj_layer_list(&mut rng, n);
    let mut csv_string = String::new();
    for layer in gened_list{
        let name = layer.name;
        csv_string += &name;
        let color = format!(", {}", layer.color);
        csv_string += &color;
        for circles in layer.circles{
            let x = format!(", {}", circles.x);
            csv_string += &x;
            let y = format!(", {}", circles.y);
            csv_string += &y;
            let r = format!(", {}", circles.r);
            csv_string += &r;
        }
        csv_string += "\n";
    }
    file.write_all(csv_string.as_bytes())?;
    Ok(())
}

fn main() {
    writecsv(100);
}