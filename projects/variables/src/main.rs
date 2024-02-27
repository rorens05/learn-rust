const TEST_CONSTANT_VALUE: u8 = 50;

fn main() {
    println!("TEST_CONSTANT_VALUE {}", TEST_CONSTANT_VALUE);

    let x = 1;
    println!("x = {}", x);
    let x = 2;
    println!("x = {}", x);
    {
        let x = x + 1;
        println!("x = {}", x);
    }
    println!("x = {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces {}", spaces);

    let i_8: i8 = 23;
    let i_32: i32 = 1298473;

    let sum = i32::from(i_8) + i_32;

    println!("SUM: {sum}");

}
