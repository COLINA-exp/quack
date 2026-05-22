mod hdf5;
mod daq;
pub mod hdf5_types;

pub use hdf5::read_hdf5;
pub use hdf5::ScalarHdf5Writer;
pub use hdf5:: ArrayHdf5Writer;

pub use daq::read_daq_file;
pub use daq::read_waveform_length;

use std::io;
pub trait Writer<T> {
    fn write(&self, value: T) -> io::Result<()>;

    fn write_many(&self, values: Vec<T>) -> io::Result<()> {
        for v in values.into_iter() {
            self.write(v)?;
        }
        Ok(())
    }
}
