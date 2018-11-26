fn main() {

	println!("Hello watermelon");//Hello watermelon
	
	let hulk = Hero::Strong(100);
	let quick_silver = Hero::Fast;
	let spider_man = Hero::Info {
		name:"Spiderman".to_owned(),secret:"Peter Parker".to_owned()
	};
	//convert string literal into a regular string 
	// and to do that we go to owns

	get_info(hulk);//Lifts 100 tons
	get_info(quick_silver);//Fast
	get_info(spider_man);//name is Spiderman,secret is Peter Parker

}

enum Hero{
	Fast,
	Strong(i32),
	Info {name:String,secret: String}
}

fn get_info(h:Hero){
	match h {
		Hero::Fast => println!("Fast"),
		Hero::Strong(i) => println!("Lifts {} tons",i),
		Hero::Info {name, secret} => {
			println!("name is {},secret is {}",name, secret);
		}
	}
}
// rustc learn_enum.rs -A warnings &&  ./learn_enum.exe