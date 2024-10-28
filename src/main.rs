pub mod helper;
fn main() {
    println!("Hello, world!");
    let full_name = helper::get_full_name("HongFei", "Du");
    println!("hello, from {0}",full_name);
    let age = helper::test_func::get_age(32);
    println!("My age is {0}",age);
    // test_func();
}

#[allow(dead_code)]
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