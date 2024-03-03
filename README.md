# NOTES
- rust style is 4 lines
- ```!``` means that you’re calling a macro instead of a normal function 
- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.

### Command line inputs
``` rust
use std::io;


let mut guess = String::new();

io::stdin()
    .readline(&mut guess)
    .expect("Failed to read line");

println!("You guessed: {}", guess);
```

### Match
Just like a switch case for the return value 
``` rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

``` js
const compare = (number1, number2) => {
    if (number1 < number2) {
        return "Less";
    } else if (number1 > number2) {
        return "Greater";
    } else {
        return "Equal";
    }
}

switch (compare(1, 2)) {
    case "Less":
        console.log("Too small!");
        break;
    case "Greater":
        console.log("Too big!");
        break;
    case "Equal":
        console.log("You win!");
        break;
    default:
        console.log("Error");
}

```

### Looping
- ```break``` is used to exit the loop
- ```continue``` is used to skip the rest of the loop and start the next iteration
``` rust
// rust
loop {
    if true {
        break;
    }
}
```

``` js
// js
while (true) {
    if (true) {
        break;
    }
}
```


### Catching Errors
Also includes parsing the string to a number
``` rust
// rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

``` js
// js
try {
    const guess = parseInt("123");
} catch (error) {
    continue;
}
```

### Constant vs Variable
- Constants are always immutable and cannot have a ```mut``` keyword
- Constants can be declared in any scope, including the global scope
- Constants are always type annotated
- Variables are immutable by default, but can be made mutable using the ```mut``` keyword
- Variables are not type annotated by default, but can be type annotated

``` rust
// Constants
const MAX_POINTS: u32 = 100_000;

// Variables
let mut x = 5;
```

### Shadowing a variable vs mut variable
- Using mut means that the variable can be changed but should be change with the same type
- Shadowing means that we can reuse the variable name but with a different type. Common use case is when parsing a string to a number so we don't have to redeclare a new variable
``` rust
// mut
let mut guess = "Testing";
guess = "Testing 2"; // This is fine
guess = 123; // This will throw an error

// Shadowing
let guess = "123";
let guess = guess.trim().parse().expect("Please type a number");
```

### Integer variables
- i32 is the most common type
- u32 is the unsigned version of i32

| lenght | signed | unsigned | range |
|--------|--------|----------|-------|
| 8-bit  | i8     | u8       | -128 to 127 or 0 to 255 |
| 16-bit | i16    | u16      | -32,768 to 32,767 or 0 to 65,535 |
| 32-bit | i32    | u32      | -2,147,483,648 to 2,147,483,647 or 0 to 4,294,967,295 |
| 64-bit | i64    | u64      | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 or 0 to 18,446,744,073,709,551,615 |
| 128-bit | i128  | u128     | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727 or 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| arch   | isize  | usize    | depends on the computer architecture |

### Parsing Integers 
You cannot perform operations with integer without converting them to the same type
``` rust
let i_8: i8 = 23;
let i_32: i32 = 1298473;

let sum = i_8 + i_32; // This will throw an error

let sum = i32::from(i_8) + i_32; // This will work
```

### Parsing Floats
``` rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32

let sum = x + y; // This will throw an error
let sum = f64::from(y) + x; // This will work

```

### Boolean Type
The boolean type in Rust is specified using the bool keyword. The possible values for a boolean type are true and false.
``` rust
let t = true;
let f: bool = false; // with explicit type annotation
```

### Character Type
The char type in Rust is specified using the char keyword. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. 
``` rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### Compound Types
#### Tuple
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
``` rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;

println!("X: {} | Y: {} | Z: {}", x, y, z);

let tup = (500, 6.4, 1);
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;

println!("Five Hundred: {} | Six Point Four: {} | One: {}", five_hundred, six_point_four, one);
```

#### Array
An array is a collection of multiple values of the same type. Arrays in Rust have a fixed length, like tuples. If you want an array that can grow or shrink in size, you want a vector.
``` rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];

println!("First: {} | Second: {}", first, second);

let a: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit type annotation
let a = [3; 5]; // This will create an array with 5 elements with the value of 3
```

### Functions
``` rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

```

### Conditions

``` rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    let number = if condition { 5 } else { "six" }; // This will throw an error

}

```

### Loops

``` rust
fn main() {
    loop {
        println!("again!");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

}
```


### labeling loops
``` rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### While Loop
``` rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### Looping Through a Collection with
``` rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

### For Loop through a collection
``` rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

### For Loop with Range
this will print 3, 2, 1 and 4 is not included
``` rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

### Strings 
``` rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```

### Ownership
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Reference
``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
```

### Modifying a reference
``` rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Multiple References Rules
- You can have either one mutable reference or any number of immutable references at a time
``` rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2); // This will throw an error
```
- You cannot have a mutable reference while having an immutable reference

``` rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

```
- You can have a mutable reference while having an immutable reference if the mutable reference is not used.
- The compiler can tell that the reference is no longer being used at a point before the end of the scope.

``` rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

### Dangling References
- Remember that all variables are deallocated after the scope (function) ends
``` rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello"); // this will be deallocated after the function ends

    &s // return a reference to the String, s, but it will be deallocated after the function ends
}
```

- The solution is to return the String itself
- Remember that the ownership of the variable is moved to the function that calls it
``` rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s // return the String itself. The ownership of the variable is moved to the function that calls it so it will not be deallocated after the function ends
}
```

### Slices
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
- Slices can reference to a part of a string or an array

``` rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let array_of_numbers = [1, 2, 3, 4, 5];
    let slice = &array_of_numbers[1..3];

}
```
- Sample program to get the first word of a string
``` rust
fn main() {
    let mut guess = String::new();

    println!("Please enter a value");

    io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

    let word = first_word(&guess[..]);

    println!("the first word is: {}", word);

    guess.clear(); // error!
}
    

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```