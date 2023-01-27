use::std::io::{stdin,stdout,Write};
use std::num::IntErrorKind;


fn main() {
    // defines variable s as a String
    let mut s = String::new();

    // output "Enter your number: " to the screen (however unlike println it stays on the same line
    write!(stdout(), "Enter your number: ").unwrap();

    // makes sure the text above is displayed in the buffer
    let _ = stdout().flush();

    // reads the user input test
    stdin().read_line(&mut s).unwrap();

    //converts the input to an i32 and also catches Position overflows and invalid digits respectively
    let decimal_number = match i32::from_str_radix(&s.trim(), 10) {
        Ok(decimal_number) => decimal_number,
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow => {
                    println!("Position Overflow: The number you entered is too big!");
                    return;
                    
                },
                IntErrorKind::InvalidDigit => {
                    println!("Invalid Digit: Make sure you are entering only numbers");
                    return;
                    

                }
                _ => {
                    println!("Error: Please enter a valid number!");
                    return;
                    
                }
        }
    }
 
    };
    // prints the binary number
    println!("That is {:b} in binary", decimal_number);


    
   
}
