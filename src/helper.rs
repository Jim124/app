pub fn get_full_name(first: &str,last:&str ) ->String{
  let full_name = format!("{0} {1}",first,last);
  return full_name;
}

pub mod test_func{

  pub fn get_age(age: u16) -> u16 {
    let age = age +5;
    return age;
  }
}