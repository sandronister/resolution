use std::io;

fn convert_to_int(data_input:& String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
     x
}

fn gdc(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } else {
        return gdc(b, a % b);
    }
}

fn main() {
   let mut first_input = String::new();
    let mut second_input = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut first_input).expect("Failed to read line");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut second_input).expect("Failed to read line");

    let num1 = convert_to_int(&first_input);
    let num2 = convert_to_int(&second_input);

    let result = gdc(num1, num2);

    println!("The greatest common divisor of {} and {} is {}", num1, num2, result);

   
}
