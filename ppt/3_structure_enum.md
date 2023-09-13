---
marp: true
title: cheese cRust 3주차
paginate: true
theme: uncover

---
<style>
{
    font-size: 30px
}
</style>

# **cheese cRust** 
# 가짜연구소 Rust 3주차
구조체, 열거형, 패턴 매칭
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 구조체
- 여러 값의 묶음을 정의하는데 사용
- C/C++ Struct
- Python @dataclass
- 연관된 값을 가질 수 있는 측면에서는 튜플과 유사

---

## 구조체 정의


```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- struct 키워드로 이름 지정
- 중괄호 안에 필드라 부르는 각 구성 이름 및 타입 지정

---

## 구조체 인스턴스 생성

```rust
let user = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

user.memail = String::from("anotheremail@example.com");
```

- 구조체 이름 작성 후 중괄호안에 필드 이름과 저장할 값 형태로 추가
- 순서는 동일하지 않아도 된다
- 점(.) 표기법으로 값을 얻어올 수 있다

---

## 필드 초기화 축약법 사용

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

- 변수명과 필드 이름이 같은 경우 그냥 넣으면 된다

---

## 기존 인스턴스로 새 인스턴스 만들기

```rust
let new_user = User {
    active: user.active,
    ..user
};
```

- 각 필드를 기존 인스턴스의 값을 하나하나 넣을 수도 있지만
- .. 문법으로 명시된 필드 외에는 기존 인스턴스를 활용할 수 있다

---

## 튜플 구조체를 사용하여 다른 타입 만들기

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

```rust
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- Rust는 튜플과 유사한 튜플 구조체도 지원
- 구조체 이름은 지어주나, 필드는 타입만 작성한 형태
- Color와 Point는 별게의 타입
- 기존 튜플과 다르게 타입이 같아도 상호 대입X

---

## 필드가 없는 유사 유닛 구조체

```rust
struct AlwaysEqual;
```

```rust
let subject = AlwaysEqual;
```

- 필드가 없는 구조체 정의도 가능
- () 타입과 비슷하게 동작하여 유사 유닛 구조체
- 필드없이 어떤 타입에 대한 trait 구현할 때 유용
- trait는 메서드 그룹, 인터페이스 같은 것 (추후 자세히 다룹니다)

---

## 트레이트 파생으로 필드 전체 값 출력하기

```rust
[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

```rust
let rect = Rectangle {...};
println!("rect is {:?}", rect);
```

- 외부 속성 [derive(Debug)]를 작성
- 인스턴스 내 필드 값 모두 출력
- {:#?}을 이용하면 좀 더 읽기 편하게 출력

---

# 메서드 문법

- 함수와 유사, 구조체 컨텍스트에 정의 
(열거형, trait 객체 안에 정의)

---

## 메서드 정의

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- Rectangle의 컨텍스트에 함수 정의를 위한 impl 블록을 작성
- 함수 시그니처 첫 번째 매개변수는 **self (인스턴스 자신)**
- self는 Self 타입 (&self: Self를 위와 같이 줄여서 사용 가능)

---

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
```

- 필드 이름과 동일한 메서드명도 사용 가능
- 괄호를 넣으면 메서드로 동작, 그 외엔 필드를 불러옴
- private 멤버에 getter 형태로 주로 사용

---

## 더 많은 매개변수를 가진 메서드

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

- self 이후로도 매개 변수를 더 넣을 수 있다

---

## 연관 함수

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

- impl 블록 내 구현된 모든 함수는 연관 함수
- 첫 번째 인자가 self가 아닌 연관 함수를 정의할 수 있음
- self가 들어가면 메서드, 없으면 연관 함수
- Self 키워드는 impl 키워드 뒤 타입의 별칭
- 연관 함수 호출시 Self::func() 와 같이 가능

---

## 여러 impl 블록

```rust
impl Rectangle {
    fn area(&self) -> u32 {...}
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {...}
}
```

- impl 블록을 나눌 이유는 없지만 여러개 작성이 가능

---

