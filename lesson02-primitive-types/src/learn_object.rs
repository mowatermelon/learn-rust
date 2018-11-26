fn main() {

	println!("Hello watermelon");//Hello watermelon
	
	let rand_tuple = ("melon", 18);

	let rand_tuple_2: (&str, i8)  = ("water", 18);
	println!("Name : {}",rand_tuple_2.0 );//Name : water
	
}


// rustc learn_object.rs -A warnings &&  ./learn_object.exe