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
# 가짜연구소 Rust 7주차
test 작성, 함수형 언어의 특징
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

# 자동화 테스트 작성하기

- **러스트는 자체적인 자동화된 소프트웨어 테스트 작성을 지원**
- python의 경우 내장된 unittest 또는 pytest를 사용함
- C/C++는 MS, Goolge, Boost 등에서 제공하는 테스트 프레임워크가 있음

---

## 테스트 작성 방법

- 테스트할 코드가 의도대로 동작하는지 검증하는 함수
1. 필요한 데이터나 상태 설정
2. 테스트할 코드 실행
3. 의도한 결과가 나오는지 확인

---

## 테스트 함수 파헤치기

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

- Rust의 테스트는 test 속성이 어노테이션된 함수
- fn 이전줄에 #[test]를 추가하면 테스트 함수로 변경
- cargo test 명령어로 테스트 실행 바이너리를 빌드 및 실행
- assert로 2+2 결과가 4인지 검사

---

```
PS D:\Projects\rust\presenter_picker\adder> cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s                  
     Running unittests src\lib.rs (target\debug\deps\adder-c4244062c19f4c41.exe)

running 1 test                                                                               
test tests::it_works ... ok                                                                  
                                                                                             
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder
```

- 실행하면 테스트 수행 결과에 대한 로그가 나옴
