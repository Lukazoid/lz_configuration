use {ConfigurationReader, ConfigurationWriter, CopyConfigurationError};
use sync::ConcurrentWriter;
use futures::{Future, BoxFuture};

#[derive(Debug, Default, Clone)]
pub struct CopyOnReadConfigurationReader<R, W> {
    reader: R,
    writer: ConcurrentWriter<W>,
}

impl<R, W> CopyOnReadConfigurationReader<R, W>
    where R: ConfigurationReader,
          W: ConfigurationWriter<Configuration = R::Configuration>
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: reader,
            writer: ConcurrentWriter::new(writer),
        }
    }
}

impl<R, W> ConfigurationReader for CopyOnReadConfigurationReader<R, W>
    where R: ConfigurationReader + 'static,
          W: ConfigurationWriter<Configuration = R::Configuration> + Send + 'static
{
    type Configuration = R::Configuration;
    type Error = CopyConfigurationError<R::Error, W::Error>;
    type ReadResult = BoxFuture<Self::Configuration, Self::Error>;

    fn read_configuration(&self) -> Self::ReadResult {
        ::copy_configuration(&self.reader, self.writer.clone()).boxed()
    }
}