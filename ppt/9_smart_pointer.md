---
marp: true
title: cheese cRust 9주차
paginate: true
theme: uncover
---

<style>
{
    font-size: 30px
}
</style>

# **cheese cRust**

# 가짜연구소 Rust 9주차

스마트 포인터
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 스마트 포인터

- 포인터는 메모리의 주솟값을 담고 있는 변수
- 러스트의 흔한 종류의 포인터는 참조자(&)
- 스마트 포인터는 포인터 처럼 작동하면서 추가적인 데이터, 기능도 있는 데이터 구조

---

- String, Vec<T> 같은 스마트 포인터를 마주침
- 스마트 포인터로 보는 이유는 메모리를 가지고 다룰 수 있기 때문
- 보통 구조체로 구현
- 보통 구조체와 달리 Deref, Drop 트레이트를 구현

---

- Deref는 인스턴스가 참조자 처럼 동작하도록 해줌
- Drop은 스마트 포인터가 스코프 밖으로 벗어났을 때 실행되는 코드를 커스텀하게 해줌

---

## Box<T>로 힙에 있는 데이터 가리키기

- 가장 직관적으로 사용하는 스마트 포인터 Box<T>
- 힙에 데이터를 저장할 수 있도록 해줌
- 스택에는 힙을 가리키는 포인터가 남음

---

### 자주 사용하는 상황

- 컴파일 타임에 크기를 알 수 없는 타입, 정확한 크기를 요구하는 컨텍스트 내에서 그 타입의 값을 사용하고 싶을 때
- 커다란 데이터를 가지고 소유권을 옮기고 싶지만 복사가 되지 않을 것을 보장하고 싶을 때
- 어떤 값을 소유, 구체화된 타입보다는 특정 트레이트를 구현한 타입이라는 점만 신경쓰고 싶을 때

---

## Box<T>로 힙에 데이터 저장

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

- 변수 b는 힙에 할당된 5라는 값을 가리키도록 생성
- 스코프를 벗어날 때 b는 해제

---

## Box로 재귀적 타입 가능하게 하기

- 재귀적 타입의 값은 자신 안에 동일한 타입의 또 다른 값을 담을 수 있음
- 러스트는 컴파일 타임에 타입 크기를 알아야해 재귀 타입은 빌드 에러
- 재귀 타입 예제로 cons list 가 있음

---

## cons list

- Lisp 프로그래밍 언어 및 그 파생 언어로 부터 유래된 데이터 구조
- 중첩된 쌍으로 구성, 연결리스트(linked list)의 Lisp 버전
- 생성 함수(construct function)의 cons에서 유래
- 두 개의 인수로부터 새로운 쌍을 생성
- cons에 어떤 값과 다른 쌍으로 구성된 쌍을 넣어 호출함으로 재귀적인 쌍을 이룬 cons list를 구성할 수 있다

---

```
(1, (2, (3, Nil)))
```

- 각 아이템은 두 개의 요소를 담고 있음
- 현재 아이템 값과 다음 아이템
- 마지막 아이템 다음 아이템은 없이 Nil 이라는 값을 담음
- Nil은 null과 다른 개념
- 아이템 리스트 대부분은 Vec<T>가 더 나음

---

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

```
error[E0072]: recursive type `List` has infinite size
```

- 위와 같이 작성하면 List 타입이 Cons 내부의 List로 재귀됨
- List 사이즈가 무한대가 된다는 에러 발생
- cons list를 구현할 수 없다!

---

## 비재귀적 타입 크기 계산

---

## Box<T>활용 알려진 크기의 재귀적 타입 생성

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

```rust
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

- 위와 같이 열거형 Cons의 List를 Box로 감싸주면 해결
- i32와 박스의 포인터를 저장할 공간 크기가 필요한 것을 아는 상태
- 무한 재귀를 깨뜨리고 정상 컴파일 가능

---

## Deref 트레이트

- 역참조 연산(dereference operator) \* 동작 커스텀 가능
- 곱하기, 글롭과 헷갈릴 수 있음...
- 스마트 포인터가 보통의 참조자처럼 취급될 수 있도록 Deref 구현

---

## 포인터를 따라가 값 얻기

```rust
let x = 5;
let y = &x;

assert_eq!(5, x);
assert_eq!(5, *y);
```

- x는 5의 값, y는 x를 참조
- y값이 5인 것을 체크하려면 역참조(\*)를 하여 참조자를 따라가 값을 체크
- assert_eq!(5, y); 로 작성했다면 실패했을 것

---

## Box<T>를 참조자처럼 사용

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- y는 x값을 복제한 Box<T> 인스턴스
- 여기서도 역참조를 통하여 y의 값이 5인 것을 체크할 수 있음

---

## 자체 스마트 포인터 정의

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

```rust
let x = 5;
let y = MyBox::new(x);

