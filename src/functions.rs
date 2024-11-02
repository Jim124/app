pub fn human_id(){
  let x =32;
  let y = 3.4;
  println!("{}",x);
  println!("y is {}",y);
}

pub fn human(name:&str,age:u32,height:f32){
  println!("My name is {}, I am {} years old, and my height is {}",name,age,height);
}

pub fn add(a:i32,b: i32) -> i32{
  a +b
}
pub fn calculate_height(weight:f64,height:f64) ->f64{
  weight / height
}