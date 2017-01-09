use ConfigurationWriter;
use futures::Future;
use futures::future::{Map, Join, MapErr};
use either::Either;

#[derive(Debug, Clone, Default)]
pub struct BroadcastConfigurationWriter<F, S> {
    first_writer: F,
    second_writer: S,
}

impl<F, S> BroadcastConfigurationWriter<F, S>
    where F: ConfigurationWriter,
          S: ConfigurationWriter<Configuration = F::Configuration>
{
    pub fn new(first_writer: F, second_writer: S) -> Self {
        Self {
            first_writer: first_writer,
            second_writer: second_writer,
        }
    }
}

impl<F, S> BroadcastConfigurationWriter<F, S> {
    pub fn push_writer_front<W>(self, writer: W) -> BroadcastConfigurationWriter<W, Self>
        where Self: ConfigurationWriter,
              W: ConfigurationWriter<Configuration = <Self as ConfigurationWriter>::Configuration>
    {
        BroadcastConfigurationWriter::new(writer, self)
    }

    pub fn push_writer_back<W>(self, writer: W) -> BroadcastConfigurationWriter<Self, W>
        where Self: ConfigurationWriter,
              W: ConfigurationWriter<Configuration = <Self as ConfigurationWriter>::Configuration>
    {
        BroadcastConfigurationWriter::new(self, writer)
    }
}

impl<F, S> ConfigurationWriter for BroadcastConfigurationWriter<F, S>
    where F: ConfigurationWriter,
          S: ConfigurationWriter<Configuration = F::Configuration>
{
    type Configuration = F::Configuration;
    type Error = Either<F::Error, S::Error>;
    type WriteResult = Map<Join<MapErr<F::WriteResult, fn(F::Error) -> Self::Error>,
             MapErr<S::WriteResult, fn(S::Error) -> Self::Error>>,
        fn(((), ())) -> ()>;

    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult {
        let first_write = self.first_writer
            .write_configuration(configuration)
            .map_err(Either::Left as fn(F::Error) -> Self::Error);

        let second_write = self.second_writer
            .write_configuration(configuration)
            .map_err(Either::Right as fn(S::Error) -> Self::Error);

        first_write.join(second_write)
            .map(drop)
    }
}