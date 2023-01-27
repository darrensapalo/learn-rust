fn main() {
    let name = "Darren";
    println!("Hello {}!", name);

    {
        let friend = "Spongebob";
        println!("Hello {}!", friend);
    }

    // This will no longer work, because `friend` is out of scope.
    // println!("Hello {}!", friend);

    println!("This is a string literal: {}", name);

    let mut name_str = String::from(name);
    name_str.push_str(" Welcome!");

    // // takes_ownership(name_str);
    // let itGotMoved = name_str;

    println!("New: {}", name_str);
}
