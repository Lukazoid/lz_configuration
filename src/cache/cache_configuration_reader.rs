use {ConfigurationReader, ConfigurationWriter, CopyConfigurationError};
use memory::MemoryConfigurationAccessor;
use futures::{Future, BoxFuture};
use std::clone::Clone;
use copy_configuration;

#[derive(Debug)]
pub struct CacheConfigurationReader<C, R> {
    memory: MemoryConfigurationAccessor<C>,
    reader: R,
}

impl<R> CacheConfigurationReader<R::Configuration, R>
    where R: ConfigurationReader
{
    pub fn new(reader: R) -> Self {
        Self {
            memory: MemoryConfigurationAccessor::empty(),
            reader: reader,
        }
    }
}

impl<R> ConfigurationReader for CacheConfigurationReader<R::Configuration, R>
    where R: ConfigurationReader + Send + Clone + 'static,
          R::Configuration: Clone + Sync
{
    type Configuration = R::Configuration;
    type Error = CopyConfigurationError<R::Error, <MemoryConfigurationAccessor<Self::Configuration> as ConfigurationWriter>::Error>;
    type ReadResult = BoxFuture<Self::Configuration, Self::Error>;

    fn read_configuration(&self) -> Self::ReadResult {
        let reader_clone = self.reader.clone();
        let memory_clone = self.memory.clone();

        self.memory
            .read_configuration()
            .or_else(move |_| copy_configuration(&reader_clone, memory_clone))
            .boxed()
    }
}