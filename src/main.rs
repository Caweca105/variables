fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let guess :u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    let a = 2.0;
    let b: f32 = 3.0;

    let sum = a + b;
    println!("The value of sum is: {sum}");

    // Character Types
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // Tuple Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    println!("The value of five_hundred is: {_five_hundred}");

    // Numeric Operations

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Boolean Types
    let _t = true;
    let _f: bool = false;

    // Arrays Types
    let m = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _b = [3; 5]; // [3, 3, 3, 3, 3]
}
