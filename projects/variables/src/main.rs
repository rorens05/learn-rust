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
    
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[1] = {} | arr[2] = {}", arr[1], arr[2]);

    another_function(100);

    let sum_of_two_numbers: i32 = sum(4, 5);
    println!("Sum = {sum_of_two_numbers}");

}
 
fn another_function(another_variable: i32) {
    println!("This is another function's another_variable: {}", another_variable);
}

fn sum(x: i32, y: i32) -> i32 {
    println!("Solving for sum of {} and {}", x, y);
    x + y
}