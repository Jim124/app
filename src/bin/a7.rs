enum  Color {
  Red,
  Blue,
  Green,
  Yellow,
  White,
  Black
}
fn println_color(cl :Color){
  match  cl {
    Color::Black =>println!("color is black"),
    Color::Blue =>println!("color is blue"),
    Color::Green =>println!("color is green"),
    Color::Red =>println!("color is red"),
    Color::White =>println!("color is white"),
    Color::Yellow =>println!("color is yellow"),
  }
}
fn main(){
 let cl = Color::Red;
 
 println_color(cl);
}