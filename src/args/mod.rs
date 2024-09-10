pub fn get_args(user_input:&String)->Vec<String>{
   let mut args:Vec<String>=Vec::new();
   let mut word=String::new();
   for i in user_input.chars(){
      if i==' '{
         if word.len()>0{
            args.push(word);
            word=String::new();
         }
      }
      else{
         word.push(i);
      }
   }
   return args;
}