assert_eq!(5, x);
assert_eq!(5, *y);
```

- 유사 스마트 포인터를 만들어 어떻게 동작하는지 알아봄
- 일단 그냥 역참조 시도하면 컴파일 에러 발생

---

## Deref 트레이트를 구현

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

- Deref 트레이트를 구현
- 내부에 deref 메서드 하나를 구현하도록 요구함
- self를 빌려와서 MyBox 내부 데이터 참조자 반환
- Deref 트레이트가 없으면 참조자(&)들만 역참조 가능

---

```rust
*(y.deref())
```

- 위와 같은 코드도 동작, deref 호출에 대한 부분은 생각하지 않아도 됨
- 소유권 시스템과 함께 작동시키기 위해 위와 같은 코드가 필요
- deref가 참조자 대신 값을 직접 반환 했다면 self 바깥으로 감

---

## 함수와 메서드를 이용한 암묵적 역참조 강제

- Deref를 구현한 어떤 타입 참조자를 다른 타입 참조자로 바꿔줌
- &String -> &str로 바꿀 수 있음
- String의 Deref 구현이 &str을 반환하도록 했기 때문
- 역참조 강제는 &, \*을 사용한 명시적 참조 및 역참조를 추가할 필요가 없도록 하기 위함

---

```rust
let m = MyBox::new(String::from("Rust"));
hello(&m);          // 역참조가 가능하여 나오는 코드
hello(&(*m)[..]);   // 역참조가 불가능하면 이와 같이 작성
```

- (\*m)은 yBox<String> 역참조
- &와 [..]가 전체 문자열과 동일한 String 문자열 슬라이스를 얻어옴

---

## 역참조 강제가 가변성과 상호작용하는 법

- Deref 트레이트로 불변 참조자 \*를 오버라이딩
- DerefMut 트레이트를 사용하여 가변 참조자 \*를 오버라이딩

---

## Drop 트레이트로 메모리 정리

- 어떤 스코프 밖으로 벗어나려 할 때 커스텀
- 어떤 타입이든 구현 가능

---

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

- String을 담고있다 Drop되면 출력하고 해제되는 예시

---

## std::mem::drop으로 값을 일찍 버리기

```rust
let c = CustomSmartPointer {data: String::from("some data"),};
println!("CustomSmartPointer created.");
c.drop();
println!("CustomSmartPointer dropped before the end of main.");
```

- 직접 메모리 해제를 위하여 drop를 호출하면 에러 발생
- 명시적 호출이 금지되어있음

---

```rust
drop(c);
```

- std::mem::drop 함수를 통하여 해제
- 직접 호출을 막은 부분은 인스턴스가 댕글링일 때 호출하면 프로그램이 망가지는 것을 방지하기 위한 것으로 보임

---

## Rc<T>, 참조 카운트 스마트 포인터

- 대부분 소유권은 명확
- 하지만 하나의 값이 여러 소유자를 가질 수 있음
- 복수 소유권은 Rc<T> 타입을 활용하여 가능

---

## Rc<T>를 사용하여 데이터 공유하기

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
```

```rust
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a)); // Box였다면 여기서 에러
```

- Box로 작성했던 Cons를 Rc로 변경
- Rc::clone를 통하여 a 참조자를 복제하고 참조 카운트만 증가

---

## Rc<T>를 클론하는 것은 참조 카운트를 증가

- 초기에는 1을 가지고 있고 clone을 호출할 때 마다 1씩 증가
- 스코프 밖으로 벗어나면 Drop에서 참조카운트 1씩 감소
- 0이되면 메모리에서 정리

---

## RefCell<T>와 내부 가변성 패턴

- 내부 가변성은 어떤 데이터에 대한 불변 참조자가 있더라도 변경할 수 있게 해주는 러스트의 디자인 패턴
- 대여 규칙 허용 X
- 데이터 구조 내에서 unsafe 코드를 사용하여 러스트의 일반적 규칙 우회

---

## RefCell<T>으로 런타임에 대여 규칙 집행하기

---

## 내부 가변성: 불변값에 대한 가변 대여

---

## 내부 가변성에 대한 용례: 목 객체

---

## RefCell<T>로 런타임에 대여 추적하기

---

## Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기

---

## 순환 참조는 메모리 누수를 발생시킬 수 있습니다

---

## 순환 참조 만들기

---

## 순환 참조 방지하기: Rc<T>를 Weak<T>로 바꾸기

---

## 트리 데이터 구조 만들기: 자식 노드를 가진 Node

---

## 자식에서 부모로 가는 참조자 추가하기

---

## strong_count와 weak_count의 변화를 시각화하기
