struct Temperature{
  degrees_f:f64,
}
fn show_temp(temp:Temperature){
  println!("{:?} degrees F",temp.degrees_f);
}

impl Temperature {
    fn freezing() -> Self{
      Self { degrees_f: 32.0 }
    }
    fn boiling() -> Self{
      Self { degrees_f: 202.0 }
    }
    fn show_degree(&self){
      println!("{:?} degrees F ",self.degrees_f);
    }
}

fn main(){
  let temp = Temperature{
    degrees_f: 99.9
  };
  show_temp(temp);
  let hot = Temperature{degrees_f:88.0};
  hot.show_degree();
  let cold = Temperature::freezing();
  cold.show_degree();
  let boil = Temperature::boiling();
  boil.show_degree();
}