#[derive(Debug,Clone,Copy)]
enum Position {
    Manager,
    Employee,
    Marker
}
#[derive(Debug,Clone,Copy)]
struct Employee{
  position:Position,
  age:i32,
}
// copy a emp not borrowing
fn print(emp:Employee){
  println!("{:?}",emp);
}
fn main(){
  let me = Employee{
    position:Position::Employee,
    age:32,
  };
  println!("{:?}",me);
  print(me);
  print(me);
}