// #![allow(warnings)]

struct Node {
   name: String, 
   question: String,
   yesNode: Option<Box<Node>>,
   noNode: Option<Box<Node>>,
}

impl Node {
    pub fn askQuestion(&mut self) {
         self.interpretQuality(prompt(self.question));
    }

    pub fn interpretQuality(&mut self, response: &str) {
        if response.starts_with("Y") {
           verifyName(prompt("IS IT A NAME"))
          println!("END!");
        } else if response.starts_with("N") {
           match self.noNode {
              None => {
                 self.noNode = self.createNode();
                 println!("END!");
               },
              Some(node) => node.askQuestion(),
           }
        } else {
           self.askQuestion();
        }
    }

    pub fn createNode() -> Node {
      let newName = prompt("THE ANIMAL YOU WERE THINKING OF WAS A ");
      let newQ = prompt("PLEASE TYPE IN A QUESTION THAT WOULD DISTINGUISH A ___ FROM A ____ \n ?");
      
      Node{
         name: newName,
         question: newQ,
         yesNode: None,
         noNode: None,
      }
    }
}

// Prompt pass through here
fn prompt(prompt_text: &str) -> &str {
   println!("{}?", prompt_text);
   let mut line = String::new();
   std::io::stdin().read_line(&mut line).unwrap();
   
   return &line.trim().to_uppercase();
}

fn start(rootNode: &mut Node) {
   let response = prompt("ARE YOU THINKING OF AN ANIMAL(Y/N)?");
   if response.starts_with("Y") {
      rootNode.askQuestion();
   } else if response == "LIST" {
      reportAnimalsList(&rootNode);
      start(&mut rootNode);
   } else {
      start(&mut rootNode);
   }
}

fn reportAnimalsList(rootNode: &Node) {
   // TODO: report list of animals known. Need to traverse the tree
   println!("{:?}", "animalsList");
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
   // let mut xs = vec![1i32, 2, 3];
   let mut rootNode = Node {
      name: String::from("ROOT"),
      question: String::from("DOES IT SWIM"), 
      yesNode: None, 
      noNode: None
   };
   let mut fishNode = Node {
      name: String::from("FISH"),
      question: String::from("DOES IT FLY"),
      yesNode: None,
      noNode: None
   };
   let mut birdNode = Node {
      name: String::from("BIRD"),
      question: None,
      yesNode: None,
      noNode: None
   };
   rootNode.yesNode = &mut fishNode;
   fishNode.yesNode = &mut birdNode;
   loop {
      start(&mut rootNode);
      println!("WHY NOT TRY ANOTHER ANIMAL?'\n");
   }


}
