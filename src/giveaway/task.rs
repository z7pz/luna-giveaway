use std::sync::Arc;

use futures::lock::Mutex;
use tokio::task::JoinHandle;

use super::giveaway::Giveaway;

#[derive(Debug)]

pub struct GiveawayTask {
    pub giveaway:  Arc<Mutex<Giveaway>>,
    pub task: JoinHandle<()>,
}