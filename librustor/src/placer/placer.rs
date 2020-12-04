
use crate::object::{Manifest, ManifestLocation};

pub trait PlacesObjects {
    fn place(&self, data: [u8]) -> RResult<Manifest>;
}

