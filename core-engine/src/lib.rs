pub fn core_engine() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = core_engine();
        assert_eq!(result, true);
    }
}
