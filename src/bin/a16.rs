struct Student{
  name:String,
  locker:Option<i32>,
}
fn main(){
  let st1 = Student{
    name:"Jim".to_owned(),
    locker:Some(23),
  };
  let st2 = Student{
    name:String::from("Thiya"),
    locker:None,
  };
  let mut students = Vec::new();
  students.push(st1);
  students.push(st2);
  for student in students{
    println!("name of student is : {:?}",student.name);
    match student.locker{
      Some(locker) =>println!("student locker assignment is : {:?}",locker),
      None => println!("student has no locker assignment"),
    }
  }
}