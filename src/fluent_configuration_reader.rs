use {ConfigurationAccessor, ConfigurationReader, ConfigurationWriter};
use fallback::FallbackConfigurationReader;
use copy_on_read::CopyOnReadConfigurationReader;

/// A trait to fluently build a `ConfigurationReader`.
pub trait FluentConfigurationReader: ConfigurationReader {
    /// # Examples
    /// ```
    /// #![feature(never_type, integer_atomics)]
    /// extern crate lz_configuration;
    /// extern crate futures;
    ///
    /// use lz_configuration::{ConfigurationReader, FluentConfigurationReader};
    /// use lz_configuration::closure::ClosureConfigurationReader;
    /// use lz_configuration::memory::MemoryConfigurationAccessor;
    /// use futures::Future;
    /// use futures::future;
    /// use std::sync::atomic::{AtomicU32, Ordering};
    /// use std::sync::Arc;
    ///
    /// #[derive(Clone)]
    /// struct TestConfiguration;
    ///
    /// fn main () {
    ///     let called_count = Arc::new(AtomicU32::new(0));
    ///
    ///     let closure_called_count = called_count.clone();
    ///     let configuration_reader = ClosureConfigurationReader::new(move ||{
    ///         closure_called_count.fetch_add(1, Ordering::SeqCst);
    ///         future::ok::<TestConfiguration, !>(TestConfiguration)
    ///     });
    ///
    ///     let configuration_reader = configuration_reader.with_cache(MemoryConfigurationAccessor::empty());
    ///     configuration_reader.read_configuration().wait().unwrap();
    ///     configuration_reader.read_configuration().wait().unwrap();
    ///
    ///     assert_eq!(called_count.load(Ordering::SeqCst), 1, "The result of the first invocation should have been cached");
    /// }
    /// ```
    fn with_cache<A, R, W>(self,
                        accessor: A)
                        -> FallbackConfigurationReader<R,
                                                       CopyOnReadConfigurationReader<Self, W>,
                                                       fn(&R::Error) -> bool>
        where Self: Sized + 'static,
            A: Into<ConfigurationAccessor<R, W>>,
            R: ConfigurationReader<Configuration = Self::Configuration>,
            W: ConfigurationWriter<Configuration = Self::Configuration> + Send + 'static
    {
        let (reader, writer) = accessor.into().into();

        let copy_configuration_reader = CopyOnReadConfigurationReader::new(self, writer);
        FallbackConfigurationReader::new(reader, copy_configuration_reader)
    }

    fn with_conditional_cache<A, R, W, P>
        (self,
         accessor: A,
         should_fallback: P)
         -> FallbackConfigurationReader<R, CopyOnReadConfigurationReader<Self, W>, P>
        where Self: Sized + 'static,
              A: Into<ConfigurationAccessor<R, W>>,
              R: ConfigurationReader<Configuration = Self::Configuration>,
              W: ConfigurationWriter<Configuration = Self::Configuration> + Send + 'static,
              P: Fn(&R::Error) -> bool + Send + 'static
    {
        let (reader, writer) = accessor.into().into();

        let copy_configuration_reader = CopyOnReadConfigurationReader::new(self, writer);
        FallbackConfigurationReader::new_conditional(reader,
                                                     copy_configuration_reader,
                                                     should_fallback)
    }
}

impl<R> FluentConfigurationReader for R where R: ConfigurationReader {}
