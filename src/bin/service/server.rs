use crate::{
    proc,
    state::{ListUtils, ManagedList},
};
use sandbox_research::ipc_srv::{
    ipc_wire_server::IpcWire, EchoRequest, EchoResponse, SpawnRequest, SpawnResponse, StopRequest, StopResponse,
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct Service {
    managed: Arc<Mutex<ManagedList>>,
}

#[tonic::async_trait]
impl IpcWire for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let req = request.into_inner();
        tracing::info!("Got an echo message: {:?}", &req.payload);

        let message = EchoResponse { message: req.payload };

        Ok(Response::new(message))
    }

    async fn spawn(&self, request: Request<SpawnRequest>) -> Result<Response<SpawnResponse>, Status> {
        let req = request.into_inner();
        let mut list = self.managed.lock().unwrap();

        if let Some(profile) = req.profile {
            tracing::info!("Got a spawn request for: {:?}", &profile.name);

            let id = profile.id;
            let mut message = SpawnResponse { error: true, id, pid: 0 };

            if let Ok(profile) = proc::spawn(profile.clone().into()) {
                message.error = false;
                message.id = profile.conf.id;
                message.pid = profile.conf.pid;
                list.add_profile(profile);

                tracing::info!("Active profiles: {:?}", self.managed);
            } else {
                tracing::warn!("Failed to spawn process: {:?}", profile);
            };

            Ok(Response::new(message))
        } else {
            tracing::error!("Invalid spawn request: {:?}", &req);
            Err(Status::invalid_argument("Invalid profile data"))
        }
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let req = request.into_inner();
        let mut list = self.managed.lock().unwrap();
        tracing::info!("Got a stop request: {:?}", &req.id);

        let id = req.id;
        let mut message = StopResponse { error: true, id: 0 };

        if let Some(conf) = list.get(&id) {
            if !proc::stop(conf.h_process) {
                tracing::warn!("Failed to terminate process: {} (ID: {})", &conf.conf.pid, &conf.conf.id);
                return Ok(Response::new(message));
            }

            message.error = false;
            message.id = id;
            list.remove(&id);
            Ok(Response::new(message))
        } else {
            tracing::error!("Invalid stop request: {:?}", &req);
            Err(Status::invalid_argument("Invalid id"))
        }
    }
}
