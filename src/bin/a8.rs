enum Flavors{
  Sparking,
  Sweet,
  Fruity
}
struct Drink{
  flavor:Flavors,
  fluid_oz:f64,
}
fn print_drink(drink:Drink){
  match drink.flavor{
    Flavors::Sparking => println!("sparking"),
    Flavors::Fruity =>println!("fruity"),
    Flavors::Sweet =>println!("sweet"),
  }
  println!("oz:{:?}",drink.fluid_oz)
}
fn main(){
  let drink = Drink{
    flavor:Flavors::Fruity,
    fluid_oz:34.23
  };
  print_drink(drink);
}