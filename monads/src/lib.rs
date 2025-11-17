mod option_monad;
mod result_monad;
mod result_monad_error;

pub use option_monad::OptionMonad;
pub use result_monad::ResultMonad;
pub use result_monad_error::ResultMonadError;

#[cfg(test)]
mod test_errors;
