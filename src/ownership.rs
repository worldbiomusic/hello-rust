fn main() {
    ownership_borrowing();
}

fn main1() {
    /*
    compile time에 크기가 고정되는 타입은 모두 stack에 저장
    크기가 변하는 타입은 heap에 저장
     */

    // 값 복사
    let n1 = 1;
    let n2 = n1;

    //
    let s1 = String::from("hi");
    let s2 = s1; // move

    // println!("{}", s1); ownership이 s2로 옮겨감(move)
    // 함수에서 반환받을 떄도 ownership move 발생
    println!("{}", s2);
}

fn main2() {
    let s = String::from("hello");
    println!("s1: {s}");
    takes_ownership(s); // ownership move
                        // println!("s2: {s}"); // 에러 발생: s의 ownership은 take_ownership()이 가져가고, 함수가 끝난즉시 메모리 해제됨

    let x = 5;
    println!("x1: {x}");
    makes_copy(x); // Copy
    println!("x1: {x}"); // 에러 안 발생
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn ownership_borrowing() {
    // 소유권 다시 명시적으로 돌려받아서 출력하기
    let s1 = String::from("yee");
    let (s1, len) = str_len(s1);
    println!("{s1}'s length is {len}");

    // 소유권을 reference(&)를 사용해서 빌려주기
    let length = str_len2(&s1);
    println!("{s1}'s length is {len}");

    // 불변 참조자는 동시에 여러개 변수에게 빌려줄 수 있음 (동시성에서 수정작업이 없으므로 안전)
    let um1 = &s1;
    let um2 = &s1;
    println!("{} {}", um1, um2);

    // 가변 소유권을 reference(&mut)를 사용해서 빌려주기
    let mut s2 = String::from("Slime");
    str_len3(&mut s2);
    println!("newSlime is {s2}");

    // 가변 참조자는 동시에 1개의 변수에게만 "빌려줄 수 있음" (동시성 제어를 위해서)
    let b1 = &mut s2;
    // let b2 = &mut s2; // b1이 borrow 끝나기 전에 다시 borrow 불가능
    // println!("{} {}", b1, b2);

    f();
}

fn f() {
    // 같은 이유로 불변 변수가 참조하고 있는경우에도, 가변 참조를 빌려줄 수 없음
    let um3 = &s2;
    let mut m3 = &mut s2;
    println!("{} {}", um3, m3);
}

fn str_len(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn str_len2(s: &String) -> usize {
    s.len()
}

fn str_len3(s: &mut String) {
    s.push_str(" and baby slime");
}


fn exam() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다. 왜냐면 println! 매크로로 ownership이 move하고 scope가 끝나버렸으므로

    let r3 = &mut s; // 문제없음
    println!("{}", r3);
}