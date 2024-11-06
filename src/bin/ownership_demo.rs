struct Book{
  page:i32,
  rating:i32,
}
fn print_page(book: &Book){
  println!("page of book is: {:?}",book.page);
}
fn print_rating(book:&Book){
  println!("rating of book is {:?}",book.rating);
}
fn main(){
  let book = Book{
    page:32,
    rating:33,
  };
  print_page(&book);
  print_rating(&book);
}