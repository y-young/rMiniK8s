use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct KubeletConfig {
    /// Path to the directory containing local (static) pods to run,
    /// or the path to a single static pod file.
    /// Defaults to "/etc/rminik8s/manifests".
    pub static_pod_path: String,
    /// Frequency that kubelet computes node status.
    /// In seconds. Default: 10 sec
    pub node_status_update_frequency: u64,
    /// Frequency that kubelet posts node status to master
    /// if node status does not change.
    /// Kubelet will ignore this frequency and
    /// post node status immediately if any change is detected.
    /// In seconds. Default: 5 min
    pub node_status_report_frequency: u64,
    /// Frequency that kubelet computes pod status.
    /// In seconds. Default: 10 sec
    pub pod_status_update_frequency: u64,
    /// API server URL
    pub api_server_url: String,
    /// API server watch URL
    pub api_server_watch_url: String,
}

impl Default for KubeletConfig {
    fn default() -> Self {
        KubeletConfig {
            static_pod_path: "/etc/rminik8s/manifests".to_string(),
            node_status_update_frequency: 10,
            node_status_report_frequency: 300,
            pod_status_update_frequency: 10,
            api_server_url: "http://localhost:8080".to_string(),
            api_server_watch_url: "ws://localhost:8080".to_string(),
        }
    }
}
