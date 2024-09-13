use std::io;


fn convet_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}


fn main() {
   let mut input_str = String::new();
    io::stdin().read_line(&input_str).expect("Failed to read line");


    let input_int = convet_to_int(&input_str);
    let result = 1;
    for i in 1..input_int+1 {
        result*=i;
    }

    println!("Factorial of {} is {}", input_int, result);
}
