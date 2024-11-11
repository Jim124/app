use std::collections::HashMap;
fn main(){
  let mut store = HashMap::new();
  store.insert("chairs", 2);
  store.insert("beds", 5);
  store.insert("table", 3);
  store.insert("couches", 0);
  let mut total = 0;
  for (key,val) in store.iter(){
    total = total + val;
    let item = if val == &0{
      "Out of stock".to_owned()
    } else {
      format!("{:?}",val)
    };
    println!("{:?} has {:?}",key,item);
  }
  println!("{:?}",total);
}