use dashmap::DashMap;
use lunatic_common_api::control::{ControlInterface, GetModule};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use crate::{local_control::local_control, Process, Signal};

#[derive(Clone)]
pub struct Environment {
    environment_id: u64,
    next_process_id: Arc<AtomicU64>,
    processes: Arc<DashMap<u64, Arc<dyn Process>>>,
    control: ControlInterface,
}

impl Environment {
    pub fn new(id: u64, control: ControlInterface) -> Self {
        Self {
            environment_id: id,
            processes: Arc::new(DashMap::new()),
            next_process_id: Arc::new(AtomicU64::new(1)),
            control,
        }
    }

    pub fn local() -> Self {
        Self::new(1, local_control())
    }

    pub fn get_process(&self, id: u64) -> Option<Arc<dyn Process>> {
        self.processes.get(&id).map(|x| x.clone())
    }

    pub fn send(&self, id: u64, signal: Signal) {
        if let Some(proc) = self.processes.get(&id) {
            proc.send(signal);
        }
    }

    pub fn get_next_process_id(&self) -> u64 {
        self.next_process_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn id(&self) -> u64 {
        self.environment_id
    }

    pub async fn get_module(&self, module_id: u64) {
        self.control.get_module.call(GetModule { module_id }).await;
        todo!()
    }
}
