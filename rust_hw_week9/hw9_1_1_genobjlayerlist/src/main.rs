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

fn main() {
    let mut rng = rand::thread_rng();
    println!("{:?}", gen_obj_layer_list(&mut rng, 10));
}

#[test]
fn test1(){
    let mut rng = rand::thread_rng();
    let n = 1000;
    let gened_list = gen_obj_layer_list(&mut rng, n);
    // generated list must have the same amount of layers as specified
    if gened_list.len() != 1000 as usize{
        assert!(false);
    }
    for layer in gened_list{
        // the number of circles must be in between 20 and 50
        if layer.circles.len() > 50_usize || layer.circles.len() < 20_usize{
            assert!(false);
        }
        for circle in layer.circles{
            // -100 ≤ x ≤ 100 , -100 ≤ y ≤ 100 , and -10 ≤ r ≤ 20
            if circle.x < -100. || circle.x > 100. || circle.y < -100. || circle.y > 100. || circle.r < -10. || circle.r > 20.{
                assert!(false);
            }
        }
    }
}