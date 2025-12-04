use crate::database::models::IpVersion;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BlocklistIpVersion {
    pub ip_version: IpVersion,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct BlocklistIp {
    pub ip: ipnetwork::IpNetwork,
}
