use h5rio::h5type;
use hdf5_metno as hdf5;

#[h5type]
pub struct DaqEventMeta {
    pub event    : u32,
    pub timestamp: u64,
}
