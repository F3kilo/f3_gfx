use crate::resource_loader::TexUnloader;

pub struct Tex {
    id: TexId,
    unloader: TexUnloader,
}

impl Tex {
    pub fn new(id: TexId, unloader: TexUnloader) -> Self {
        Self { id, unloader }
    }
}

impl Drop for Tex {
    fn drop(&mut self) {
        self.unloader.unload(self.id)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TexId(u64);

impl From<u64> for TexId {
    fn from(i: u64) -> Self {
        Self(i)
    }
}
