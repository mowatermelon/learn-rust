use std::io::stdin;

fn main() {

	'outer: loop {
		let number :i32 =10;

	    println!("Guess the number!");
	    
		loop {
			let mut line = String::new();
			let input = stdin().read_line(&mut line);

			let guess: Option<i32> = input.ok().map_or(None,|_| line.trim().parse().ok());
			match guess {
				None => println!("Please input your guess."),
				Some(x) if x == number => {
					 println!("right!!!");
					 break 'outer;
				}
				Some(x) if x < number => {
					 println!("Too Low");
				}
				Some(x) if x > number => {
					 println!("Too High");
				}
				Some(_) => println!("Error!!!")
			}
		}
		
	}
}


// rustc learn_input.rs -A warnings &&  ./learn_input.exe

