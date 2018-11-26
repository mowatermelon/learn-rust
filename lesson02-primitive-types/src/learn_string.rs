fn main() {
	println!("Hello watermelon");//Hello watermelon

	let rand_string = " hello watermelon";

	println!("Length : {}", rand_string.len());//Length : 17

	let (first,second) = rand_string.split_at(6);
	println!("First : {},Second：{}", first, second);//First :  hello,Second ：  watermelon

	let count = rand_string.chars().count();
	let mut chars = rand_string.chars();

	let mut indiv_char = chars.next();

	loop {
		match indiv_char {
			Some(x) => println!("{}", x),
			None => break,
		}
		indiv_char = chars.next();
	}


	// h
	// e
	// l
	// l
	// o

	// w
	// a
	// t
	// e
	// r
	// m
	// e
	// l
	// o
	// n


	let mut iter = rand_string.split_whitespace();

	let mut indiv_word = iter.next();

	loop {
		match indiv_word {
			Some(x) => println!("{}", x),
			None => break,
		}
		indiv_word = iter.next();
	}

	// hello
	// watermelon

	let rand_string2 = "balabalabala\nwelcome to there\nI am watermelon";
	let mut lines = rand_string2.lines();
	let mut indiv_line = lines.next();

	loop {
		match indiv_line {
			Some(x) => println!("{}", x),
			None => break,
		}
		indiv_line = lines.next();
	}

	// balabalabala
	// welcome to there
	// I am watermelon

	println!("melon is contain : {}", rand_string2.contains("melon"));//melon is contain : true

}	

// rustc learn_string.rs -A warnings &&  ./learn_string.exe

