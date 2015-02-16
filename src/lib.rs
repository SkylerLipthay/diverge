/// Aborts the outer function if given `Some` by returning its value, or continues the function
/// if given `None`. This is particularly useful for understandable extension injection site
/// specification.
///
/// # Example
///
/// ```ignore
/// fn reject_if_wrong_password(password: &str) -> Option<bool> {
///     match password {
///         "let_me_in" => None,
///         _ => {
///             // Hide the responsibility of recording the failure from `test_password()`
///             record_failed_login_attempt();
///             Some(false)
///         },
///     }
/// }
///
/// fn test_password(password: &str) -> bool {
///     diverge!(reject_if_wrong_password(password));
///     true
/// }
/// ```
#[macro_export]
macro_rules! diverge {
    ($e:expr) => ({
        if let Some(e) = $e {
            return e;
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_conclude() {
        assert_eq!(conclude_or_20(Some(10)), 10);
    }

    #[test]
    fn test_proceed() {
        assert_eq!(conclude_or_20(None), 20);
    }

    fn conclude_or_20(fork: Option<u32>) -> u32 {
        diverge!(fork);
        20
    }
}
