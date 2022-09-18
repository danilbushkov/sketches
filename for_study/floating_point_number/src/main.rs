use floating_point_number::f64_to_b32;
use floating_point_number::b32_to_f64;
use std::env;
use std::io;

fn main() {


    // let m: u32 = 1 << 31;

    // let a: u32 = 0b1_01111111_11111111111111111111111;

    // println!("{:}", b32_to_f64(a));
    // println!("{:b}", f64_to_b32(b32_to_f64(a)));
    let args: Vec<String> = env::args().collect();

    //b32 or f64
    let mut arg = &"".to_string();
    if args.len() > 1 {
        arg = &args[1];
    }
    

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        panic!("Error: {:?}", e);
    }
    input.retain(|c| c != '\n');

    if arg == "b32" {
        match input.parse::<f64>() {
            Ok(n) => {
                println!("{:b}", f64_to_b32(n));
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    } else {
        match u32::from_str_radix(&input, 2) {
            Ok(n) => {
                println!("{}", b32_to_f64(n));
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
    //println!("Searching for {}", query);
    //println!("In file {}", file_path);

}

