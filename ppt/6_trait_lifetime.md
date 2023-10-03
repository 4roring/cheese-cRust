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
