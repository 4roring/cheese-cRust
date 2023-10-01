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
# 가짜연구소 Rust 5주차
에러처리, 제네릭
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 에러 처리
- 주로 복구 가능, 불가능 두가지 범주로 묶음

---

## panic!으로 복구 불가능한 에러 처리

- 코드가 직접 발생
- 개발자가 직접 panic! 호출
- 두 가지 방법으로 패닉 발생
- 발생 후 에러 메시지 출력, 스택 되감기(unwind)로 청소 후 종료

---

## 스택 되감기 

---

### panic! 직접 호출하기

```rust
fn main() {
    panic!("crash and burn");
}
```

---

```
thread 'main' panicked at 'Yee', src\main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\example.exe` (exit code: 101)
```

- 위와 같은 에러 발생
- 어떤 코드의 라인에서 발생했는지 메시지 출력

---

### panic BACKTRACE 이용

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

- buffer overread 상황
- panic 발생

---

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\example.exe` (exit code: 101)
```

- 위와 같이 길이가 3인데 index가 99라는 에러 메시지 발생

---

```
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library\std\src\panicking.rs:593 
   1: core::panicking::panic_fmt
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library\core\src\panicking.rs:67 
   2: core::panicking::panic_bounds_check
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3/library\core\src\panicking.rs:162
   3: core::slice::index::impl$2::index<i32>
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3\library\core\src\slice\index.rs:261
   4: alloc::vec::impl$12::index<i32,usize,alloc::alloc::Global>
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3\library\alloc\src\vec\mod.rs:2675
   5: example::main
             at .\src\main.rs:3
   6: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/d5c2e9c342b358556da91d61ed4133f6f50fc0c3\library\core\src\ops\function.rs:250
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
- RUST_BACKTRACE 환경 변수 0이 아니게 설정
- 위와 같이 stack backtrace를 얻을 수 있다
- 에러까지 콜스택

---

## Result로 복구 가능한 에러 처리

---

# 제네릭

