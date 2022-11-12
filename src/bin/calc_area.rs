use std::f64::consts::PI;

fn main() {
    println!("{}", calc_area(5.0));
}

pub fn calc_area(r: f64) -> f64 {
    if r <= 0.0 { panic!("中１数学からやり直せ"); }

    r.powi(2) * PI - (r * 2.0).powi(2) / 2.0
}