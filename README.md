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
