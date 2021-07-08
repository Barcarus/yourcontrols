use std::{collections::HashMap, net::SocketAddr};

use crate::base::Payload;
use laminar::Packet;
use serde::{Deserialize, Serialize};
use yourcontrols_types::{ChangedDatum, ClientId, Time};

pub type ControlDelegationsMap = HashMap<ControlSurfaces, ClientId>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlSurfaces {
    Throttle,
    Mixture,
    Yoke,
}

impl ControlSurfaces {
    pub fn all() -> Vec<Self> {
        vec![Self::Throttle, Self::Mixture, Self::Yoke]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MainPayloads {
    // Handshake payloads
    Hello {
        session_id: String,
        version: String,
    },
    RequestSession {
        self_hosted: bool,
    },
    SessionDetails {
        session_id: String,
    },
    AttemptConnection {
        public_ip: String,
        local_ip: String,
    }, // Base64 encoded strings
    InvalidSession,
    InvalidVersion {
        server_version: String,
    },
    // Main Game Payloads
    Update {
        is_reliable: bool,
        time: Time,
        changed: Vec<ChangedDatum>,
    },
    Name {
        name: String,
    },
    // Assign client id
    Welcome {
        client_id: ClientId,
    },
    MakeHost {
        client_id: ClientId,
    },
    TransferControl {
        delegation: ControlSurfaces,
        to: ClientId,
    },
    ControlDelegations {
        delegations: ControlDelegationsMap,
    },
    ClientAdded {
        id: ClientId,
        is_observer: bool,
        is_host: bool,
        name: String,
    },
    ClientRemoved {
        id: ClientId,
    },
}

impl Payload for MainPayloads {
    // Handshake payloads
    fn get_packet(&self, addr: SocketAddr, bytes: Vec<u8>) -> laminar::Packet {
        match self {
            MainPayloads::Hello { .. }
            | MainPayloads::Welcome { .. }
            | MainPayloads::RequestSession { .. }
            | MainPayloads::SessionDetails { .. }
            | MainPayloads::AttemptConnection { .. }
            | MainPayloads::InvalidSession
            | MainPayloads::InvalidVersion { .. }
            | MainPayloads::Name { .. }
            | MainPayloads::MakeHost { .. }
            | MainPayloads::TransferControl { .. }
            | MainPayloads::ClientAdded { .. }
            | MainPayloads::ClientRemoved { .. }
            | MainPayloads::ControlDelegations { .. } => {
                Packet::reliable_ordered(addr, bytes, None)
            }
            MainPayloads::Update { is_reliable, .. } => {
                if *is_reliable {
                    Packet::reliable_ordered(addr, bytes, None)
                } else {
                    Packet::unreliable_sequenced(addr, bytes, None)
                }
            }
        }
    }
}