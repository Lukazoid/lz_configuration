use {ConfigurationReader, ConfigurationWriter, ConfigurationAccessor};
use std::sync::{Arc, RwLock};
use futures::IntoFuture;
use futures::future::{self, Ok as OkFuture, FutureResult};

mod memory_configuration_read_error;
pub use self::memory_configuration_read_error::*;

#[derive(Debug, Clone, Default)]
pub struct MemoryConfigurationAccessor<C> {
    configuration: Arc<RwLock<Option<C>>>,
}

impl<C> MemoryConfigurationAccessor<C>{
    pub fn empty() -> Self{
        Self{
            configuration: Arc::new(RwLock::new(None))
        }
    }

    pub fn new<V:Into<Option<C>>>(configuration:V) -> Self{
        Self{
            configuration: Arc::new(RwLock::new(configuration.into()))
        }
    }
}

impl<C> From<MemoryConfigurationAccessor<C>> for ConfigurationAccessor<MemoryConfigurationAccessor<C>, MemoryConfigurationAccessor<C>>
    where C : Clone + Send + 'static
{
    fn from(value: MemoryConfigurationAccessor<C>) -> Self{
        ConfigurationAccessor::new(value.clone(), value)
    }
}

impl<C> ConfigurationReader for MemoryConfigurationAccessor<C>
    where C: Clone + Send + 'static
{
    type Configuration = C;
    type Error = MemoryConfigurationReadError;
    type ReadResult = FutureResult<Self::Configuration, Self::Error>;

    fn read_configuration(&self) -> Self::ReadResult {
        let read_lock = self.configuration.read().unwrap();
        read_lock.clone().ok_or(MemoryConfigurationReadError::NoConfiguration).into_future()
    }
}

impl<C> ConfigurationWriter for MemoryConfigurationAccessor<C>
    where C: Clone + Send + 'static
{
    type Configuration = C;
    type Error = !;
    type WriteResult = OkFuture<(), Self::Error>;

    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult {

        let mut write_lock = self.configuration.write().unwrap();
        *write_lock = Some(configuration.clone());
        
        future::ok(())
    }
}

fn _static_assertions(){
    fn _assert_send<T:Send>(){}
    fn _assert_sync<T:Sync>(){}
    
    _assert_send::<MemoryConfigurationAccessor<()>>();
    _assert_sync::<MemoryConfigurationAccessor<()>>();
}


#[cfg(test)]
mod tests {
    use {ConfigurationReader, ConfigurationWriter};
    use futures::Future;
    use memory::{MemoryConfigurationAccessor, MemoryConfigurationReadError};

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct TestConfiguration(pub i32);

    #[test]
    fn read_configuration_before_write_returns_error() {
        // Arrange
        let accessor = MemoryConfigurationAccessor::<TestConfiguration>::default();

        // Act
        let error = accessor.read_configuration().wait().unwrap_err();

        // Assert
        assert_eq!(error, MemoryConfigurationReadError::NoConfiguration);
    }

    #[test]
    fn read_configuration_after_write_returns_configuration() {
        // Arrange
        let mut accessor = MemoryConfigurationAccessor::<TestConfiguration>::default();
        
        accessor.write_configuration(&TestConfiguration(5)).wait().unwrap();

        // Act
        let configuration = accessor.read_configuration().wait().unwrap();

        // Assert
        assert_eq!(configuration, TestConfiguration(5));
    }

    #[test]
    fn clone_references_same_written_configuration() {
        // Arrange
        let mut accessor = MemoryConfigurationAccessor::<TestConfiguration>::default();
        accessor.write_configuration(&TestConfiguration(5)).wait().unwrap();
        
        // Act
        let cloned_accessor = accessor.clone();

        // Assert
        let first_configuration = accessor.read_configuration().wait().unwrap();
        let second_configuration = cloned_accessor.read_configuration().wait().unwrap();

        assert_eq!(first_configuration, TestConfiguration(5));
        assert_eq!(second_configuration, TestConfiguration(5));
    }

    #[test]
    fn write_updates_cloned_accessors() {
        // Arrange
        let mut accessor = MemoryConfigurationAccessor::<TestConfiguration>::default();
        let cloned_accessor = accessor.clone();

        // Act
        accessor.write_configuration(&TestConfiguration(5)).wait().unwrap();

        // Assert
        let first_configuration = accessor.read_configuration().wait().unwrap();
        let second_configuration = cloned_accessor.read_configuration().wait().unwrap();

        assert_eq!(first_configuration, TestConfiguration(5));
        assert_eq!(second_configuration, TestConfiguration(5));
    }
}