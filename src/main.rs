use std::io::stdin;
fn get_args(user_input:&String)->Vec<String>{
   println!("{}",user_input);
   return Vec::new();
}
fn get_input()->String{
   let mut user_input=String::new();
   let res=stdin().read_line(&mut user_input);
   match res{
      Ok(_)=>(),
      Err(error)=>println!("{}",error),
   }
   return String::from(user_input.trim());
}
fn main() {
   let user_input=get_input();
   let args=get_args(&user_input);
   println!("{:?}",args);
}