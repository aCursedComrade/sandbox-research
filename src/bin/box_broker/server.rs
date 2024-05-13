use crate::{
    proc,
    state::{ListUtils, ManagedList},
};
use sandbox_research::ipc_srv::{
    ipc_wire_server::IpcWire, EchoRequest, EchoResponse, SpawnRequest, SpawnResponse, StopRequest, StopResponse,
};
use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};
use tonic::{Request, Response, Status};
use windows_sys::Win32::Foundation::BOOL;

#[derive(Default)]
pub struct Service {
    managed: Arc<Mutex<ManagedList>>,
}

impl Service {
    pub extern "system" fn _process_wait_callback(_pid_ref: *const c_void, _fired: BOOL) {
        todo!()
    }
}

#[tonic::async_trait]
impl IpcWire for Service {
    /// Server test routine
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Got an echo message: {:?}", &req.payload);

        let message = EchoResponse { message: req.payload };

        Ok(Response::new(message))
    }

    /// Process spawn routine
    async fn spawn(&self, request: Request<SpawnRequest>) -> Result<Response<SpawnResponse>, Status> {
        let req = request.into_inner();
        let mut list = self.managed.lock().unwrap();

        if let Some(profile) = req.profile {
            tracing::info!("Got a spawn request for: {:?}", &profile.name);

            let id = profile.id;
            let mut message = SpawnResponse { error: true, id, pid: 0 };

            if let Ok(profile) = proc::spawn(profile.clone().into()) {
                // TODO WMI callback should be registered here for process termination

                message.error = false;
                message.id = profile.conf.id;
                message.pid = profile.conf.pid;

                tracing::info!("Process spawned successfully!: {:?}", profile);
                list.add_profile(profile);
            } else {
                tracing::error!("Failed to spawn process: {:?}", profile);
            };

            Ok(Response::new(message))
        } else {
            tracing::error!("Invalid spawn request: {:?}", &req);
            Err(Status::invalid_argument("Invalid profile data"))
        }
    }

    /// Process stop routine
    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let req = request.into_inner();
        let mut list = self.managed.lock().unwrap();
        tracing::info!("Got a stop request: {:?}", &req.id);

        let id = req.id;
        let mut message = StopResponse { error: false, id };

        if let Some(conf) = list.get(&id) {
            if !proc::stop(conf.h_process) {
                tracing::warn!("Failed to terminate process: {}", &conf.conf.pid);
                message.error = true;
            }

            list.remove(&id);
            Ok(Response::new(message))
        } else {
            tracing::error!("Invalid stop request: {:?}", &req);
            Err(Status::invalid_argument("Invalid id"))
        }
    }
}
