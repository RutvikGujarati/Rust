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

//------------------- heap memory

// fn main() {
//     let s1 = String::from("Hello, world!"); // current owner is s1 of Hello world
//     fn_str_pass(s1); // transfer to second function
//     println!("s1 printing{}", s1); // gives error cause no owner of hello world at this point
// }

// fn fn_str_pass(value: String) {
//     println!("s2 printing..{}", value);
// }

// ------------ stack memory

// fn main() {
//     let integer: u8 = 34;
//     pass_integer(integer);
//     println!("int passing..{}", integer);// this will run successfully cause it works on stack
// }

// fn pass_integer(value:u8) {
//     println!("int passing..{}", value);
// }

//using references to avoid ownership transfer

fn main() {
    let s1: String = String::from("Hello, world!");
    fn_str_pass(&s1); // pass reference to the function
    println!("s1 printing..{}", s1); // this will run successfully cause we still own s1
}

fn fn_str_pass(value: &String) {
    println!("s2 printing..{}", value);
}
