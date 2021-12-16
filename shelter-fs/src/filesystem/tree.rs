use camino::Utf8PathBuf;
use crdt_tree::TreeReplica;
use serde::{Deserialize, Serialize};
use shelter_block::{BlockId, BlockType, ShelterBlock};
use std::sync::{Arc, RwLock};

type TypeId = Utf8PathBuf;
type TypeMeta = BlockId;
type TypeActor = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub(super) id: BlockId,
    pub(super) replica: TreeReplica<TypeId, TypeMeta, TypeActor>, // FileSystem tree (CRDT)
}

impl Tree {
    /// Create new Tree with root path `/`
    pub fn new() -> Self {
        let id = BlockId::new();
        let machine_id = String::from("42");
        let mut replica = TreeReplica::new(machine_id);
        let op = replica.opmove(Utf8PathBuf::new(), id, Utf8PathBuf::from("/"));
        replica.apply_op(op);

        Self {
            id: BlockId::new(),
            replica,
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl ShelterBlock for Tree {
    type ItemBlock = Self;

    fn get_block_id(&self) -> BlockId {
        self.id
    }

    fn get_block_type(&self) -> BlockType {
        BlockType::TREE
    }
}

pub type TreeLock = Arc<RwLock<Tree>>;
