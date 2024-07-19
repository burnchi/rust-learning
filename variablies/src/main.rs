fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 7;
    let y = y + 1;
    println!("The value of y is {}", y);

    let z: u8 = 10;
    println!("The value of z is {}", z);

    let tup: (i32, f64, u8) = (10, 3.14, 255);
    let (a, _, _) = tup;
    println!("The value of tup is ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("The value of a is {}", a);
}
