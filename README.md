# Denary to Binary Convertor





## Features
    - Error handling 
        - Positive Overflow Error
        - Negative Overflow Error
        - Invalid Digit Error
    - Fully commented for clarity

## Installation

### **Use the binary provided in releases**

### Self Compilation
This section is for people who want to compile the project themselves

 Requires [Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/) to compile

```sh
git clone https://github.com/GiddyLinux/denary-to-binary-convertor.git
cd denary-to-binary-convertor
cargo build --release --target <platform of choice>
```

## Usage

Below is an example demonstration of the program
```sh
$ ./binary-decimal-x86_64-unknown-linux-gnu
Enter your number: 15
That is 1111 in binary
```
Below is an example of an error possibly triggered by misusing the program
```sh
$ ./binary-decimal-x86_64-unknown-linux-gnu
Enter your number: not a number
Invalid Digit: Make sure you are entering only numbers
```
