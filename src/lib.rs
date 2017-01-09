#![feature(more_struct_aliases, conservative_impl_trait, never_type, associated_type_defaults)]

extern crate futures;
#[macro_use]
extern crate quick_error;
extern crate either;

mod configuration_reader;
pub use self::configuration_reader::*;

mod configuration_writer;
pub use self::configuration_writer::*;

mod configuration_accessor;
pub use self::configuration_accessor::*;

mod configuration_target;
pub use self::configuration_target::*;

mod copy_configuration_error;
pub use self::copy_configuration_error::*;

use std::borrow::BorrowMut;
use futures::{Future};

pub fn copy_configuration<R, W, BW>
    (reader: &R,
     mut writer: BW)
     -> impl Future<Item = R::Configuration, Error = CopyConfigurationError<R::Error, W::Error>> + Send + 'static
    where R: ConfigurationReader,
          W: ConfigurationWriter<Configuration = R::Configuration>,
          BW : BorrowMut<W> + Send + 'static
{
    reader.read_configuration()
        .map_err(CopyConfigurationError::ReadError)
        .and_then(move |c| {
            let writer = writer.borrow_mut();
            writer.write_configuration(&c)
                .map(move |_| c)
                .map_err(CopyConfigurationError::WriteError)
        })
}

pub mod sync;
pub mod memory;
pub mod broadcast;
pub mod fallback;
pub mod cache;
pub mod closure;
pub mod copy_on_read;

mod fluent_configuration_reader;
pub use self::fluent_configuration_reader::*;