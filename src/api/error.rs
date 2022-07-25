//! Typed errors for all API calls supported.

use serde::{Deserialize, Serialize};

use crate::models::Error;

#[allow(unused_imports)]
use crate::MicrovmClient; // imported only for the docs

/// Error type returned by [`MicrovmClient::create_snapshot`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSnapshotError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::create_sync_action`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyncActionError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::describe_balloon_config`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeBalloonConfigError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::describe_balloon_stats`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeBalloonStatsError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::describe_instance`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescribeInstanceError {
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::get_export_vm_config`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExportVmConfigError {
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::get_firecracker_version`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFirecrackerVersionError {
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::get_machine_configuration`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMachineConfigurationError {
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::get_mmds`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMmdsError {
    NotFound(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::load_snapshot`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoadSnapshotError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_balloon`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchBalloonError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_balloon_stats_interval`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchBalloonStatsIntervalError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_guest_drive_by_id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchGuestDriveByIdError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_guest_network_interface_by_id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchGuestNetworkInterfaceByIdError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_machine_configuration`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchMachineConfigurationError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_mmds`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchMmdsError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::patch_vm`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchVmError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_balloon`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutBalloonError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_guest_boot_source`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuestBootSourceError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_guest_drive_by_id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuestDriveByIdError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_guest_network_interface_by_id`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuestNetworkInterfaceByIdError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_guest_vsock`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutGuestVsockError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_logger`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutLoggerError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_machine_configuration`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutMachineConfigurationError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_metrics`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutMetricsError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_mmds`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutMmdsError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}

/// Error type returned by [`MicrovmClient::put_mmds_config`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutMmdsConfigError {
    BadRequest(Error),
    InternalServerError(Error),
    UnknownValue(serde_json::Value),
}
