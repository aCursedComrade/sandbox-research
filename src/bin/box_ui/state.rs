use sandbox_research::Profile;
use std::collections::HashMap;

/// State list holding profile data (client)
pub(crate) type ProfileList = HashMap<u32, Profile>;

pub(crate) trait ListUtils {
    fn add_profile(&mut self, profile: Profile);
}

impl ListUtils for ProfileList {
    fn add_profile(&mut self, profile: Profile) {
        self.insert(profile.id, profile);
    }
}
