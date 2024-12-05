const GREETING: u32 = 123;

fn main() {
    shadow();
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadow() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner scope value x: {x}");
    }

    println!("after inner scope dead value x: {x}");
}

fn retype() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn literals() {
    let decimal = 10_000;
    let hex = 0xff;
    let octal = 0o123;
    let binary = 0b1111_0000;
    let byte = b'A';
}

fn etc() {
    // i8, i16.. i128, isize(컴터 비트)
    // u8 ... usize
    // 오버/언더 플로우는 패닉검사, --realse인자는 검사 x
}

fn func(a: i32, b: char) {
    print!("a: {a}, b: {b}")
}

fn five() -> i32 { 5 } // ;을 붙이면 statement가 되면서 반환 x