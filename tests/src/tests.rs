use rust_ci_bin::add12;
use rust_ci_helpers::add4;
use rust_ci_lib::add8;

#[test]
fn add4_works() {
    let result = add4(6);
    assert_eq!(result, 10);
}
#[test]
fn add8_works() {
    let result = add8(6);
    assert_eq!(result, 14);
}
#[test]
fn add12_works() {
    let result = add12(6);
    assert_eq!(result, 18);
}
