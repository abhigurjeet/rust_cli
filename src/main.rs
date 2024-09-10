mod args;
mod input;

fn main() {
   let user_input=input::get_input();
   let args=args::get_args(&user_input);
   println!("{:?}",args);
}