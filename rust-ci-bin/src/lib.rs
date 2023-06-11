use rust_ci_helpers::add4;
use rust_ci_lib::add8;

pub fn add12(x: usize) -> usize {
    add4(add8(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add12(2);
        assert_eq!(result, 14);
    }
}
