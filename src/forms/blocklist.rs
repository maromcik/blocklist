use crate::database::models::{BlocklistCreate, IpVersion};
use ipnetwork::IpNetwork;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BlocklistIpVersion {
    #[serde(default)]
    pub ip_version: Option<IpVersion>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BlocklistIp {
    pub ip: ipnetwork::IpNetwork,
    #[serde(default)]
    pub isp: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub user_agent: Option<String>,
}

impl From<BlocklistIp> for BlocklistCreate {
    fn from(value: BlocklistIp) -> Self {
        let version = match value.ip {
            IpNetwork::V4(_) => IpVersion::Ipv4,
            IpNetwork::V6(_) => IpVersion::Ipv6,
        };
        BlocklistCreate {
            ip: value.ip,
            version,
            country_code: value.country_code,
            isp: value.isp,
            user_agent: value.user_agent,
            description: None,
        }
    }
}
