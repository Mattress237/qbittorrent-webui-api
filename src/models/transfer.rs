use serde::{Deserialize, Serialize};

use crate::models::ConnectionStatus;

/// Transfer info data object
///
/// This is the data that whuld usually se in the Qbit status bar.
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct TransferInfo {
    /// Global download rate (bytes/s)
    #[serde(rename = "dl_info_speed")]
    pub global_download_rate: i64,
    /// Data downloaded this session (bytes)
    #[serde(rename = "dl_info_data")]
    pub downloaded_this_session: i64,
    /// Global upload rate (bytes/s)
    #[serde(rename = "up_info_speed")]
    pub global_upload_rate: i64,
    /// Data uploaded this session (bytes)
    #[serde(rename = "up_info_data")]
    pub uploaded_this_session: i64,
    /// Download rate limit (bytes/s)
    #[serde(rename = "dl_rate_limit")]
    pub download_rate_limit: i32,
    /// Upload rate limit (bytes/s)
    #[serde(rename = "up_rate_limit")]
    pub upload_rate_limit: i32,
    /// Last external IPv4 address
    ///
    /// This field has not been documented in the API!
    pub last_external_address_v4: String,
    /// Last external IPv4 address
    ///
    /// This field has not been documented in the API!
    pub last_external_address_v6: String,
    /// DHT nodes connected to
    pub dht_nodes: i64,
    /// The connection status of qbitt.
    pub connection_status: ConnectionStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use serde_json::{Number, Value, json};

    mod serialization {
        use super::*;

        #[test]
        fn transfer_info() {
            let mut transfer_info = TransferInfo::default();
            transfer_info.global_download_rate = 1000;
            transfer_info.global_upload_rate = 2000;
            transfer_info.download_rate_limit = 100;
            transfer_info.upload_rate_limit = 200;

            let serialized = serde_json::to_string(&transfer_info).unwrap();
            assert!(serialized.contains("\"dl_info_speed\":1000"));
            assert!(serialized.contains("\"up_info_speed\":2000"));
            assert!(serialized.contains("\"dl_rate_limit\":100"));
            assert!(serialized.contains("\"up_rate_limit\":200"));
        }
    }

    mod deserialization {
        use super::*;

        #[test]
        fn transfer_info() {
            let json = json!({
                "dl_info_speed": 1000,
                "dl_info_data": 456525,
                "up_info_speed": 2000,
                "up_info_data": 456456,
                "dl_rate_limit": 100,
                "up_rate_limit": 200,
                "last_external_address_v4": "192.168.1.1",
                "last_external_address_v6": "2001:db8::1",
                "dht_nodes": 100,
                "connection_status": "connected"
            });
            let transfer_info: TransferInfo = serde_json::from_value(json).unwrap();
            assert_eq!(transfer_info.global_download_rate, 1000);
            assert_eq!(transfer_info.global_upload_rate, 2000);
            assert_eq!(transfer_info.download_rate_limit, 100);
            assert_eq!(transfer_info.upload_rate_limit, 200);
            assert_eq!(transfer_info.last_external_address_v4, "192.168.1.1");
            assert_eq!(transfer_info.last_external_address_v6, "2001:db8::1");
            assert_eq!(transfer_info.dht_nodes, 100);
            assert_eq!(transfer_info.connection_status, ConnectionStatus::Connected);
        }
    }
}
