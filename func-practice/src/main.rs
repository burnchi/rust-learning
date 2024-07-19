fn main() {
    println!("main func");
    eat();
    num(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("{}", five());
}

fn eat() {
    println!("I am eating");
}

fn num(x: u32) {
    println!("x={}", x);
}

fn five() -> u32 {
    5
}
