//! CRDT Core - Shared CRDT logic for Notion Killer
//!
//! This crate provides the core CRDT functionality used across
//! web (via WASM), backend (native Rust), and mobile (via FFI).

use serde::{Deserialize, Serialize};
use yrs::{Doc, ArrayRef, MapRef, TextRef, Transact, ReadTxn};
use yrs::updates::encoder::Encode;

/// Block types supported by the editor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    BulletList,
    NumberedList,
    TodoList,
    Toggle,
    Quote,
    Callout,
    Code,
    Divider,
    Image,
    Video,
    Embed,
    Table,
    PageLink,
    DatabaseInline,
}

/// A document in our system, wrapping a Yjs Doc
pub struct CRDTDocument {
    doc: Doc,
}

impl CRDTDocument {
    /// Create a new empty document
    pub fn new() -> Self {
        Self { doc: Doc::new() }
    }

    /// Create a document from existing state
    pub fn from_state(state: &[u8]) -> Result<Self, String> {
        let doc = Doc::new();
        {
            let mut txn = doc.transact_mut();
            yrs::updates::decoder::Decode::decode_v1(state)
                .map_err(|e| format!("Failed to decode state: {}", e))
                .and_then(|update| {
                    txn.apply_update(update)
                        .map_err(|e| format!("Failed to apply update: {:?}", e))
                })?;
        }
        Ok(Self { doc })
    }

    /// Get the blocks map
    pub fn blocks(&self) -> MapRef {
        self.doc.get_or_insert_map("blocks")
    }

    /// Get the block order array
    pub fn block_order(&self) -> ArrayRef {
        self.doc.get_or_insert_array("blockOrder")
    }

    /// Get the document title
    pub fn title(&self) -> TextRef {
        self.doc.get_or_insert_text("title")
    }

    /// Encode the current state
    pub fn encode_state(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&yrs::StateVector::default())
    }

    /// Encode state vector for sync
    pub fn encode_state_vector(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.state_vector().encode_v1()
    }

    /// Apply an update from another client
    pub fn apply_update(&self, update: &[u8]) -> Result<(), String> {
        let mut txn = self.doc.transact_mut();
        let decoded = yrs::updates::decoder::Decode::decode_v1(update)
            .map_err(|e| format!("Failed to decode update: {}", e))?;
        txn.apply_update(decoded)
            .map_err(|e| format!("Failed to apply update: {:?}", e))
    }

    /// Get the underlying Yjs Doc for advanced operations
    pub fn inner(&self) -> &Doc {
        &self.doc
    }
}

impl Default for CRDTDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let doc = CRDTDocument::new();
        let state = doc.encode_state();
        assert!(!state.is_empty());
    }

    #[test]
    fn test_sync_documents() {
        let doc1 = CRDTDocument::new();
        let doc2 = CRDTDocument::new();

        // Modify doc1
        {
            let mut txn = doc1.inner().transact_mut();
            let blocks = doc1.blocks();
            blocks.insert(&mut txn, "block1", "test content");
        }

        // Sync to doc2
        let update = doc1.encode_state();
        doc2.apply_update(&update).unwrap();

        // Verify sync
        let txn = doc2.inner().transact();
        let blocks = doc2.blocks();
        assert!(blocks.get(&txn, "block1").is_some());
    }
}
