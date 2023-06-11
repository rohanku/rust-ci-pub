use rust_ci_helpers::{add4, add5};

pub fn add8(x: usize) -> usize {
    add4(add4(x))
}

pub fn add10(x: usize) -> usize {
    add5(add5(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add8(2);
        assert_eq!(result, 10);
    }
}
