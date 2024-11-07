#[allow(dead_code)]
enum Color{
  Red,
  Blue
}
impl Color{
  fn print_color(&self){
    match self{
      Color::Blue =>println!("color is blue"),
      Color::Red => println!("color is red"),
    }
  }
}
struct Dimension{
  height:i32,
  width:i32,
  deep:i32,
}
impl Dimension{
  fn print_dimension(&self){
    println!("the height is {:?}, width is {:?}, deep is {:?}.",self.height,self.width,self.deep);
  }
}
struct ShippingBox{
  color:Color,
  dimensions:Dimension,
  weight:f64,
}
impl ShippingBox{
  fn new(color:Color,dimension:Dimension,weight:f64) ->Self{
    Self { color, dimensions: dimension, weight }
  }
  fn print(&self){
    self.color.print_color();
    self.dimensions.print_dimension();
    println!("The weight is {:?}",self.weight);
  }
}
fn main(){
  let color = Color::Blue;
  let dimensions = Dimension {
    height:100,
    width:100,
    deep:20
  };
  let shipping_box = ShippingBox::new(color, dimensions, 22.0);
  shipping_box.print();
}