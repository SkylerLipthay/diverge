/// A type that is used to determine whether the outer function should continue execution or end
/// with a conclusion value.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Fork<T> {
    Conclude(T),
    Proceed,
}

/// Aborts the outer function if given `Conclude` by returning its value, or continues the function
/// given if `Proceed`. This is particularly useful for understandable extension injection site
/// specification.
///
/// # Example
///
/// ```ignore
/// fn reject_if_wrong_password(password: &str) -> Fork<bool> {
///     match password {
///         "let_me_in" => Fork::Proceed,
///         _ => {
///             // Hide the responsibility of recording the failure from `test_password()`
///             record_failed_login_attempt();
///             Fork::Conclude(false)
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
        use $crate::Fork::Conclude;

        if let Conclude(e) = $e {
            return e;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::Fork;

    #[test]
    fn test_conclude() {
        assert_eq!(conclude_or_20(Fork::Conclude(10)), 10);
    }

    #[test]
    fn test_proceed() {
        assert_eq!(conclude_or_20(Fork::Proceed), 20);
    }

    fn conclude_or_20(fork: Fork<u32>) -> u32 {
        diverge!(fork);
        20
    }
}
