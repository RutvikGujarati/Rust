fn main() {
    let num: u8 = 46;
    let str_value: String = String::from("non changeable variable");

    println!(" {}", str_value);
    let str_value = "new change";
    println!("Hello, world! {}", num);
    println!(" {}", str_value);
}
