fn main() {
	println!("Hello watermelon");//Hello watermelon

	// learn print skill
	println!("5 + 4 = {}",5 + 4 );//5 + 4 = 9
	println!("5 - 4 = {}",5 - 4 );//5 - 4 = 1
	println!("5 * 4 = {}",5 * 4 );//5 * 4 = 20
	println!("5 / 4 = {}",5 / 4 );//5 / 4 = 1
	println!("5 % 4 = {}",5 % 4 );//5 % 4 = 1

	let mut neg_4 = -4i32;
 
 	println!("abs(-4) = {}",neg_4.abs());//abs(-4) = 4
	println!("4 ^ 6 = {}",4i32.pow(6));//4 ^ 6 = 4096
	println!("sqrt 9 = {}",9f64.sqrt());//sqrt 9 = 3
	println!("Cbrt 9 = {}",9f64.cbrt());//Cbrt 9 = 2.080083823051904
	println!("Round 1.45 = {}",1.45f64.round());//Round 1.45 = 1
	println!("Floor 1.45 = {}",1.45f64.floor());//Floor 1.45 = 1
	println!("Ceiling 1.45 = {}",1.45f64.ceil());//Ceiling 1.45 = 2
	println!("e ^ 2 = {}",2f64.exp());//e ^ 2 = 7.38905609893065
	println!("log(2) = {}",2f64.ln());//log(2) = 0.6931471805599453
	println!("log10(2) = {}",2f64.log10());//log10(2) = 0.3010299956639812
	println!("90 to Radians = {}",90f64.to_radians());//90 to Radians = 1.5707963267948966
	println!("PI to Degress = {}",3.14f64.to_degrees());//PI to Degress = 179.9087476710785
	println!("Max 4,5 = {}",4f64.max(5f64));//Max 4,5 = 5
	println!("Min 4,5 = {}",4f64.min(5f64));//Min 4,5 = 4 

	println!("Sin 3.14 = {}",3.14f64.sin());//Sin 3.14 = 0.0015926529164868282
	println!("Cos 3.14 = {}",3.14f64.cos());//Cos 3.14 = -0.9999987317275395
	println!("Tan 3.14 = {}",3.14f64.tan());//Tan 3.14 = -0.001592654936407223


	// learn && || !
	let age_old = 5;

	if(age_old == 5){
		println!("Go to Kindergarten");
	}else if (age_old > 5)&&(age_old <= 18){
		println!("Go to grade {}",(age_old - 5));
	}else if (age_old > 18)&&(age_old <= 25){
		println!("Go to College");
	}else{
		println!("Do what you want");
	}

	//Go to Kindergarten

	println!("!true = {}",!true);
	println!("true || false = {}",true||false);
	println!("true != false : {}",(true != false));

	let can_vote = if (age_old >= 18) {true} else {false};

	println!("Can Vote : {}",can_vote);//Can Vote : false 


	//learn loop
	let mut x = 1;

	loop{
		if((x%2)== 0){
			println!("{}",x);
			x += 1;

			continue;
		}
		if(x> 10){
			break;
		}

		x += 1;
		continue;
	}
	// 2
	// 4
	// 6
	// 8
	// 10

	println!("use y mode");
	let mut y = 1;

	while y <= 10{
		println!("{}", y);
		y += 1;
	}

	println!("use z mode");
	for z in 1..10 {
		println!("For {}",z);
	}
	// For 1
	// For 2
	// For 3
	// For 4
	// For 5
	// For 6
	// For 7
	// For 8
	// For 9

}

// rustc learn_math.rs -A warnings &&  ./learn_math.exe
