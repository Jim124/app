struct GroceryItem{
  quantity:i32,
  id:i32,
}
fn print_quantity(grocery_item:&GroceryItem){
  println!("quantity is {:?}",grocery_item.quantity)
}
fn print_id(grocery_item:&GroceryItem){
  println!("id is {:?}",grocery_item.id)
}
fn main(){
  let grocery_item = GroceryItem{
    quantity:4,
    id:12,
  };
  // borrowing
  print_quantity(&grocery_item);
  print_id(&grocery_item);
}