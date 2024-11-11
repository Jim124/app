// Topic Advanced match
enum Ticket{
  Backstage(String,i32),
  Vip(i32),
  Standard(String,i32)
}
fn main(){
  let tickets = vec![Ticket::Backstage("Jim".to_owned(),45),Ticket::Vip(60),Ticket::Standard("Daniel".to_owned(),30)];
  for ticket in tickets{
    match ticket{
      Ticket::Backstage(back,price) =>{println!("backstage ticket holder :{:?}, price = {:?}",back,price)}
      Ticket::Standard(std,price) =>println!("standard ticket Holder: {:?}, price = {:?}",std,price),
      Ticket::Vip(price) =>{println!("Vip ticket, price = {:?}",price)}
    }
  }
}