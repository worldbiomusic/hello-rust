[러스트 튜토리얼](https://doc.rust-kr.org/) 배우면서 만든 코드

---

# ownership
Copy trait를 구현하고 있으면 일반적인 값 복사가 일어나지만, 아니면 ownership move가 발생
```rs
let a = 1;
let b = a; // 정수타입은 Copy trait을 구현하므로 값 복사 (ownership move 없음)

let c = String::from("hi");
let d = c; // 문자열은 Copy trait을 구현 안하므로, ownership move 발생 (=c변수 더이상 사용불가 = 문자열"hi"의 메모리 참조 권한이 변수 c로부터 d로 옮겨짐)

// ownership move는 함수로 변수를 참조없이 그대로 넘겨도 발생함
fn main() {
    let a = 1;
    println!("{a}");
    ownership_is_mine1(a);
    println!("{a}");

    let s = String::from("hi");
    println!("{s}");
    ownership_is_mine2(s);
    // println!("{s}"); // 에러 (소유권이 ownership_is_mine2으로 move됨)
}

fn ownership_is_mine1(a: i32) {
    println!("값 Copy가 발생해서 이 함수 끝나도 사용가능");
}

fn ownership_is_mine2(a: String) {
    println!("이 함수 끝나면 인자 못 씀");
}
```
Copy trait을 구현하면 함수로 변수를 넘길 때 값만 복사됨. 하지만 String같이 힙 메모리에 값이 저장되는 타입은 Copy trait **구현 불가**!


**참조 reference**  
`&`(불변) 또는 `&mut`(가변)를 사용하면 ownership borrow (빌리기)가 가능 = 진짜 ownership은 본체가 가지고 있음
```rs
let a = 1;
let b = &a; // 불변 변수 참조 borrow

let mut c = 1;
let d = &mut c; // 가변 변수 참조 borrow
```

**참조/역참조 연산자**
- `&`: 불변 참조 borrow
- `&mut`: 가변 참조 borrow
- `*`: 역참조 (참조 변수의 실제 값) (매크로(println!())에서는 auto referencing/dereferencing 발생해서 불필요)
```rs
let mut a = 1;
let b = &a; // 불변 참조 borrow
let c = &mut a; // 가변 참조 borrow
// c = 2; // 에러: 참조변수(포인터)를 변경하려 함
*c = 2; // 포인터 값 접근 후 변경

// 함수에서 참조 사용법
fn change(n: &mut i32) {
    *n = 2;
    println!("{}", n);
}
```

**ownership borrow할 때 주의**  
> ownership 빌리기의 제한은 OS의 동시성 제어를 생각하면 편함

- 동시에, 가변을 여러개 가능
```rs
let a = 1;
let b = &a;
let c = &c;
```

- 동시에, 불변 1개만 가능
> "동시" = 변수 borrow가 끝나기 전까지
```rs
let mut a = 1;
let b = &mut a;
let c = &mut a; // 불가능
println!("{b}"); // 변수 b의 borrow는 여기까지 이므로, 동시에 b,c를 borrow 불가능
```
하지만 아래처럼 scope가 끝난 상태 (=동시가 아닌)면 가능
```rs
let mut a = 1;
let b = &mut a;
*b = 2;
println!("{b}"); // b scope 끝 = 가변 a 변수 borrow 끝
let mut c = &mut a;
*c = 3;
println!("{c}"); // c scope 끝 = 가변 a 변수 borrow 끝
```

- 동시에, 가변과 불변 둘중 1개만 가능
```rs
let mut a = 1;
let b = &a;
let c = &mut a;
print!("{b}"); // 불변 b scope 여기서 끝나는데, 이전에 가변c가 접근하려고 하면 에러
```

동시에, 불변과 가변 둘중 1개만 가능
```rs
let mut a = 1;
let b = &mut a;
let c = &a; 
print!("{b}"); // 가변 b scope 여기서 끝나는데, 이전에 불변c가 접근하려고 하면 에러
```



**자동 참조 auto referencing 상황**  
1. 매크로 인자
> 예. println!()에는 그냥 변수를 줘도 참조로 바뀜
```rs
let a = String::from("hi");
println!("{}", a); // a가 &a로 참조로 바뀜
```

2. 구조체 메서드
> 예. self는 자동 참조로 바뀜
```rs
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

let rect = Rec.. // 생략
let w = rect.width(); // 내부적으로 "Rectangle.width(&rect);"로 처리됨, 이 때 rect를 자동 참조로 변경
```

---


# 기타
- compile 시점에서 모든 자료형 타입 알아야 함 (=런타임때 타입이 결정되는 행위 허용을 안 함)
- ;가 붙으면 statement가 되어버림
- auto referencing dereferencing 해줌 (예. 메서드에서 변수앞에 안붙여도 자동으로 &, &mut, *붙여서 참조와 역참조 해줌)
- println!()에 변수를 넘길 때 참조(`&`)를 안하고 넘기는데, ownership move가 발생하지 않는 이유는 내부
- src/main.rs 가 시작점
- Cargo.toml이 프로젝트 빌드 관리 관리문서

# 용어
| 용어           | 설명                                                                                                     |
|----------------|---------------------------------------------------------------------------------------------------------|
| **Trait**      | 인터페이스와 유사한 기능으로, 타입이 특정 동작을 수행할 수 있음을 명시.                                 |
| **Ownership**  | 각 값이 하나의 소유자를 가지며, 소유자가 스코프를 벗어나면 메모리가 자동 해제되는 Rust의 메모리 관리 모델. |
| **Lifetime**   | 참조의 유효 범위를 명시적으로 지정하는 시스템.                                                          |
| **Pattern Matching** | 값의 구조를 확인하고 분해하는 방식으로 `match`와 `if let`이 주로 사용됨.                                |
| **Smart Pointer** | 데이터를 가리키는 포인터 역할을 하면서, 추가적인 메모리 관리 기능 제공. 주요 종류는 `Box`, `Rc`, `RefCell`.|
| **Module**     | 코드를 구조화하고 네임스페이스를 관리하는 단위. `mod` 키워드로 선언.                                       |
| **Crate**      | Rust의 가장 작은 컴파일 단위. 바이너리 실행 파일이나 라이브러리 형태로 존재.                              |
| **Macro**      | 반복적 코드 생성을 위한 도구. `println!`과 같은 선언형 또는 프로시저형 매크로로 구성.                     |
| **Send/Sync**  | `Send`: 데이터를 스레드 간 이동 가능. `Sync`: 여러 스레드에서 동시에 접근 가능.                            |
| **Async/Await** | 비동기 작업을 작성하는 키워드. `async`로 비동기 함수 정의, `await`로 작업 대기.                             |

---

# cmd
- rustc <file>: 컴파일
- cargo new <name>: rust 프로젝트 생성
- cargo build: 빌드 (컴파일 + 바이너리)
- cargo run: build + 실행