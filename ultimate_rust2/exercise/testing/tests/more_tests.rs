use testing::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sploosh_splist() {
        assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4)
    }
}
