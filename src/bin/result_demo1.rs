
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}
fn get_choice(input: &str) ->Result<MenuChoice,String>{
  match input{
    "mainmenu" =>Ok(MenuChoice::MainMenu),
    "start" =>Ok(MenuChoice::Start),
    "quit" => Ok(MenuChoice::Quit),
    _ => Err("menu choice not found".to_owned()),
  }
}
fn print_choice(choice: &MenuChoice){
  println!("choice value = {:?}",choice);
}
fn pick_choice(input:&str) ->Result<(),String>{
  let choice:MenuChoice = get_choice(input)?;
  print_choice(&choice);
  Ok(())
}
fn main(){
  let choice = pick_choice("end");
  println!("choice value = {:?}",choice);
  //let choice = get_choice("end");
  // match choice{
  //   Ok(choice) =>print_choice(&choice),
  //   Err(e) => println!("err is {:?}",e),
  // }
  //print_choice(choice);
  // println!("choice = {:?}",choice);
}