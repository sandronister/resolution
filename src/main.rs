use std::io;

fn convet_to_int(data_input:& String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
     x
}
fn main() {
  let mut result =1;
  let mut input_value = String::new();
  io::stdin().read_line(&mut input_value).expect("Erro ao ler o input de entrada");
  let mut int_value=convet_to_int(&input_value);

  while int_value!=1{
    result*=int_value;
    int_value-=1;
  }
  println!("O valor somando é igual {}",result)
}
