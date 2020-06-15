
pub struct ServerMeta {
    pub leader_id: u64
}

impl ServerMeta {
    pub fn default() -> Self {
        ServerMeta {
            leader_id: 0
        }
    }
}