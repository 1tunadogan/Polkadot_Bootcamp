fn main() {
    let new_string = String::from("Tunahan");
    // we borrow the new_string reference to the calculate_length function
    let len = calculate_length(&new_string); 

    println!("The length of {} is {}.", new_string, len);

}

fn calculate_length(s: &String) -> usize { // 's' a reference here
    s.len()
} // 's' is out of scope here, but the value is not dropped because it has no owner