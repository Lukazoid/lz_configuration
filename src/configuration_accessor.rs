use super::{ConfigurationReader, ConfigurationWriter};

/// A struct which encapsulates both a `ConfigurationReader` and a `ConfigurationWriter`.
#[derive(Debug, Default, Clone)]
pub struct ConfigurationAccessor<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> ConfigurationAccessor<R, W>
    where R: ConfigurationReader,
          W: ConfigurationWriter<Configuration = R::Configuration>
{
    /// Creates a new `ConfigurationAccessor<R, W>` using the specified `ConfigurationReader` and `ConfigurationWriter`
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: reader,
            writer: writer,
        }
    }
}


impl<R, W> Into<(R, W)> for ConfigurationAccessor<R, W> {
    fn into(self) -> (R, W) {
        (self.reader, self.writer)
    }
}

impl<R, W> From<(R, W)> for ConfigurationAccessor<R, W>
    where R: ConfigurationReader,
          W: ConfigurationWriter<Configuration = R::Configuration>
{
    fn from(value: (R, W)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<R, W> ConfigurationReader for ConfigurationAccessor<R, W>
    where R: ConfigurationReader
{
    type Configuration = R::Configuration;
    type Error = R::Error;
    type ReadResult = R::ReadResult;

    fn read_configuration(&self) -> Self::ReadResult {
        self.reader.read_configuration()
    }
}

impl<R, W> ConfigurationWriter for ConfigurationAccessor<R, W>
    where W: ConfigurationWriter
{
    type Configuration = W::Configuration;
    type Error = W::Error;
    type WriteResult = W::WriteResult;

    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult {
        self.writer.write_configuration(configuration)
    }
}
