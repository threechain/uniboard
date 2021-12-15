use hdk::prelude::*;

pub const USERS_PATH: &str = "users";

entry_defs![
    Path::entry_def(),
    Profile::entry_def()
];

#[hdk_entry(id = "profile")]
pub struct Profile {
    pub handle: String,
    // pub avatar_url: String,
    pub address: AgentPubKeyB64,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateProfileInput {
    pub handle: String,
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Path::from(USERS_PATH).ensure()?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn create_profile(input: CreateProfileInput) -> ExternResult<EntryHashB64> {
    let agent_pubkey = AgentPubKeyB64::from(agent_info()?.agent_latest_pubkey);
    let profile = Profile {
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

    Ok(EntryHashB64::from(entry_hash))
}
