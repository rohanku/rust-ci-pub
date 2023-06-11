pub fn add4(x: usize) -> usize {
    x + 4
}

pub fn add5(x: usize) -> usize {
    x + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add4(2);
        assert_eq!(result, 6);
    }
}
