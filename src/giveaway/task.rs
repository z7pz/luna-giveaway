use std::sync::Arc;

use tokio::task::JoinHandle;

use super::giveaway::Giveaway;

#[derive(Debug)]

pub struct GiveawayTask {
    pub giveaway: Arc<Giveaway>,
    pub task: JoinHandle<()>,
}