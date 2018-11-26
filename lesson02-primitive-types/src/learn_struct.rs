fn main() {

	println!("Hello watermelon");//Hello watermelon

	let mut circle1 = Circle{
		x: 10.0,y: 10.0,radius: 10.0
	};

	println!("x: {},y: {}, radius: {}", circle1.x,circle1.y,circle1.radius);//x: 10,y: 10, radius: 10
	println!("x: {},y: {}, radius: {}", circle1.get_x(),circle1.get_y(),get_radius(&circle1));//x: 10,y: 10, radius: 10

	println!("Circle Area : {}", circle1.area());//Circle Area : 314.159

	let mut rectangle1 = Rectangle{
		height: 10.0,width: 30.0
	};

	println!("Rectangle Area : {}", rectangle1.area());//Rectangle Area : 300

}

struct Circle {
	x: f64,
	y: f64,
	radius: f64
}

fn get_radius(circle: &Circle) -> f64{
	circle.radius
}

impl Circle {
	pub fn get_x(&self) -> f64{
		self.x
	}
	pub fn get_y(&self) -> f64{
		self.y
	}
}

struct Rectangle {
	height: f64,
	width: f64
}

trait  HasArea {
	fn area(&self) -> f64;
}

impl HasArea for Circle {
	fn area(&self) -> f64{
		3.14159 *(self.radius * self.radius)
	}
}

impl HasArea for Rectangle {
	fn area(&self) -> f64{
		self.height * self.width
	}
}
// rustc learn_struct.rs -A warnings &&  ./learn_struct.exe