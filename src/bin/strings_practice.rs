struct Person {
  age: i32,
  name:String,
  color:String,
}

fn print_string(data:&str){
  println!("{data:?}");
}
fn main(){
  let people = vec![Person{
    age:12,
    name:"Jim".to_owned(),
    color:"blue".to_owned(),
  },Person{
    age:10,
    name:String::from("daniel"),
    color:String::from("yellow"),
  },Person{
    age:9,
    name:String::from("thiya"),
    color:String::from("red"),
  }];
  for person in people{
    if person.age <=10{
      print_string(&person.color);
      print_string(&person.name);
    }
  }
}