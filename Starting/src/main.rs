// function to find a user by their username which returns an Option type
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // returns an OptionCsjyPKhvea8Z8x7zEysNGUcRah5w6Sc18oaqo8796pGa
    let user_option = get_user("Hari");
    // use of match expression to get the result out of Option
    let result = match user_option {
        Some(user) => user,
        None => "not found!",
    };
    // print the result
    println!("user = {:?}", result);
}
