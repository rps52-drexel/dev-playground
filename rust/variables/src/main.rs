fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is: {} the value of y is: {}", x, y);

    let tup: (i64, f64, u8) = (500, 6.4, 1);
    let(x, y, z) = tup;
    println!("The value of x is {} The value of y is: {} The value of z is {}", x, y, z);
}
