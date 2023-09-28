use rand::Rng;

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

fn cal_average_area(layer_list: Vec<Layer>) -> Vec<(String, f64)>{
    let mut return_vec: Vec<(String, f64)> = Vec::new();
    for layer in &layer_list{
        let mut total_area = 0.;
        for circle in &layer.circles{
            let radius = circle.r.powf(2.) * std::f32::consts::PI as f64;
            total_area += radius;
        }
        let average_area = total_area/layer.circles.len() as f64;
        return_vec.push((layer.name.to_string(), average_area))
    }
    return_vec
}

fn main() {
    let mut rng = rand::thread_rng();
    let layer_test = gen_obj_layer_list(&mut rng, 10);
    let average_area = cal_average_area(layer_test);
    for i in average_area{
        println!("{}, {}", i.0, i.1)
    }
}

#[test]
fn test1() {
    let mut rng = rand::thread_rng();
    let n = 100;
    let layer_test = gen_obj_layer_list(&mut rng, n);
    let average_area = cal_average_area(layer_test);
    // output size is the same as input size
    if average_area.len() != n as usize{
        assert!(false)
    }
    for i in average_area{
        // average area has to be between the area of r=0 circle and r=20 circle
        if i.1 < 0. || i.1 > 1256.64{
            assert!(false)
        }
    }
}