use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64};

fn main() {
	println!("Hello watermelon");//Hello watermelon

	println!("Max i8 {}", i8::MAX);
	println!("Max i16 {}", i16::MAX);
	println!("Max i32 {}", i32::MAX);
	println!("Max i64 {}", i64::MAX);
	println!("Min i8 {}", i8::MIN);
	println!("Min i16 {}", i16::MIN);
	println!("Min i32 {}", i32::MIN);
	println!("Min i64 {}", i64::MIN);

	println!("Max u8 {}", u8::MAX);
	println!("Max u16 {}", u16::MAX);
	println!("Max u32 {}", u32::MAX);
	println!("Max u64 {}", u64::MAX);
	println!("Min u8 {}", u8::MIN);
	println!("Min u16 {}", u16::MIN);
	println!("Min u32 {}", u32::MIN);
	println!("Min u64 {}", u64::MIN);

	println!("Max isize {}", isize::MAX);
	println!("Max usize {}", usize::MAX);
	println!("Max f32 {}", f32::MAX);
	println!("Max f64 {}", f64::MAX);
	println!("Min isize {}", isize::MIN);
	println!("Min usize {}", usize::MIN);
	println!("Min f32 {}", f32::MIN);
	println!("Min f64 {}", f64::MIN);

	// Max i8 127
	// Max i16 32767
	// Max i32 2147483647
	// Max i64 9223372036854775807
	// Min i8 -128
	// Min i16 -32768
	// Min i32 -2147483648
	// Min i64 -9223372036854775808
	// Max u8 255
	// Max u16 65535
	// Max u32 4294967295
	// Max u64 18446744073709551615
	// Min u8 0
	// Min u16 0
	// Min u32 0
	// Min u64 0
	// Max isize 9223372036854775807
	// Max usize 18446744073709551615
	// Max f32 340282350000000000000000000000000000000
	// Max f64 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
	// Min isize -9223372036854775808
	// Min usize 0
	// Min f32 -340282350000000000000000000000000000000
	// Min f64 -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

}

// rustc learn_number.rs -A warnings &&  ./learn_number.exe
