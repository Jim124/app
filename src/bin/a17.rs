#[derive(Debug)]
struct Adult{
  name:String,
  age:i32,
}
impl Adult{
  fn new_adult(name:String,age:i32) ->Result<Self,&str>{
    if age <=21 {
        Err("You are too young")
    } else{
      Ok(Self { name, age })
    }
  }
}
fn new(name:String,age:i32) ->Result<Adult,String>{
  if age >= 21{
    Ok(Adult { name,age})
  } else{
    Err("You are too young".to_owned())
  }
}

fn main(){
  let adult1 = new("Jim".to_owned(), 22);
  let adult2 = new("Daniel".to_owned(), 18);
  let mut vec = Vec::new();
  vec.push(adult1);
  vec.push(adult2);
  for adult in vec{
    match adult {
      Ok(a) =>println!("name is{:?}, you are {:?} years old.",a.name,a.age),
      Err(e) => println!("{:?}",e),
    }
  }
}
