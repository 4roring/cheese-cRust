---
marp: true
title: cheese cRust 6주차
paginate: true
theme: uncover

---
<style>
{
    font-size: 30px
}
</style>

# **cheese cRust** 
# 가짜연구소 Rust 6주차
trait, lifetime
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 트레이트로 공통된 동작 정의

- 특정 타입이 가지고 있으면서 다른 타입과 공유할 수 있는 기능을 정의
- 공통된 기능 추상화
- 트레이트 바운드를 이용, 어떤 제네릭 타입에 특정 동작(트레이트)을 갖춘 타입이 올 수 있음을 명시 가능
- 다른 언어의 인터페이스(interface)와 유사

---

## 트레이트 정의

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- Summary 이름의 트레이트
- pub로 선언하여 다른 크레이트가 사용할 수 있도록
- 중괄호 안에는 트레이트를 구현할 타입의 메서드 시그니처 선언

---

## 특정 타입에 트레이트 구현

```rust
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- impl 뒤에 트레이트 명을 적고 for 뒤에 구현할 타입명을 명시
- 중괄호안에 트레이트 메서드 구현


---

## 기본 구현

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

```rust
println!("New article available! {}", article.summarize());
```

- 트레이트 정의에서 기본 기능 구현 가능
- 트레이트 구현 후 메서드를 구현하지 않으면 기본 구현으로 동작
- 오버라이딩 하는 경우 기본 동작은 하지 않음


---

## 매개변수로서의 트레이트

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

- Summary 트레이트를 구현한 객체라면 전부 인자로 사용 가능

---

## 트레이트 바운드 문법

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- 콜론(:) 뒤가 트레이트 바운드
- 조금 더 복잡한 상황에서 활용 가능

---

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
↓
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

- 위와 같이 각각 impl 문법을 통하여 타입을 지정했던 것을 줄일 수 있음

---

## 구문으로 트레이트 바운드를 여럿 지정

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

or

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

- 여러 트레이트 바운드를 지정할 수 있음
- 많아지면 가독성이 안좋음

---

## where 조항으로 트레이트 바운드 정리

```rust
pub fn notify<T>(item: &T) 
where
    T: Summary + Display
{
```

- where 조항을 사용하는 것으로 가독성 증가

---

## 트레이트를 구현하는 타입 반환

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        ...
    }
}
```

- 트레이트를 구현한 타입에 대한 반환도 가능

---


```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            ...
        }
        Tweet  {
            ...
        }
    }
}
```

- 다양한 타입 반환은 불가능
- 방법은 있으나 이후에 다룰 예정

---

## 트레이트 바운드를 사용해 조건부 메서드 구현

```rust
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

- 위와 같이 Display, PartialOrd 트레이트에 동작하는 Pair의 메서드 작성
- **포괄 구현(blanket implementations)** 이라고 함

---

# 라이프 타임

- 또 다른 종류의 제네릭
- 어떤 참조자가 필요한 기간 동안 유효함을 보장하도록 함
- 러스트의 모든 참조자는 라이프타임이라는 유효성 보장 범위를 가짐

---

```rust
let r;

{
    let x = 5;
    r = &x;
}

println!("r: {}", r);
```

- 스코프 밖으로 벗어난 값을 참조하는 코드
- x가 충분히 오래 유효하지 않아 에러 발생

---

## 대여 검사기

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

- 러스트 컴파일러는 대여 검사기(borrow checker)로 스코프를 비교, 대여의 유효성을 판단함
- 주석으로 r은 'a, x는 'b로 표현
- 'a 블록보다 'b 블록이 짧아 참조 대상이 참조자 보다 오래 살지 못하니 러스트에서는 컴파일 하지 않음

---

## 함수에서 제네릭 라이프타임

```rust
let string1 = String::from("abcd");
let string2 = "xyz";
let result = longest(string1.as_str(), string2);
```

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- 위 longest 함수는 라이프 타임 관련 에러 발생
- 라이프타임 매개개변수가 필요하다는 내용
- 반환할 참조자가 x인지 y인지 rust는 알 수 없음
- 전달 받는 값을 알 수 없어 라이프타임도 알 수 없음
- **x와 y중 뭐가 반환될지도 모르고 언제 사라지는 지 모르는 상황**

---

## 라이프타임 명시 문법

```rust
&i32        // 참조자
&'a i32     // 명시적인 라이프타임이 있는 참조자
&'a mut i32 // 명시적인 라이프타임이 있는 가변 참조자
```

- 대여 검사기가 분석할 수 있도록 인자에 라이프타임 명시가 필요
- 어퍼스트로피(')로 시작, 제네릭 처럼 짧은 소문자로 보통은 지정

---

## 함수 시그니처에 라이프타임 명시

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- 제네릭과 동일하게 제네릭 라이프타임 메개변수를 선언
- 매개변수의 참조자 모두 유효한 동안 반환된 참조자도 유효하다는 명시
- 매개변수와 반환 값 간의 라이프타임 관계
- **단 명시한다고 라이프타임이 변경되지는 않음**

---

```rust
let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is {}", result);
```

- 서로 다른 라이프타임을 가진 상태로 longest 함수 호출
- 스코프 밖에서 result 사용
- 컴파일 에러가 발생하고 string2와 result에 대한 에러 출력

---

```
let string2 = String::from("xyz");
    ------- binding `string2` declared here
result = longest(string1.as_str(), string2.as_str());
                                   ^^^^^^^^^^^^^^^^ borrowed 
}
- `string2` dropped here while still borrowed
println!("The longest string is {}", result);
                                     ------ borrow later used here
```

- 코드적으로는 string1이 result로 넘어와서 문제 없을 것 같지만 아님
- 라이프타임 'a를 명시했기에 정확하게 문제를 파악함

---

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

- y의 라이프타임은 반환값 x와 관계가 없어 위와 같이 작성 가능
- 레퍼런스로 넘어온 인자에 값을 채우는 작업 등에 활용 될 것 같음

---

## 구조체 정의에서 라이프타임 명시

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

- 구조체가 참조자를 가지고 있게 하는 경우 라이프타임 명시 필요

---

## 라이프타임 생략


```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

- 위 함수에서는 라이프타임을 명시하지 않아도 동작했음
- rust 1.0 버전에서는 불가능 했음
- 하지만 반복되는 패턴들이 있었음 
- 이 상황을 대여 검사기가 추론할 수 있도록 함
- 라이프타임 생략 규칙(lifetime elision rules)이라고 함

---

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

- 매개변수는 입력 라이프타임
- 반환 값은 출력 라이프타임
- 명시가 없을 때 라이프타임을 알 수 없는 참조자가 있다면 컴파일에러

---

## 메서드 정의에서 라이프타임 명시

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

---

## 정적 라이프타임

```rust
let s: &'static str = "I have a static lifetime.";
```

- 'static로 명시
- 프로그램 전체 생애주기 동안 살아있음을 의미
- 모든 문자열 리터럴은 'static 라이프 타임을 가짐

---

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

제네릭 타입 매개변수, 트레이트 바운드, 라이프타임 문법 모두 사용한 코드

---

# 6주차 미션

---

- 12.I/O프로젝트: 커멘드 라인 프로그램 만들기의 12.3, 12.4, 12.5를 작업
**(필수 미션)**