# 열거형과 패턴 매칭
- 하나의 타입이 가질 수 있는 Variant를 열거하여 타입을 정의
- match 표현식의 패턴 매칭을 통해 열거형 값에 따른 코드 실행

---

## 열거형 정의하기

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

- IP를 예시로 v4 또는 v6만 될 수 있음
- 이런 variant를 늘어 놓을 수 있어 열거형
- 위와 같이 IpAddrKind 커스텀 데이터 타입을 열거형으로 정의할 수 있다

---

## 열거형 값

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- 각 variant에 대한 인스턴스 생성
- 각 variant 앞에 이중 콜론(::)을 붙여 접근

---

```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```

- IP를 저장하는 구조체에 IP 타입과 주소 문자열을 저장하도록 하는 예시
- 위와 같이 하지 않고 별도로 열거형에 값을 가지게 할 수 있다

---

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

```rust
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

- 열거형 각 variant에 직접 데이터를 붙일 수 있다
- 구조체를 이용하지 않아도 된다

---

```rust
impl IpAddr {
    fn call(&self) {
        // 메서드 본문
    }
}
```

- 열거형도 impl을 통하여 메서드를 정의할 수 있다

---

## Option 열거형

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- 표준라이브러리에 정의된 열거형
- 값이 있거나 없을 수 있는 상황을 나타냄
- Rust에는 null 값 대신 **Option::None** 을 사용함
- 어디에나 넣을 수 있는 null에 의한 실수를 방지하기 위한 값
- 기본 임포트 목록에 있어 그냥 **None** 으로 사용해도 됨

---

```rust
let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
```

- Option 값 예시

---

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;  // 컴파일 에러
```

- Option과 Option에 지정한 타입끼리 더하는 건 불가능
- Option 타입 문서
https://doc.rust-lang.org/std/option/enum.Option.html 

---

# match 제어 흐름 구조

- Rust에서 제공하는 강력한 제어 흐름 연산자
- switch case와 비슷하다

---

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Nickel Coin!!!");
            5
        },
        Coin::Dime => 10,
    }
}
```

- match 키워드 뒤 표현식은 coin의 값
- if 와 다른점은 어떤 타입이든 가능
- 중괄호부터 match의 갈래, 패턴과 코드로 나눠짐
- 패턴 뒤 => 연산자 후 코드, Coin::Penny에선 1 표현식으로 반환
- 결과를 각 갈래 패턴에 대해 순차적으로 비교, 매칭되면 실행
- 코드가 짧으면 중괄호 생략 가능

---

## 값을 바인딩 하는 패턴

```rust
enum IpAddrKind {
    V4(String),
    V6(String),
}
```

```rust
fn get_addr(&self) -> &String {
    let result = match self {
        IpAddrKind::V4(ip) => ip,
        IpAddrKind::V6(ip) => ip,
    };
    result
}
```

- 열거형에 값을 넣을 수 있었는데 이것을 **패턴 매칭에서만 사용할 수 있다**
- 직접 꺼내올 수 없다...

---

## Option<T>를 이용하는 매칭

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- 함수 인자로 Option<i32>를 받음
- 내부에서 매칭을 통해 None 이면 None을 반환
- Some이라면 기존 값에 1을 더한 Some를 반환

---

## match의 철저함

```rust
// 아래의 코드는 컴파일 에러 발생
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

- 갈래의 모든 패턴을 다뤄야함
- 아니면 컴파일 에러
- 이 부분이 switch ~ case와는 다른점

---

## 포괄 패턴과 _ 자리표시자

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
    // _ => move_player(other),
}
```

- 주사위를 굴려 3, 7에는 이벤트를 발생하고 그 외에는 이동하는 예시
- 3과 7 외의 값이 들어온다면 other 또는 _ 갈래에서 처리할 수 있다
- switch의 default와 유사

---

## if let을 사용한 간결한 제어 흐름

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

- match를 사용할 때 보다 간결하게 사용 가능
- 단 미구현 갈래에 대한 검증을 하지 않아 개발 상황에 따른 고민 필요