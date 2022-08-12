use std::collections::BTreeMap;

use holochain_deterministic_integrity::prelude::*;

/// Profile entry definition.
///
/// The profile must include at a minimum the nickname of the agent
/// in order to be able to search for agents by nickname.
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Profile {
    pub nickname: String,
    pub fields: BTreeMap<String, String>,
}
