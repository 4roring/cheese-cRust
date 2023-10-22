---
marp: true
title: cheese cRust 7주차
paginate: true
theme: uncover

---
<style>
{
    font-size: 30px
}
</style>

# **cheese cRust** 
# 가짜연구소 Rust 8주차
함수형 언어의 특징
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 함수형 언어의 특징: 반복자와 클로저

- 함수를 값처럼 넘기는 것
- 다른 함수 결과값을 함수로 반환하는 것
- 나중에 실행하기 위해 변수에 함수를 할당하는 것

---

## 클로저 (Closure)

- 변수나 다른 함수에 전달 할 수 있는 익명 함수
- 함수와 다른 점은 정의된 클래스에서 값을 캡처할 수 있음

---

## 캡처

```rust
let string1 = String::from("test");
let closure1 = || {
    println!("{string1}");
};
closure1()
```
- closure는 현재 환경에서의 변수 및 함수를 가져올 수 있다
- 위와 같이 가져오는 것을 캡처라고 한다
- 선언 후 함수처럼 호출할 수 있다

---

## 클로저 타입 추론과 명시

```rust
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1;
```

- 함수와 비교하여 두번째는 모든 것이 명시된 클로저
- 세번째는 리턴 타입이 제거된 클로저
- 네번째는 표현식이 하나 뿐이라 중괄호도 없는 클로저

---

```rust
let example_closure = |x| x;    
let s = example_closure(String::from("hello"));    
let n = example_closure(5);
```

- 타입 명시 없이 두 타입에 대해 클로저 사용시 컴파일 에러
- 처음 컴파일러가 String 타입으로 추론하고 타입을 고정해버림

---

## 참조자를 캡처 또는 소유권 이동

```rust 
let list = vec![1, 2, 3];    
println!("Before defining closure: {:?}", list);
let only_borrows = || println!("From closure: {:?}", list);
println!("Before calling closure: {:?}", list);
only_borrows();
println!("After calling closure: {:?}", list); 
```

- 세가지 방식으로 자신의 환경의 값을 가져올 수 있음
- 위 방법은 불변 참조자를 가져와서 사용하는 부분
- 컴파일 에러 없이 전부 출력된다

---

```rust
let mut list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);
let mut borrows_mutably = || list.push(7);
borrows_mutably();
println!("After calling closure: {:?}", list); 
```

- 가변 참조자 캡처
- 캡처된 순간 가변 대여 상태
- 호출 후엔 7이 추가된 결과를 볼 수 있다


---

```
4 |     let mut borrows_mutably = || list.push(7);
  |                               -- ---- first borrow occurs due to use of `list` in closure
  |                               |
  |                               mutable borrow occurs here
5 |     println!("Before call closure: {:?}", list);
  |                                           ^^^^ immutable borrow occurs here
6 |     borrows_mutably();
  |     --------------- mutable borrow later used here
```

- 캡처 후 호출 전 println!을 호출하면 가변 대여중이라 불변 대여를 허용하지 않는 상태

---

```rust
use std::thread;
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

- 소유권을 넘기려면 move 키워드가 필요
- 스레드에는 빌려주고 작업 중 밖에서 list가 해제되면 위험
- 스레드에서 실행되는 클로저에 list 소유권을 넘겨서 관리하도록 한다

---


## 캡처한 값을 클로저 밖으로 이동, Fn 크레이트



---

## 반복자로 일련의 아이템 처리

---

## Iterator 트레이트, next 메서드

---

## 반복자를 소비하는 메서드

---

## 다른 반복자를 생성하는 메서드

---

## 환경을 캡처하는 클로저

---

## 반복자 어댑터로 더 간결한 코드 만들기

---

## 루프와 반복자 선택하기

---

## 루프 vs 반복자 