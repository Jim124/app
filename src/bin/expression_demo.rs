enum Access{
  Admin,
  User,
  Guest
}
fn main(){
  let access_level = Access::Guest;
  let can_access_level = match access_level{
    Access::Admin =>true,
    _ => false,
  };
  println!("can access file? {:?}",can_access_level);
}