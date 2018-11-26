fn main() {
	println!("Hello watermelon");//Hello watermelon

	// learn primitive
	let num = 10;//the mode can not change value
	println!("the num is {}",num);//the num is 10
    let mut temp: i8 = 10;//the mode can change value
    temp = 18;
	println!("the temp is {}",temp);//the temp is 18

	let is_it_true: bool= true;//the boolean mode
	println!("Is it true ? {}",is_it_true);//Is it true ? true

    let let_x: char = 'x';//the char mode
	println!("the temp char is {}",let_x);//the temp char is x
	
	let (a_name,b_name) =("temp_a","temp_b");//Declare multiple variables at once
	println!("the name of a is {},the name of b is {}",a_name,b_name);//the name of a is temp_a,the name of b is temp_b

	// learn print skill

	//Specifies the module index value you need to print
	println!("It is {0} that {1} is {0}", is_it_true,let_x);//It is true that x is true

	//Specify the type of print

		//Specifies the print decimal
		println!("{:.2}", 1.234);//1.23

		//Specify the printing system
		println!("B :{:b} ,H :{:x},O :{:o}", 10, 10, 10);//B :1010 ,H :a,O :12


	//Print variable statement results
	println!("{ten:>ws$}", ten =10, ws=5);//   10
	//You can specify a number for the complement
	println!("{ten:>0ws$}", ten =10, ws=5);//00010  

}

// rustc learn_base.rs -A warnings &&  ./learn_base.exe
