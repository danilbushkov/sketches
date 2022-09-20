use floating_point_number::f64_to_b32;
use floating_point_number::b32_to_f64;
use std::env;
use std::io;

const ACCURACY: usize = 8;


fn main() {


    
    let args: Vec<String> = env::args().collect();

    
    let mut arg1 = &"".to_string();
    let mut arg2 = &"".to_string();
    if args.len() > 1 {
        arg1 = &args[1];
    }
    if args.len() > 2 {
        arg2 = &args[2];
    }

    if arg2 == "two" {
        two_nums(arg1);
    } else {
        one_num(arg1);
    }
    

}

fn two_nums(arg: &String) {
    let mut num1 = String::new();
    let mut num2 = String::new();

    if let Err(e) = io::stdin().read_line(&mut num1) {
        panic!("Error: {:?}", e);
    }
    if let Err(e) = io::stdin().read_line(&mut num2) {
        panic!("Error: {:?}", e);
    }
    num1.retain(|c| c != '\n');
    num2.retain(|c| c != '\n');

    if arg == "f64" {
        match num1.parse::<f64>() {
            Ok(n1) => {
                match num2.parse::<f64>() {
                    Ok(n2) => {
                        println!("A:");
                        println!("{}", n1);
                        println!("{:032b}", f64_to_b32(n1));
                        println!("B:");
                        println!("{}", n2);
                        println!("{:032b}", f64_to_b32(n2));
                        println!("Result:");
                        println!("{}", f_fmt(n1/n2, ACCURACY));
                        println!("{}", n1/n2);
                        println!("{:032b}", f64_to_b32(n1/n2));
                        println!("---------------------------------------------");
                    },
                    Err(e) => {
                        panic!("Error: {:?}", e);
                    } 
                }
                
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    } else {
        match u32::from_str_radix(&num1, 2) {
            Ok(n1) => {
                match u32::from_str_radix(&num2, 2) {
                    Ok(n2) => {
                        println!("A:");
                        println!("{:032b}", n1);
                        println!("{}", b32_to_f64(n1));
                        println!("B:");
                        println!("{:032b}", n2);
                        println!("{}", b32_to_f64(n2));
                        println!("Result:");
                        let result = b32_to_f64(n1)/b32_to_f64(n2);
                        println!("{}", f_fmt(result, ACCURACY));
                        println!("{}", result);
                        println!("{:032b}", f64_to_b32(result));
                        println!("---------------------------------------------");
                    },
                    Err(e) => {
                        panic!("Error: {:?}", e);
                    }
                }
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}




fn one_num(arg: &String) {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        panic!("Error: {:?}", e);
    }
    input.retain(|c| c != '\n');

    if arg == "f64" {
        match input.parse::<f64>() {
            Ok(n) => {
                println!("{}", f_fmt(n, ACCURACY));
                println!("{}", n);
                println!("{:032b}", f64_to_b32(n));
                println!("---------------------------------------------");
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    } else {
        match u32::from_str_radix(&input, 2) {
            Ok(n) => {
                println!("{:032b}", n);
                println!("{}", f_fmt(b32_to_f64(n), ACCURACY));
                println!("{}", b32_to_f64(n));
                println!("---------------------------------------------");
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}


fn f_fmt(num: f64, accuracy: usize) -> String {
    let mut num = num;
    let mut exp: isize = 0;
    if num.abs() >= 10.0 {
        num = num/10.0;
        exp += 1;
    } else if num.abs() < 0.0 {
        num = num * 10.0;
        exp += 1;
    }

    format!("{:.*}e{}", accuracy, num, exp)
}

