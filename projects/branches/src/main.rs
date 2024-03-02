use std::io;
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
