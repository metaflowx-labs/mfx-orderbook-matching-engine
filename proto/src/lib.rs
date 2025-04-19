pub fn proto() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = proto();
        assert_eq!(result, true);
    }
}
