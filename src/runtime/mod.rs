use std::sync::Arc;
use tokio::sync::Mutex;

pub mod error_process;
pub mod get_categories;
pub mod process;
pub mod ws_pool;

use process::Process;

use crate::prerun::ArgsParsed;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub ws_pool: ws_pool::WsPool,
    pub process_in_run: Arc<Mutex<Option<Process>>>,
    pub args: ArgsParsed,
}

impl Runtime {
    pub fn new(ws_pool: &ws_pool::WsPool, args: &ArgsParsed) -> Self {
        Runtime {
            process_in_run: Default::default(),
            ws_pool: ws_pool.clone(),
            args: args.clone(),
        }
    }
}
