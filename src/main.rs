use std::io;

fn convet_to_int(data_input:& String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
     x
}
fn main() {
  let mut number1 = String::new();
  io::stdin().read_line(&mut number1).expect("Erro ao ler number1");
  let mut number2: String = String::new();
  io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

  if convet_to_int(&number1) > convet_to_int(&number2){
    println!("O numero {} eh maior que {}",number1,number2);
  }else{
    println!("O numero {} eh menor ou igual que {}",number1,number2)
  }
}
