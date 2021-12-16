// use std::mem::size_of;
use std::fmt::{Display, Formatter, Result as FmtResult};
use xid::{new, Id};

/// BlockId use SonyFlake to generate id with the following properties :
/// 39 bits for time in units of 10 msec
/// 8 bits for a sequence number
/// 16 bits for a machine id

const RAW_LEN: usize = 12;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(remote = "Id")]
pub struct IdDef(pub [u8; RAW_LEN]);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct BlockId(#[serde(with = "IdDef")] Id);

impl BlockId {
    pub fn new() -> Self {
        BlockId(new())
    }

    /// Always return 42 Id value
    pub const fn get_magic() -> Self {
        let mut array: [u8; RAW_LEN] = [0; RAW_LEN];
        array[0] = 42;
        BlockId(Id(array))
    }

    // pub fn from_slice(buf: &[u8]) -> Self {
    //     assert_eq!(buf.len(), size_of::<BlockId>());
    //     let id = BlockId::default();
    //     id.0.copy_from_slice(buf);
    //     id
    // }
}

impl Default for BlockId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::BlockId;

    #[test]
    fn test1() {
        let id = BlockId::new();
        println!("{:?}", id.to_string());
    }
}
