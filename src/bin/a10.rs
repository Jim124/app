fn print_message(num:i32){
  let my_bool = if num > 100 {true} else {false};
  match my_bool{
    true =>println!("its big"),
    false =>println!("its small"),
  }
}
fn main(){
  let num = 110;
  print_message(num);
}