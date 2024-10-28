fn main() {
    println!("Hello, world!");
    test_func();
}

fn test_func(){
   let x:f64= 255.0;
   let y:u8= x as u8 -5;
   println!("{}",y);
   let mut  iamold = true;
   println!("{}",iamold);
   iamold = false;
    println!("{}",iamold);
    let my_str = 'A';
    println!("{}",my_str);
    let mut first_name = "Jim";
    println!("{}",first_name);
    first_name = "Hong-fei";
    println!("{}",first_name);
    let tuples = ("hong","jim",40);
    println!("{:?}",tuples);
    let ages:[u8;3] = [12,34,56];
    println!("{:?}",ages);
    let slices = &ages[0..=2];
    println!("{:?}",slices);
}