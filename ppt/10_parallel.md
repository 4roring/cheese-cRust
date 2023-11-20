---
marp: true
title: cheese cRust 10주차
paginate: true
theme: uncover
---

<style>
{
    font-size: 30px
}
</style>

# **cheese cRust**

# 가짜연구소 Rust 10주차

겁 없는 동시성
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 동시성 or 병렬 프로그래밍

- 스레드를 생성하여 여러 조각의 코드를 동시에 실행
- 채널들이 스레드 간 메시지를 보내는 메시지-패싱 동시성
- 여러 스레드가 어떤 동일 데이터에 접근할 수 있는 상태-공유 동시성
- 동시성을 사용자 저의 타입으로 확장하는 Sync, Send 트레이트

---

# 스레드를 이용하여 코드를 동시에 실행하기

```rust
thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1));
}
```

- thread::spawn을 통하여 스레드를 생성
- 단 메인 스레드가 완료되면 모든 스레드는 실행이 종료
- 위 코드는 10까지 전부 출력하진 못한다

---

## join 핸들을 사용하여 모든 스레드 종료 대기

```rust
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();
```

- 스레드를 spawn 후 handle을 받고 join을 호출하면 종료까지 대기한다

---

## 스레드에 move 클로저 사용하기

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

- 클로저 내에서 v를 캡처하여 사용
- 그냥은 별도의 스레드에서 해당 값을 사용할 수 없음
- move 키워드를 통하여 넘겨줘야함

---

# 메시지 패싱으로 스레드간 데이터 전송

- 안전한 동시성을 보장하기 위한 접근법 중 하나
- 아이디어는 **메모리를 공유하여 통신X**
- 통신하여 메모리를 공유
- **Rust는 채널(channel) 구현체를 제공**

---

```rust
let (tx, rx) = mpsc::channel();
```

- 채널은 송신자(transmitter)와 수신자(receiver)가 존재
- 송신자 또는 수신자가 버려지면 채널은 closed
- mpsc는 복수 생산자, 단일 소비자(**m**ultiple **p**roducer, **s**ingle **c**onsumer)
- 송신 단말은 여럿 가질 수 있으나 수신 단말은 하나만 가질 수 있음

---

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {received}");
```

- 스레드에 송신자(tx)를 넘겨주고 hi라는 메시지를 전송
- 메인 스레드의 수신자(rx)는 송신자가 보낸 메시지를 받아서 출력
- 이때 송신자가 넘겨준 val의 소유권은 수신자에게 넘어감

---

## 여러 값 보내기와 수신자 대기 알아보기

```rust
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

- 수신자(rx) 또한 반복자 처럼 사용 가능
- 위 코드는 송신자가 4개의 문자열 모두 전송할 때까지 지속됨

---

- join을 걸지 않았지만 기다림
- 송신자 또는 수신자 객체가 소멸하여 closed 될 때 까지 살아있기 때문
- 문자열 4개를 다 보내고 송신자는 스레드가 종료되며 소멸
- 수신자도 종료되고 프로그램이 종료됨

---

# 공유 상태 동시성

- 메시지 패싱이 유일한 수단은 아님
- 여러 스레드가 동일한 공유 데이터에 접근하는 방법
- 오히려 고전적인 mutex를 활용한 동기화 방법

---

## 뮤텍스 활용으로 한 번에 한 스레드 접근 허용

- Mutex는 상호 배제(mutual exclusion)의 줄임말
- 데이터 접근하기 전 락(lock)을 얻는 요청을 하여 접근 희망 신호 전송
- 잠금 시스템으로 데이터를 보호하는 것으로 묘사

---

## 뮤텍스의 두가지 규칙

- 데이터를 사용하기 전 락을 얻어야함
- 뮤텍스가 보호하는 데이터 사용이 끝났다면, 언락을 해야 다른 스레드가 락을 얻을 수 있음

---

```rust
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}
println!("m = {:?}", m);
```

- Mutex<T>를 new를 통해 생성
- lock 메서드를 사용하여 락을 얻고 뮤텍스가 가진 값을 6으로 교체
- Mutex<T>는 스마트 포인터!

---

## 여러 스레드 사이에서 Mutex<T> 공유하기

```rust
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
```

- 위와 같이 counter를 스레드로 넘겨줘 값을 더하려 했으나 컴파일 에러

---

- 첫 번째 스레드로 counter의 소유권이 이전되어 버림
- 두 번쨰 스레드부터 무효화된 counter를 사용하려고 하여 실패

---

```rust
let counter = Rc::new(Mutex::new(0));
...
```

- 다중 참조를 위해 Rc 스마트 포인터를 활용
- 하지만 다른 내용으로 실패
- Rc<T>는 thread safe 하지 않음
- clone 호출할 때 더하고 버릴 때 카운트를 제외
- 하지만 다른 스레드에 의해 카운트 변경이 잘못 될 수 있음
  (레이스 컨디션)

---

## Arc<T>를 이용한 아토믹 참조 카운팅

```rust
    let counter = Arc::new(Mutex::new(0));
    ...
```

- 동시적 상황에서 안전하게 사용할 수 있는 Rc<T> 같은 타입
- a는 아토믹(atomic)을 의미
- 원자적으로 참조자를 세는 타입
- 자세히는 다루지 않음, 표준 라이브러리 문서 참조
- 스레드를 교차해도 안전하다는 것만 알면됨

---

# Sync와 Send 트레이드로 동시성 확장

- Rust는 매우 적은 숫자의 동시성 기능을 갖고 있음
- 커스텀한 동시성 작성 가능
- std::marker 트레이트인 Sync와 Send

---

## Send - 스레드 사이에 소유권 이동 허용

- Send가 구현된 타입은 스레드간 소유권 이동이 가능함을 나타냄
- 대부분의 러스트 타입은 Send 이지만 예외가 있음
- 대표적으로 Rc<T>

---

## Sync - 여러 스레드로 부터 접근 허용

- Sync가 구현된 타입은 여러 스레드로부터 안전하게 참조 가능함을 나타냄
- &T가 Send면, 참조자가 다른 스레드로 안전하게 보낼 수 있다면 T는 Sync
- 동일하게 Rc<T>는 Sync하지 않음

---

## Send, Sync 손수 구현은 안전하지 않음

- 손수 구현하는 것은 unsafe한 러스트 코드 구현을 하게될 수 있음
- 신중하게 구현해야함
- 이러한 정보가 많은 곳은 [러스트노미콘](https://doc.rust-lang.org/nomicon/)에 정리되어 있음
- 흑마술!

---
