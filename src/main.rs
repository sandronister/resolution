
fn double(x: i32) -> i32 {
    x * 2
}

fn major(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
   println!("O Dobro de 32 é {}", double(32));

   println!("O maior entre 32 e 64 é {}", major(32, 64));

   
}
