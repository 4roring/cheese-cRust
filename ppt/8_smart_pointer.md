---
marp: true
title: cheese cRust 8주차
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
스마트 포인터
![height:300px](../images/study_logo.png) ![height:300px](../images/pseudo_lab_logo.jpg)

---

## Box<T>로 힙에 있는 데이터 가리키기

---

## Box<T>로 힙에 데이터 저장

---

## Box로 재귀적 타입 가능하게 하기

---

## 콘스리스트, Lisp??

---

## 비재귀적 타입 크기 계산

---

## Box<T>활용 알려진 크기의 재귀적 타입 생성

---

## Deref 트레이트

---

## 포인터를 따라가 값 얻기

---

## Box<T>를 참조자처럼 사용

---

## 자체 스마트 포인터 정의

---

## Deref 트레이트를 구현

---

## 함수와 메서드를 이용한 암묵적 역참조 강제

---

## 역참조 강제가 가변성과 상호작용하는 법

---

## Drop 트레이트로 메모리 정리

---

## std::mem::drop으로 값을 일찍 버리기 

---

## Rc<T>, 참조 카운트 스마트 포인터

---

## Rc<T>를 사용하여 데이터 공유하기

---

## Rc<T>를 클론하는 것은 참조 카운트를 증가시킵니다 

---

## RefCell<T>와 내부 가변성 패턴

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
