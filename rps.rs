use std::io;
fn main() 
{

//srand(static_cast<unsigned int>(time(NULL)));

let mut n=String::new();
let mut m=String::new();
let mut user1=String::new();
let mut user2=String::new();
println!("Welcome Player 1!! What is your name?");
io::stdin().read_line(&mut user1).expect("No input given");
println!("1:rock\n2:paper\n3:scissors");
println!("Hello {}! What do you choose?",user1);
io::stdin().read_line(&mut n).expect("No input given");
let user_choice1 = n.trim()
                .parse::<u32>()
                .unwrap();

//println!("{}",user_choice1);
/*while user_choice1 != "rock" && user_choice1 != "paper" && user_choice1 != "scissors"
{
			println!("Sorry! Didn't get that. Please enter again.");
			io::stdin().read_line(&mut user_choice1).expect("No input given");

}*/
println!("Welcome Player 2!! What is your name?");
io::stdin().read_line(&mut user2).expect("No input given");
println!("1:rock\n2:paper\n3:scissors");
println!("Hello {}! What do you choose?",user2);
io::stdin().read_line(&mut m).expect("No input given");
let user_choice2 = m.trim()
                .parse::<u32>()
                .unwrap();

//for (int i=0;i<userChoice.length();i++){
  //  userChoice[i]=tolower(userChoice[i]);
//}

/*while userChoice1 != "rock" && userChoice1 != "paper" && userChoice1 != "scissors" || userChoice2 != "rock" && userChoice2 != "paper" && userChoice2 != "scissors") {
    println!("Sorry! Didn't get that. Please enter again.");
    getline(cin, userChoice);
    for (int i=0;i<userChoice.length();i++){
        userChoice[i]=tolower(userChoice[i]);
    }
}*/
/*while user_choice2 != "rock" && user_choice2 != "paper" && user_choice2 != "scissors"
	{
			println!("Sorry! Didn't get that. Please enter again.");
			io::stdin().read_line(&mut user_choice2).expect("No input given");

	}*/



//println!("Press enter to continue...");
//std::cin.ignore();

if user_choice1 == 1 && user_choice2 == 1 
{
    println!( "It was a tie!" );
} 
else if user_choice1 == 1 && user_choice2 == 3 {
    println!("The {} won! Better luck next time!",user1);
}
 else if user_choice1 == 2 && user_choice2 == 2 {
    println!("It was a tie!");
}
 else if user_choice1 == 2 && user_choice2 == 1 {
    println!("The {} won! Better luck next time!",user1);
}
else if user_choice1 == 3 && user_choice2 == 3 {
    println!( "It was a tie!");
}
 else if user_choice1 == 3 && user_choice2 == 2 {
    println!("The {} won! Better luck next time",user1);
} 
else {
    println!("Congrats! You won {}!",user2);
}
}


