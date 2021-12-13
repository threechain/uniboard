use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;

pub const USERS_PATH: &str = "users";

#[hdk_entry(id = "profile")]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub handle: String,
    // pub status: Status,
    // pub avatar_url: String,
    pub address: AgentPubKeyB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateProfileInput {
    pub first_name: String,
    pub last_name: String,
    pub handle: String,
    // pub avata_url: String,
}

#[hdk_extern]
pub fn create_profile(input: CreateProfileInput) -> ExternResult<EntryHash> {
    let agent_pubkey = AgentPubKeyB64::from(agent_info()?.agent_latest_pubkey);
    let profile = Profile {
        first_name: input.first_name,
        last_name: input.last_name,
        handle: input.handle,
        address: agent_pubkey,
    };
    let _ = create_entry(&profile)?;
    let entry_hash = hash_entry(&profile)?;

    let users_path_hash = Path::from(USERS_PATH).hash()?;
    create_link(users_path_hash, entry_hash.clone(), ())?;

    let agent_init_pubkey = agent_info()?.agent_initial_pubkey;
    let agent_init_entry_hash = EntryHash::from(agent_init_pubkey);
    create_link(agent_init_entry_hash, entry_hash.clone(), ())?;

    Ok(entry_hash)
}
