use std::io::stdin;
pub fn get_input()->String{
   let mut user_input=String::new();
   let res=stdin().read_line(&mut user_input);
   match res{
      Ok(_)=>(),
      Err(error)=>println!("{}",error),
   }
   user_input= String::from(user_input.trim());
   user_input.push(' ');
   return user_input;
}