---
marp: true
title: cheese cRust 11주차
paginate: true
theme: uncover
---

<style>
{
    font-size: 30px
}
</style>

# **cheese cRust**

# 가짜연구소 Rust 11주차

객체 지향 프로그래밍 기능들
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 객체 지향 언어의 특성

- 함수형 프로그래밍 등 OOP에서 영향을 받음
- 그 외에도 객체, 캡슐화, 상속 등 지원

---

## 객체는 데이터와 동작을 담습니다

- 객체-지향 프로그램은 객체로 구성
- 객체는 데이터 및 이를 활용하는 프로시저로 묶음
- 이 프로시저들은 메소드 혹은 연산으로 불림
- **Rust 또한 구조체와 열거형은 데이터를 가지고 impl 브록은 메소드를 제공**
- **객체라 호칭하지 않지만 동일한 기능을 수행** 

---

## 상세 구현을 은닉하는 캡슐화

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

```rust
impl AveragedCollection {
    pub fn add(&mut self, value: i32) { ... self.update_average(); }

    pub fn remove(&mut self) -> Option<i32> {... self.update_average(); ...}
                
    pub fn average(&self) -> f64 { self.average }

    fn update_average(&mut self) { ... }
}
```

- 공개 메소드를 통하여 값을 추가, 삭제할 때 비공개 메소드로 평균을 업데이트
- list와 average 필드를 비공개로 두었으니 외부 코드가 직접 추가 제거 할 수 없음
- 내부 구현을 캡슐화하였기에 차후 데이터 구조 등 쉽게 변경 가능

---

## 타입 시스템과 코드 공유로서의 상속

- 러스트는 부모 구조체의 필드와 메소드 구현을 상속받는 구조체를 정의 할 수 없음
- 대신 trait을 이용, 기본 메소드를 구현하여 공유할 수 있음
- 상속의 목적 중 하나인 코드 재사용

---

- 또 다른 목적은 타입 시스템 (다형성))
- 자식 타입을 같은 위치에서 부모 타입처럼 사용할 수 있게 하는 부분
- Rust 에서는 제네릭을 사용하여 호환 가능한 타입 추상화, trait 바운드를 이용
- 범주내 매개변수형 다형성이라고도 부름
- 최근 상속의 인기가 떨어지고 있는데 필요 이상의 코드가 공유될 위험이 있기때문

---

## 트레잇 객체를 사용하여 다른 타입 간의 값 허용

- GUI 앱 개발시 UI를 그릴 때 상속이 있는 언어라면 draw라는 메소드를 정의
- 각 컴포넌트에서 오버라이딩하여 그리는 부분을 처리할 것
- Rust는 상속이 없기에 다른 방법으로 확장하도록 할 필요가 있음

---

## 공통된 동작을 위한 trait 정의

```rust
pub trait Draw {
    fn draw(&self);
}
```

- draw 메소드를 가지는 Draw 트레잇을 정의
- 트레잇 객체를 취하는 벡터를 정의할 수 있음
- 트레잇 객체는 특정 트레잇을 구현한 타입의 인스턴스

---

```rust
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- Draw 트레잇을 구현하는 객체들의 벡터 항목 components를 소유한 구조체
- 트레잇 객체는 런타임에 여러 구체 타입을 트레잇 객체에 채울 수 있음

---

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
```

- 위와 같이 제네릭으로 구현했을 경우 하나의 타입으로 고정되어버릴 것

---

## trait 구현

```rust
pub struct Button { ... }
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

```rust
struct SelectBox { ... }
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

- 위와 같이 필요 컴포넌트를 정의
- Draw trait을 구현

---

```rust
let screen = Screen {
        components: vec![
            Box::new(SelectBox {...}),
            Box::new(Button {...}),
        ],
    };
    screen.run();
```

- trait 객체를 사용하여 동일한 trait 을 구현하는 서로 다른 값 저장
- 이러한 값의 구체적인 타입이 아닌 값이 응답하는 메시지만 고려한 개념
- 동적 타입 언어의 오리 타이핑(duck typing) 개념과 유사
(오리처럼 뒤뚱 거리고 꽥꽥 거리면 오리임이 틀림 없다 라는 의미)
- 트레잇 객체와 러스트 타입 시스템을 사용하는 장점은 특정 메소드 구현 검사, 미구현 상태 호출 등을 컴파일러에서 검사해주기에 따로 체크하지 않아도 사용할 수 있음

---

## 트레잇 객체는 동적 디스패치를 수행

- 제네릭은 타입에 따라 각 함수, 메소드 구현체를 생성
- 단형성화로 부터 야기된 정적 디스패치를 수행
- 동적 디스패치는 컴파일러는 런타임에 어떤 메소드가 호출될지 알아내는 코드를 작성

---

- trait 객체 내 존재하는 포인터를 사용하여 어떤 메소드가 호출될지 알아냄
- 정적 디스패치시에는 일어나지 않는 탐색이 발생, 런타임 비용이 있음
- 코드 인라인화 선택도 막음
- **몇 가지 최적화를 막지만 유연성을 얻음**

---

## 트레잇 객체에 대한 객체 안정성 요구

- object-safe 한 트레잇만 객체로 만들 수 있음
- 몇 가지 복잡한 규칙이 있으나 실전에선 두 가지만 만족하면 안전
- 반환값의 타입이 Self가 아니어야함
- 제네릭 타입 매개변수가 없어야함

---

## 객체 지향 디자인 패턴 구현

---

## 미션은 디자인패턴 하나 공부해서 구현하기