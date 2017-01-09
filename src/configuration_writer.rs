use futures::Future;
use std::fmt::Debug;

/// The trait for types which write a configuration.
pub trait ConfigurationWriter {
    /// The type of the configuration.
    type Configuration: Send + 'static;

    /// The type of the error, this may be `!` if no error could occur.
    type Error: Debug + Send + 'static;

    /// The type of the result of write_configuration.
    type WriteResult: Future<Item = (), Error = Self::Error> + Send + 'static;

    // TODO LH When impl return types are supported for traits this could return impl Future<Item = Self::Configuration, Error = Self::Error>

    /// Asynchronously writes the configuration returning an error in cases of failure.
    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult;
}