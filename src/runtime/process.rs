use anyhow::Result;
use jakmall_client::{fetcher_models::category::Parent, ClientCategories};
use serde::Serialize;

use super::error_process::ErrorProcess;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]

pub enum ProcessStatus {
    Done,
    Pending,
    Error,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessName {
    GetCategories,
}

#[derive(Debug, Serialize, Clone)]
pub struct Process {
    pub id: String,
    pub name: ProcessName,
    pub status: ProcessStatus,
    pub error: Option<ErrorProcess>,
}

// Process factory
impl Process {
    pub fn get_categories() -> Self {
        Process {
            id: nanoid::nanoid!(16),
            name: ProcessName::GetCategories,
            status: ProcessStatus::Pending,
            error: None,
        }
    }

    pub fn make_done(&self) -> Self {
        Process {
            id: self.id.clone(),
            name: self.name.clone(),
            status: ProcessStatus::Done,
            error: None,
        }
    }

    pub fn make_error(&self, err: ErrorProcess) -> Self {
        Process {
            id: self.id.clone(),
            name: self.name.clone(),
            status: ProcessStatus::Error,
            error: Some(err),
        }
    }
}

impl Process {
    pub async fn get_categories_run() -> Result<Vec<Parent>> {
        let client = ClientCategories::new()?;
        let cats = client.get_categories().await?;

        Ok(cats)
    }
}
