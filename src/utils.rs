pub mod fns {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, nafsy::plus_one(five));
    /// ```
    pub fn plus_one(num: i32) -> i32 {
        num + 1
    }

    /// Reduces one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(4, nafsy::minus_one(five));
    /// ```
    pub fn minus_one(num: i32) -> i32 {
        num - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(6, fns::plus_one(5));
    }

    #[test]
    fn it_reduces_one() {
        assert_eq!(4, fns::minus_one(5));
    }
}
