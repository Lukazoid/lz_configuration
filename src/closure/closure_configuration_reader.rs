use ConfigurationReader;
use futures::{Future};
use futures::future::{self, Ok as OkFuture};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ClosureConfigurationReader<F> {
    factory: F
}

fn default_factory<C: Default>() -> OkFuture<C, !> {
    future::ok(C::default())
}

impl<C: Default> Default for ClosureConfigurationReader<fn() -> OkFuture<C, !>> {
    fn default() -> Self {
        Self {
            factory: default_factory::<C> as fn() -> OkFuture<C, !>,
        }
    }
}

impl<F, R> ClosureConfigurationReader<F>
    where F: Fn() -> R,
          R: Future
{
    pub fn new(default_factory: F) -> Self {
        Self {
            factory: default_factory,
        }
    }
}

impl<F, R> ConfigurationReader for ClosureConfigurationReader<F>
    where F: Fn() -> R,
          R: Future + Send + 'static,
          R::Item: Send + 'static,
          R::Error: Debug + Send + 'static
{
    type Configuration = R::Item;
    type Error = R::Error;
    type ReadResult = R;

    fn read_configuration(&self) -> Self::ReadResult {
        (self.factory)()
    }
}

#[cfg(test)]
mod tests {
    use super::ClosureConfigurationReader;
    use ConfigurationReader;
    use futures::Future;
    use futures::future::{self};

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    struct TestConfiguration(pub i32);

    #[test]
    fn default_read_configuration_returns_default() {
        // Arrange
        let reader = ClosureConfigurationReader::default();

        // Act
        let configuration : TestConfiguration = reader.read_configuration().wait().unwrap();

        // Assert
        assert_eq!(configuration, TestConfiguration::default());
    }


    #[test]
    fn factory_read_configuration_returns_factory() {
        // Arrange
        let reader = ClosureConfigurationReader::new(|| future::ok::<TestConfiguration, !>(TestConfiguration(5)));

        // Act
        let configuration = reader.read_configuration().wait().unwrap();

        // Assert
        assert_eq!(configuration, TestConfiguration(5));
    }
}