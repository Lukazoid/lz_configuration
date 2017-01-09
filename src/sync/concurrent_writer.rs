use ConfigurationWriter;
use std::sync::{Mutex, Arc};

/// Wraps a `ConfigurationWriter` for concurrent access.
#[derive(Debug, Default)]
pub struct ConcurrentWriter<W> {
    writer: Arc<Mutex<W>>,
}

impl<W> Clone for ConcurrentWriter<W> {
    fn clone(&self) -> Self {
        // Clone should point at the same `Mutex`
        Self { writer: self.writer.clone() }
    }
}

impl<W: ConfigurationWriter> ConcurrentWriter<W> {
    /// Creates a new `ConcurrentWriter` wrapping the specified `ConfigurationWriter`.
    ///
    /// Use this in situations where concurrent access to the `ConfigurationWriter` is required but the `ConfigurationWriter` does not support concurrent access.
    ///
    /// # Examples
    /// ```
    /// #![feature(integer_atomics)]
    /// extern crate lz_configuration;
    /// extern crate futures;
    ///
    /// use lz_configuration::ConfigurationWriter;
    /// use lz_configuration::closure::ClosureConfigurationWriter;
    /// use lz_configuration::sync::ConcurrentWriter;
    /// use futures::Future;
    /// use futures::future;
    /// use std::sync::atomic::{AtomicU32, Ordering};
    ///
    /// struct TestConfiguration;
    ///
    /// fn main () {
    ///     let called_count = AtomicU32::new(0);
    ///
    ///     // Create a non-concurrent `ConfigurationWriter`
    ///     let writer = ClosureConfigurationWriter::new(|cfg : &TestConfiguration| {
    ///         called_count.fetch_add(1, Ordering::SeqCst);
    ///         future::ok::<(), ()>(())
    ///     });
    ///
    ///     // Two concurrent accessors
    ///     let mut first = ConcurrentWriter::new(writer);
    ///     let mut second = first.clone();
    ///
    ///     // Both can be used to concurrently write, these could be called at any point from anywhere
    ///     let first_write = future::lazy(||first.write_configuration(&TestConfiguration));
    ///     let second_write = future::lazy(||second.write_configuration(&TestConfiguration));
    ///
    ///     first_write.wait().unwrap();
    ///     second_write.wait().unwrap();
    ///
    ///     assert_eq!(called_count.load(Ordering::SeqCst), 2, "Both writers should have resulted in the same closure being called");
    /// }
    /// ```
    pub fn new(writer: W) -> Self {
        Self { writer: Arc::new(Mutex::new(writer)) }
    }
}

impl<W: ConfigurationWriter> ConfigurationWriter for ConcurrentWriter<W> {
    type Configuration = W::Configuration;
    type Error = W::Error;
    type WriteResult = W::WriteResult;

    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult {
        let mut write_guard = self.writer.lock().unwrap();
        let writer = &mut *write_guard;
        writer.write_configuration(configuration)
    }
}