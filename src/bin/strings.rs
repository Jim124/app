struct LineItem{
  name:String,
  count:i32,
}

fn print_name(name:&str){
  println!("name :{:?}",name);
}
fn main(){
  let lines = vec![LineItem{
    name:"cereal".to_owned(),
    count:2,
  },LineItem{
    name:String::from("fruit"),
    count:3,
  }];
  for line in lines{
    print_name(&line.name);
    println!("count is {:?}",line.count);
  }
}