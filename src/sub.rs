pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn it_wont_work() {
        let result = sub(2, 2);
        assert_ne!(result, 5);
    }
}