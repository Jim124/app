
fn get_sound(name:&str) ->Result<(),String>{
  if name == "alert"{
    Ok(())
  } else{
    Err("Unable to find sound data".to_owned())
  }
}
fn main(){
  let sound = get_sound("sound data");
  match sound{
    Ok(_) =>println!("sound data located"),
    Err(e) =>println!("error:{:?}",e),
  }
}