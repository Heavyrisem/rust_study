fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!"); // ERROR!, 변환을 할 타입을 지정해야 함

    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("heart eyed cat: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let element = a[10];
    println!("The value of element is: {}", element); // ERROR, 배열의 범위를 벗어남
}
