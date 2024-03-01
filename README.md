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


