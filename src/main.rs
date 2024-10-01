fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    "if" is an expression, it returns the value which branch satisfy thee condition, therefore, "if" can
    use with "let".
    The type in "if" and "else" part should be same type.
    */
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
