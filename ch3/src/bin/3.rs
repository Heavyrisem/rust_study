fn main() {
    println!("Hello, World!");
    another_function();
    println!();
    another_function_arg(5);
    println!();
    another_function_args(5, 6);
    println!();

    // let x = (let y = 6); // ERROR
    let x = 5; // 구문
    let y = {
        let x = 3;
        x + 1 // 표현식은 마지막 줄에 세미콜론을 쓰지 않음, 쓰게 되면 구문으로 변경되어 반환되는 값이 아니게 됨
    };

    println!("The value of y is: {}", y);
    println!();

    let x = five();
    println!("The value of x is: {}", x);
    println!();

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    // Rust의 기본 네이밍 규칙은 snake case
    // 함수의 선언 순서를 고려하지 않음
    println!("Another Hello World");
}

fn another_function_arg(x: i32) {
    // 항상 매개변수의 타입을 지정 해 주어야 함
    println!("The value of x is: {}", x);
}

fn another_function_args(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // 표현식을 사용하여 값 반환
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 표현식을 사용하여 값 반환
}
