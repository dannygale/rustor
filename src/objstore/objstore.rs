use uuid::Uuid;

pub trait ObjectStore {
    fn put(self, data: &[u8]) -> Uuid;
    fn get(self, uuid: Uuid) -> &'static [u8];
}

