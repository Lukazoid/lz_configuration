use ConfigurationWriter;
use futures::Future;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ClosureConfigurationWriter<F, C> {
    callback: F,
    phantom_data: PhantomData<C>,
}

impl<F, C, R> ClosureConfigurationWriter<F, C>
    where F: FnMut(&C) -> R,
          R: Future
{
    pub fn new(callback: F) -> Self {
        Self {
            callback: callback,
            phantom_data: Default::default(),
        }
    }
}

impl<F, C, R> ConfigurationWriter for ClosureConfigurationWriter<F, C>
    where F: FnMut(&C) -> R,
          C: Send + 'static,
          R: Future<Item = ()> + Send + 'static,
          R::Error: Debug + Send + 'static
{
    type Configuration = C;
    type Error = R::Error;
    type WriteResult = R;

    fn write_configuration(&mut self, configuration: &Self::Configuration) -> Self::WriteResult {
        (self.callback)(configuration)
    }
}