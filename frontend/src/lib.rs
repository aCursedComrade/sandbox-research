pub(crate) mod sample_service {
    tonic::include_proto!("ipc_interface");
}

mod app;
pub use app::Frontend;

pub(crate) mod comms;
pub(crate) mod util;
