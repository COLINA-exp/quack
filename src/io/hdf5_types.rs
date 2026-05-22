use hdf5_metno as hdf5;

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct DaqEventMeta {
    pub event    : u32,
    pub timestamp: u64,
}
