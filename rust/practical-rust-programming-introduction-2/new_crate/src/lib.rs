pub mod module_a;
mod module_b;

/// Thi function adds 2 numbers.
///
/// # Example
///
/// ```
/// use new_crate::add
///
/// add(1, 2);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
