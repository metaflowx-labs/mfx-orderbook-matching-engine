pub mod orders;

pub fn common() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = common();
        assert_eq!(result, true);
    }
}
