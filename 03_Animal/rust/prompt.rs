#![allow(warnings)]

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
      doesItSwim();
   } else if "LIST" == response {
      reportAnimalsList();
   } else {
      start();
   }
}

fn reportAnimalsList() {
   // report list of animals known
   let animalsList = ["BIRD", "FISH"];
   println!("{:?}", animalsList);
   start()
}

fn doesItSwim() {
   let response = prompt("DOES IT SWIM(Y/N)?");
   if "Y" == response {
      println!("This animal swims.");
   } else {
      println!("this is not a why");
   }
}

fn main(){
   println!("PLAY 'GUESS THE ANIMAL'\n");

   println!("THINK OF AN ANIMAL AND THE COMPUTER WILL TRY TO GUESS IT.\n");
   let mut xs = vec![1i32, 2, 3];
   start();


}
