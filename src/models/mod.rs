//! Entities defined in Firecracker's API, along with convenient builder types in certain cases.
//!
//! # Note
//!
//! For now, rather than defining public accessor methods etc, all types' fields are declared
//! public, because I think it makes them easier (and less restraining) to handle.
//!
//! This might change in the future, according to my needs I guess.

pub mod balloon;
pub use balloon::Balloon;

pub mod balloon_stats;
pub use balloon_stats::BalloonStats;

pub mod balloon_stats_update;
pub use balloon_stats_update::BalloonStatsUpdate;

pub mod balloon_update;
pub use balloon_update::BalloonUpdate;

pub mod boot_source;
pub use boot_source::BootSource;
pub use boot_source::BootSourceBuilder;

pub mod cpu_template;
pub use cpu_template::CpuTemplate;

pub mod drive;
pub use drive::CacheType;
pub use drive::Drive;
pub use drive::DriveBuilder;
pub use drive::IoEngine;

pub mod error;
pub use error::Error;

pub mod firecracker_version;
pub use firecracker_version::FirecrackerVersion;

pub mod full_vm_configuration;
pub use full_vm_configuration::FullVmConfiguration;

pub mod instance_action_info;
pub use instance_action_info::ActionType;
pub use instance_action_info::InstanceActionInfo;

pub mod instance_info;
pub use instance_info::InstanceInfo;

pub mod logger;
pub use logger::Level;
pub use logger::Logger;
pub use logger::LoggerBuilder;

pub mod machine_configuration;
pub use machine_configuration::MachineConfiguration;
pub use machine_configuration::MachineConfigurationBuilder;

pub mod memory_backend;
pub use memory_backend::BackendType;
pub use memory_backend::MemoryBackend;

pub mod metrics;
pub use metrics::Metrics;

pub mod mmds_config;
pub use mmds_config::MmdsConfig;

mod network_interface;
pub use network_interface::NetworkInterface;
pub use network_interface::NetworkInterfaceBuilder;

pub mod partial_drive;
pub use partial_drive::PartialDrive;
pub use partial_drive::PartialDriveBuilder;

pub mod partial_network_interface;
pub use partial_network_interface::PartialNetworkInterface;
pub use partial_network_interface::PartialNetworkInterfaceBuilder;

pub mod rate_limiter;
pub use rate_limiter::RateLimiter;
pub use rate_limiter::RateLimiterBuilder;

pub mod snapshot_create_params;
pub use snapshot_create_params::SnapshotCreateParams;
pub use snapshot_create_params::SnapshotCreateParamsBuilder;
pub use snapshot_create_params::SnapshotType;

pub mod snapshot_load_params;
pub use snapshot_load_params::SnapshotLoadParams;
pub use snapshot_load_params::SnapshotLoadParamsBuilder;

pub mod token_bucket;
pub use token_bucket::TokenBucket;

pub mod vm;
pub use vm::Vm;

pub mod vsock;
pub use vsock::Vsock;
