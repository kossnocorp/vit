use crate::prelude::*;

pub struct VitSourceFile {
    pub revision: String,
    pub bytes: Vec<u8>,
}

impl VitFileWritable for VitSourceFile {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
