// fn main() {
//     let s1: String = get_string();

//     println!("s1: {}", s1);
//     let s2_1: String = String::from("typing....");
//     let s2: String = get_Second_string(s2_1);
//     println!("s2: {}", s2);
// }

// fn get_string() -> String {
//     let s: String = String::from("Hello, world!");
//     return s;
// }

// fn get_Second_string(s: String) -> String {
//     return s;
// }

// heap memory 

fn main() {
    let s1 = String::from("Hello, world!"); // current owner is s1 of Hello world
    fn_str_pass(s1); // transfer to second function
    println!("s1 printing{}", s1); // gives error cause no owner of hello world at this point
}

fn fn_str_pass(value: String) {
    println!("s2 printing..{}", value);
}