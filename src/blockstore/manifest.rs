use std::collections::Vec
use uuid::Uuid;

use crate::object::Object;

type ObjectID = Uuid;

struct ManifestEntry {
    lba: u64,
    n_blocks: u32,
}

pub struct Manifest {
    objid: ObjectID,
    blocks: Vec<ManifestEntry>,
}
