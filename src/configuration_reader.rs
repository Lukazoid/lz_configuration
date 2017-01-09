use futures::Future;
use std::fmt::Debug;

/// The trait for types which read a configuration.
pub trait ConfigurationReader {
    /// The type of the configuration.
    type Configuration: Send + 'static;

    /// The type of the error, this may be `!` if no error could occur.
    type Error: Debug + Send + 'static;

    /// The type of the result of read_configuration.
    type ReadResult: Future<Item = Self::Configuration, Error = Self::Error> + Send + 'static;

    // TODO LH When impl return types are supported for traits this could return impl Future<Item = Self::Configuration, Error = Self::Error>

    /// Asynchronously reads the configuration returning an error in cases of failure.
    fn read_configuration(&self) -> Self::ReadResult;
}
