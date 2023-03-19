//! A handy extension trait for `Result` that brings `Result::inspect_err` to stable Rust in the form of `InspectError::inspect_error`.
//!
//! Instead of this...
//!
//! ```rs
//! // ...
//!     .map_err(|err| {
//!         eprintln!("Something went wrong: '{err}'.");
//!         err
//!     })
//! // ...
//! ```
//!
//! ...write this:
//!
//! ```rs
//! // ...
//!     .inspect_error(|err| eprintln!("Something went wrong: '{err}'."))
//! // ...
//! ```
//!
//! # Usage
//!
//! Bring the `InspectError` trait into scope. This will let you call `.inspect_error()` on a `Result`.
//!
//!
//! ```rs
//! use inspect_error::InspectError;
//!
//! fn read_magic_number_from_db() -> Result<i32, &'static str> {
//!     // Pretend this is a real `Result`.
//!     let magic_number = Err("couldn't connect to the database")
//!         .inspect_error(|err| eprintln!("Something went wrong: '{err}'."))?;
//!
//!     // Do something with the magic number.
//!
//!     Ok(magic_number)
//! }
//!
//! let output = read_magic_number_from_db();
//! assert_eq!(output, Err("couldn't connect to the database"));
//! ```

/// The core trait of this crate. Implemented for `Result`.
pub trait InspectError<E> {
    #[must_use]
    fn inspect_error(self, inspect: impl FnOnce(&E)) -> Self;
}

impl<T, E> InspectError<E> for Result<T, E> {
    /// Call the provided closure with a reference to the contained error (*if* the
    /// `Result` is an error) and then return the `Result`. Mainly intended for
    /// logging errors in a more natural manner than using `Result::map_err` and
    /// manually returning the error.
    ///
    /// ```
    /// use inspect_error::InspectError;
    ///
    /// fn read_magic_number_from_db() -> Result<i32, &'static str> {
    ///     // Pretend this is a real `Result`.
    ///     let magic_number = Err("couldn't connect to the database")
    ///         .inspect_error(|err| eprintln!("Something went wrong: '{err}'."))?;
    ///
    ///     // Do something with the magic number.
    ///
    ///     Ok(magic_number)
    /// }
    ///
    /// let output = read_magic_number_from_db();
    /// assert_eq!(output, Err("couldn't connect to the database"));
    /// ```
    fn inspect_error(self, inspect: impl FnOnce(&E)) -> Self {
        if let Err(ref error) = self {
            (inspect)(error);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::InspectError;

    #[test]
    fn closure_is_called() {
        let error_code = 42;
        let mut output = None;

        let _: Result<i32, _> =
            Err(error_code).inspect_error(|magic_error_code| {
                output = Some(*magic_error_code);
            });

        assert_eq!(output, Some(error_code));
    }

    #[test]
    fn result_is_returned_unchanged() {
        let error_code = 42;
        let result: Result<i32, _> = Err(error_code).inspect_error(|_| ());
        assert_eq!(result, Err(error_code));
    }
}
