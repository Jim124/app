struct Adult{
  name:String,
  age:u8,
}
impl Adult{
  fn new(name:&str,age:u8) ->Result<Self,&str>{
    if age <21{
      Err("age must be at least 21")
    } else {
      Ok(Self{
        name:name.to_string(),age,
      })
    }
  }
}
fn main(){
  let child = Adult::new("jim", 18);
  match child{
    Ok(child) => println!("{:?} is {:?} years old.",child.name,child.age),
    Err(e) =>println!("{e}"),
  }
}