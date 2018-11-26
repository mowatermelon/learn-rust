fn main() {

	say_hello("melon");//Hello melon

	println!("5 + 4 = {}", get_sum(5, 4));//5 + 4 = 9
	
	let sum = get_sum;

	println!("6 + 4 = {}", get_sum(6, 4));//6 + 4 = 10

	let sum_nums = |x:i32,y:i32| x + y;

	println!("7 + 8 = {}",sum_nums(7,8) );//7 + 8 = 15

	let num_ten = 10;
	let add_10 = |x:i32| x + num_ten;

	println!("7 + 10 = {}",add_10(7) );//7 + 10 = 17
}


fn say_hello(name :&str){
	println!("Hello {}", name);
}

fn get_sum(num1: i32,num2: i32) -> i32{
	num1 + num2
}

// rustc learn_function.rs -A warnings &&  ./learn_function.exe