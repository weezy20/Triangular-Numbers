trait Triangular {
    fn is_triangular(self) -> bool;
}
impl Triangular for u32 {
    fn is_triangular(self) -> bool {
        for i in 1..=self {
            if triangular(i) == self {
                return true;
            }
        }
        false
    }
}
use std::collections::HashMap;
// let triangular_numbers : HashMap<u32,u32> = HashMap::new();
fn main() {
    // compute the number of triangle numbers
    // between squares
    let mut seq = String::new();
    let mut i: u32 = 0;
    let range = 100__u32;
    loop {
        if i == range {
            break;
        } else {
            i += 1;
        }
        let _x = find_squares(triangular(i), triangular(i + 1));
        let _y = find_triangular(i.pow(2), (i + 1).pow(2));
        seq.push_str(&_y.to_string());
    }
    println!("This is the calc'd sequences \n {}", seq);
}

fn find_triangular(a: u32, b: u32) -> u32 {
    let mut count = 0;
    for i in a..=b {
        if i.is_triangular() == true {
            count += 1;
        }
    }
    count
}
fn find_squares(a: u32, b: u32) -> u32 {
    let mut count: u32 = 0;
    for i in a..=b {
        let i = i as f64;
        if i.sqrt().fract() < 1e-10 {
            count += 1;
        }
    }
    count
}

fn triangular(a: u32) -> u32 {
    // return 1+2+..a
    (a * (a + 1)) / 2
}
