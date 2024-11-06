pub fn match_control(){
  let some_bool = true;
  match some_bool{
    true =>println!("Hello"),
    false => println!("bye"),
  }
  let num = 4;
  match num {
    1 =>println!("1--"),
    2 =>println!("2--"),
    3 => println!("3--"),
    _ =>println!("Something else"),
  }
}

pub fn match_demo(){
  let decision = true;
  match decision{
    true =>println!("it's true"),
    false =>println!("it's false"),
  }
}