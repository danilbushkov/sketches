


fn main() {


    let m: u32 = 1 << 31;

    let a: u32 = 0b1_10000001_11111111111111111111111;

    println!("{:}", binary_to_f64(a));

    

}

fn binary_to_f64(binary_number: u32) -> f64 {
    let sign = sign(binary_number);
    let exponent = exponent(binary_number);
    let fraction = fraction(binary_number);

    sign * fraction * 2.0_f64.powi(exponent)
}



fn sign(binary_number: u32) -> f64 {
    let bitmask: u32 = 1 << 31;
    match binary_number & bitmask {
        0 => 1.0,
        _ => -1.0,
    }
}

fn exponent(binary_number: u32) -> i32 {
    let bitmask: u32 = 1 << 31;
    let exponent = (binary_number & !bitmask) >> 23;

    (exponent as i32) - 128
}

fn fraction(binary_number: u32) -> f64 {
    let bitmask: u32 = 0b1_11111111 << 23;
    let number = (binary_number & !bitmask) as f64;
    let fraction: f64 = number / 2.0_f64.powi(23);

    fraction
}
