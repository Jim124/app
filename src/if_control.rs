pub fn if_control_flow(a:i32) {
  if a >200 {
    println!("{a:?}");
  } else if a > 99{
    println!("big number");
  } else {
    println!("small number");
  }
}