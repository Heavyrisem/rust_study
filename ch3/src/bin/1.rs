fn main() {
    let mut x = 5; // 아래에서 값을 재할당 하기 때문에 mutable 변수여야 함
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 상수는 항상 타입을 지정해야 하고, mutable 형태일 수 없음
    const MAX_POINTS: u32 = 100_000;

    // Shadowing 때문에 이름이 같은 변수를 재선언 할 수 있음, mutable 아닌 경우에도 가능 (재선언이기 때문에)
    // mut 변수가 아니기 때문에 실행 중에 값의 "변경"을 막아줌, 단 재선언은 안됨
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    "; // String
    let spaces = spaces.len(); // number로 재선언됨

    // let mut spaces = "    "; // mutable String
    //spaces = spaces.len(); // String 변수에 number를 저장하려 해서 오류 발생
}
