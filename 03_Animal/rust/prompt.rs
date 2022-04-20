// Prompt pass through here
fn prompt(prompt_text: &str) -> String {
   println!("{}", prompt_text);
   let mut line = String::new();
   std::io::stdin().read_line(&mut line).unwrap();
   return line.trim().to_uppercase();
}

fn start() {
   let response = prompt("ARE YOU THINKING OF AN ANIMAL(Y/N)?");
   if "Y" == response {
      println!("This is a y");
   } else {
      println!("this is not a why");
   }
}


fn main(){
   println!("PLAY 'GUESS THE ANIMAL'\n");

   println!("THINK OF AN ANIMAL AND THE COMPUTER WILL TRY TO GUESS IT.\n");
   start();


}
