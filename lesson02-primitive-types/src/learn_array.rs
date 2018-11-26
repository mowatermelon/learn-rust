fn main() {

	println!("Hello watermelon");//Hello watermelon
	
	let rand_array = [1,3,2,4];

	println!("{}", rand_array[0]);//1

	println!("{}", rand_array.len());//4

	println!("Second 2 : {:?}", &rand_array[1..3]);//Second 2 : [3, 2]


	println!("vect");
	let mut vect1 = vec![1,2,3,4,5,6,7,8,9];

	println!("Item 2 {}", vect1[1]);//Item 2 2

	vect1.push(10);

	for i in &vect1 {
		println!("Vect : {}", i);
	}

	// Vect : 1
	// Vect : 2
	// Vect : 3
	// Vect : 4
	// Vect : 5
	// Vect : 6
	// Vect : 7
	// Vect : 8
	// Vect : 9
	// Vect : 10

	vect1.pop();
	//thread 'main' panicked at 'index out of bounds: the len is 9 but the index is 9'
	// println!("Item 9 {}", vect1[9]);

	let vect2 = vec![1,2,3,4,5,6,7,8,9];
	let vect3 = vect2;
	// println!("vect2[0] : {}", vect2[0]);
	// error[E0382]: use of moved value: `vect2`
	//   --> learn_array.rs:42:28
	//    |
	// 41 |     let vect3 = vect2;
	//    |         ----- value moved here
	// 42 |     println!("vect2[0] : {}", vect2[0]);
	//    |                               ^^^^^ value used here after move
	//    |
	//    = note: move occurs because `vect2` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait

	// error: aborting due to previous error

	// For more information about this error, try `rustc --explain E0382`.

	println!("Sum of Vect : {}", sum_vects(&vect3));//Sum of Vect : 45

	println!("Vect : {:?}", vect3);//Vect : [1, 2, 3, 4, 5, 6, 7, 8, 9]

	let prim_val = 1;
	let prim_val2 = prim_val;
	println!("prim_val : {}", prim_val);

}

fn sum_vects(v1: &Vec<i32>) -> i32 {
	let sum = v1.iter().fold(0, |mut sum ,&x| { sum += x ;sum});
	return sum;
}
// rustc learn_array.rs -A warnings &&  ./learn_array.exe
