use std::io;

fn convert_to_int(data_input:& String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
     x
}
fn main() {
   let mut medias_str = String::new();
   io::stdin().read_line(&mut medias_str).expect("Failed to read line");

   let mut sum_rec_i32 = 0;
   let mut i_32 = 0;

   while convert_to_int(&medias_str) >i_32 {
        let mut medias_str = String::new();
        io::stdin().read_line(&mut medias_str).expect("Failed to read line");
        if convert_to_int(&medias_str)>=3 && convert_to_int(&medias_str)<6{
            sum_rec_i32 +=1
            
        }
        i_32 += 1;
   }


    println!("Numero de alunos em recuperação {}", sum_rec_i32);

   
}
