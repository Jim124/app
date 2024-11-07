struct Test{
  score: i32,
}
fn main(){
  let scores = vec![
    Test{score:55},Test{score:66},Test{score:77},Test{score:88}];
  for test in scores{
    println!("score is {:?}",test.score);
  }
}