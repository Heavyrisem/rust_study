fn main() {
    let number = 3;

    // 조건은 항상 bool 값이여야 함
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!();

    let number = 6;

    // else if 문이 많아지면 가독성이 떨어지기 때문에 match를 사용, Ch6.2
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!();

    let condition = true;
    // 코드 블럭은 마지막에 위치한 표현식을 산출함, 숫자는 표현식이 될 수 있음
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // ERROR, number의 타입이 두 가지일 수 없음

    println!("The value of number is: {}", number);
    println!();

    // 무한 루프
    // loop {
    //     println!("again!");
    // }

    let mut number = 3;
    // True인 경우 반복
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!");
    println!();

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while문으로 배열 요소 반복
    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }
    println!();

    // for문으로 배열 요소 반복
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    println!();

    // Range.rev(), range reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
