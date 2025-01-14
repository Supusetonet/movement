use movement_types::{Block, Transaction, AtomicTransactionBundle};

pub trait Sequencer {

    async fn publish(&self, atb: Transaction) -> Result<(), anyhow::Error>;

    async fn wait_for_next_block(&self) -> Result<Option<Block>, anyhow::Error>;

}

pub trait SharedSequencer {

    async fn publish(&self, atb: AtomicTransactionBundle) -> Result<(), anyhow::Error>;

    async fn wait_for_next_block(&self) -> Result<Option<Block>, anyhow::Error>;

}