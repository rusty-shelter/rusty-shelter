/// Block address
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BlockAddress {
    address: String,
    len: usize,
    offset: usize,
}

impl BlockAddress {
    pub fn new(address: String, len: usize, offset: usize) -> Self {
        Self {
            address,
            len,
            offset,
        }
    }

    #[inline]
    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    #[inline]
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    #[inline]
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn end_offset(&self) -> usize {
        self.offset() + self.len()
    }
}
