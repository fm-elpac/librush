pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let r = add(2, 2);
        assert_eq!(r, 4);
    }
}
