fn sum_array(input: &[i32]) -> i32 {
    todo!();
}

fn swap(lhs: &mut String, rhs: &mut String) {
    todo!();
}

fn main() {
    let input_arr = [1, 2, 3, 4, 5];
    let sum_arr = sum_array(&input_arr);
    const SUM_RESULT: i32 = 15;
    assert!(sum_arr == SUM_RESULT);
    println!("{sum_arr} == {SUM_RESULT}");

    let mut lhs = String::from("AAA");
    let mut rhs = String::from("BBB");
    swap(&mut lhs, &mut rhs);
    assert!(lhs == "BBB");
    assert!(rhs == "AAA");
    println!("{lhs} {rhs}");
}
