// All std library types automatically are printable with {:?} and {:#?}:
// But the user's custom structure requires explicit declarations that require `std::fmt::Debug` trait
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Structure<'a> (&'a str);

fn main() {

    let sex = Structure("girl");

    // normal print
    println!("{:?}", sex); //Structure("girl")

    // Pretty print
    println!("{:#?}", sex);
    //Structure(
    //  "girl"
    // )
    //

    let name = "Melon";
    let age = 18;
    let melon = Person { name, age };//same as let melon = Person { "name":name, "age": age };

    // normal print
    println!("{:?}", melon); //Person { name: "Melon", age: 18 }

    // Pretty print
    println!("{:#?}", melon);
    //Person {
    //  name: "Melon",
    //  age: 18
    //}
}
