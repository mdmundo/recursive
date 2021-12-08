use recursive::fact;

#[test]
fn fact_0() {
    let result = fact(0);
    assert_eq!(result, 1);
}

#[test]
fn fact_1() {
    let result = fact(1);
    assert_eq!(result, 1);
}

#[test]
fn fact_2() {
    let result = fact(2);
    assert_eq!(result, 2);
}

#[test]
fn fact_3() {
    let result = fact(3);
    assert_eq!(result, 6);
}

#[test]
fn fact_4() {
    let result = fact(4);
    assert_eq!(result, 24);
}

#[test]
fn fact_5() {
    let result = fact(5);
    assert_eq!(result, 120);
}

#[test]
fn fact_6() {
    let result = fact(6);
    assert_eq!(result, 720);
}
