pub fn is_armstrong_number(num: u32) -> bool {
    let n_string = num.to_string();
    let num_chars = n_string.len() as u32;

    n_string
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x.pow(num_chars)))
        .sum::<u32>()
        == num
}

pub fn main() {
    println!("{}", is_armstrong_number(9));
}
