#[cfg(not(target_os = "windows"))]
compile_error!("This project only targets the Windows platform");

mod profile;
mod status;
pub mod ipc_srv {
    tonic::include_proto!("ipc_interface");
}

pub use profile::Profile;
pub use status::Status;

pub const APP_NAME: &str = "The Box";
