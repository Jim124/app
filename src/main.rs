pub mod helper;
pub mod functions;
pub mod ownership;
fn main() {
    println!("Hello, world!");
    let full_name = helper::get_full_name("HongFei", "Du");
    println!("hello, from {0}",full_name);
    let age = helper::test_func::get_age(32);
    println!("My age is {0}",age);
    // test_func();
    // let x  = 5;
    // let _y :i32;
    // assert_eq!(x,5);
    // println!("success");
    let x  = 10;
    {
        let y = 5i32;
        println!("The value of x is {} and value of y is {}",x,y);
    }
    let x :i32 = 42;
    println!("x is {}",x);
    let v:i32  = {
        let mut x :i32 = 1;
        x = x +2;
        x
    };
    assert_eq!(v,3);
    println!("Success");
    let mut stone_cold:String = String::from("Hell,");
    stone_cold.push_str("Yeah");
    println!("Stone Cold Says:{}",stone_cold);
    let str :&str = "hello";
    println!("{}",str);
    let string :String = String::from("Hello, world");
    let slice: &str = &string;
    let slice1 :&str = &string[1..3];
    println!("Slice value:{}",slice);
    println!("Slice1 value:{}",slice1);
    functions::human_id();
    functions::human(&String::from("Jim"), 32, 132.5);
    println!("Value from function 'add' is : {}",functions::add(1,3));
    let weight = 230.0;
    let height = 120.0;
    let BMI = functions::calculate_height(weight, height);
    println!("BMI is {:.2}",BMI);
    let s1 = String::from("Rust");
    let size = ownership::calculate_add(&s1);
    println!("The len of {} is {}",s1,size);
    let s2 = String::from("Rust");
    let s3 = s2;
    // println!("s2 is {}",s2); error
    println!("s3 is {}",s3);

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