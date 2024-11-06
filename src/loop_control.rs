pub fn loop_control(){
  let mut a = 0;
  loop{
    if a == 5 {
      break;
    }
    println!("{:?}",a);
    a += 1;

  }
}
pub fn while_control(){
  let mut a = 0;
  while a != 5{
    println!("{:?}",a);
    a += 1;
  }
}