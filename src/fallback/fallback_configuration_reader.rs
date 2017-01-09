use ConfigurationReader;
use super::FallbackConfigurationReadError;
use futures::{BoxFuture, Future};
use futures::future::err as err_future;
use std::sync::Arc;
use either::Either;

#[derive(Debug)]
pub struct FallbackConfigurationReader<R, F, P> {
    reader: R,
    fallback: Arc<F>,
    should_fallback: P,
}

fn predicate_true<T>(_: &T) -> bool {
    true
}

impl<R, F> FallbackConfigurationReader<R, F, fn(&R::Error) -> bool>
    where R: ConfigurationReader,
          F: ConfigurationReader<Configuration = R::Configuration>
{
    pub fn new(reader: R, fallback: F) -> Self {
        Self::new_conditional(reader, fallback, predicate_true as fn(&R::Error) -> bool)
    }
}

impl<R, F, P> FallbackConfigurationReader<R, F, P> {
    pub fn push_reader_front<NR>(self,
                                 reader: NR)
                                 -> FallbackConfigurationReader<NR, Self, fn(&NR::Error) -> bool>
        where Self: ConfigurationReader,
            NR: ConfigurationReader<Configuration = <Self as ConfigurationReader>::Configuration>
    {
        FallbackConfigurationReader::new(reader, self)
    }

    pub fn push_conditional_reader_front<NR, NP>(self,
                                                 reader: NR,
                                                 should_fallback: NP)
                                                 -> FallbackConfigurationReader<NR, Self, NP>
        where Self: ConfigurationReader,
            NR: ConfigurationReader<Configuration = <Self as ConfigurationReader>::Configuration>,
            NP: Fn(&NR::Error) -> bool
    {
        FallbackConfigurationReader::new_conditional(reader, self, should_fallback)
    }

    pub fn push_reader_back<NR>(self,
                                reader: NR)
                                -> FallbackConfigurationReader<Self, NR, fn(&<Self as ConfigurationReader>::Error) -> bool>
        where Self: ConfigurationReader,
            NR: ConfigurationReader<Configuration = <Self as ConfigurationReader>::Configuration>

    {
        FallbackConfigurationReader::new(self, reader)
    }

    pub fn push_conditional_reader_back<NR, NP>(self,
                                                reader: NR,
                                                should_fallback: NP)
                                                -> FallbackConfigurationReader<Self, NR, NP>
        where Self: ConfigurationReader,
            NR: ConfigurationReader<Configuration = <Self as ConfigurationReader>::Configuration>,
            NP: Fn(&<Self as ConfigurationReader>::Error) -> bool
    {
        FallbackConfigurationReader::new_conditional(self, reader, should_fallback)
    }
}

impl<R, F, P> FallbackConfigurationReader<R, F, P>
    where R: ConfigurationReader,
          F: ConfigurationReader<Configuration = R::Configuration>,
          P: Fn(&R::Error) -> bool
{
    pub fn new_conditional(reader: R, fallback: F, should_fallback: P) -> Self {
        Self {
            reader: reader,
            fallback: Arc::new(fallback),
            should_fallback: should_fallback,
        }
    }
}

impl<R, F, P> ConfigurationReader for FallbackConfigurationReader<R, F, P>
    where R: ConfigurationReader + 'static,
          F: ConfigurationReader<Configuration = R::Configuration> + Send + Sync + 'static,
          P: Fn(&R::Error) -> bool + Send + Copy + 'static
{
    type Configuration = R::Configuration;
    type Error = FallbackConfigurationReadError<R::Error, Option<F::Error>>;
    type ReadResult = BoxFuture<Self::Configuration, Self::Error>;

    fn read_configuration(&self) -> Self::ReadResult {
        let fallback = self.fallback.clone();
        let should_fallback = self.should_fallback;

        self.reader
            .read_configuration()
            .or_else(move |e| {

                let fallback_future = if (should_fallback)(&e) {
                    let fallback_future = fallback.read_configuration()
                        .map_err(move |fb_err| {
                            FallbackConfigurationReadError::new(e, Some(fb_err))
                        });

                    Either::Left(fallback_future)
                } else {
                    Either::Right(err_future(FallbackConfigurationReadError::new(e, None)))
                };

                fallback_future.either(Future::boxed, Future::boxed)
            })
            .boxed()
    }
}
