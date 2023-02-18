use server::ServerModBox;
use crate::mods::ModProfile;

pub struct ServerRuntime {
    pub(crate) mod_profile: ModProfile,
    pub(crate) loaded_mods: Vec<ServerModBox>
}