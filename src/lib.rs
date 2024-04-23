mod profile;
mod status;
pub mod ipc_srv {
    tonic::include_proto!("ipc_interface");
}

pub use profile::Profile;
pub use status::Status;
use std::collections::HashMap;

pub const APP_NAME: &str = "The Box";

pub type ProfileList = HashMap<u32, Profile>;
