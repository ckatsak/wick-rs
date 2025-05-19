//! The example of the ["Getting Started with Firecracker"][1] guide, using [`wick`].
//!
//! ## Note
//!
//! This example does not create and/or configure the required TAP interface.
//!
//! ## Example
//!
//! ```console
//! $ ./getting_started --id <VM_ID> \
//!     --tap-name <TAP_IFACE_NAME> \
//!     --kernel-path <PATH_TO_VMLINUX> \
//!     --rootfs <PATH_TO_ROOTFS> \
//!     --firecracker-bin <PATH_TO_FIRECRACKER_BIN>
//! ```
//!
//! [1]: https://github.com/firecracker-microvm/firecracker/blob/v1.12.0/docs/getting-started.md

use std::time::Duration;

use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use compact_str::{format_compact, CompactString, ToCompactString};
use tokio::{process::Command, time::sleep};
use wick::Api;

const KERNEL_BOOT_ARGS: &str = "console=ttyS0 reboot=k panic=1 pci=off";
const FC_MAC_ADDRESS: &str = "06:00:AC:10:00:02";
const FIRECRACKER_BIN: &str = "firecracker";

/// The example of the "Getting Started with Firecracker" guide, using wick-rs.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about)]
struct Cli {
    /// Unique microVM ID
    #[arg(long)]
    id: CompactString,

    /// Name of the tap device to use
    #[arg(short, long)]
    tap_name: CompactString,

    /// Path to microVM kernel image
    #[arg(short, long)]
    kernel_path: Utf8PathBuf,

    /// Path to microVM rootfs (RW) image
    #[arg(short, long, value_name = "ROOTFS_PATH")]
    rootfs: Utf8PathBuf,

    /// Path to firecracker binary
    #[arg(short, long, default_value = FIRECRACKER_BIN)]
    firecracker_bin: Utf8PathBuf,
}

#[::tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    eprintln!("{cli:?}");
    let api_socket_path = format!("/tmp/fc_{}.socket", cli.id);

    // fork/exec firecracker
    let mut guest_vm_process = Command::new(&cli.firecracker_bin)
        .arg("--api-sock")
        .arg(&api_socket_path)
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("failed to fork/exec '{}'", cli.firecracker_bin))?;

    // give the api server some time to initialize
    sleep(Duration::from_millis(500)).await;

    // setup the guest vm
    setup_guest_vm(&api_socket_path, cli)
        .await
        .context("failed to setup guest VM")?;

    // wait for guest vm to exit
    let status = guest_vm_process
        .wait()
        .await
        .context("failed to wait child VMM process")?;
    if !status.success() {
        if let Some(code) = status.code() {
            bail!("Child firecracker process exited with code '{code}'")
        } else {
            bail!("Child firecracker process was terminated by a signal")
        }
    }

    // unlink(2) API socket after successfully reaping firecracker
    ::tokio::fs::remove_file(&api_socket_path)
        .await
        .with_context(|| format!("failed to remove API socket '{api_socket_path}'"))
}

async fn setup_guest_vm(
    api_socket_path: impl AsRef<Utf8Path>,
    Cli {
        id,
        tap_name,
        kernel_path,
        rootfs,
        ..
    }: Cli,
) -> Result<()> {
    // Create Firecracker client
    let fcc = ::wick::Client::new(api_socket_path.as_ref());

    // print firecracker version
    let fc_version = fcc
        .get_firecracker_version()
        .await
        .context("failed querying Firecracker version")?;
    eprintln!("{fc_version:?}");

    // Create log file
    let log_file_path = Utf8PathBuf::from(format_compact!("/tmp/fc_{id}.log").as_str());
    touch_file(&log_file_path)
        .await
        .context("failed to touch log file")?;

    // Set log file
    set_log_file(&fcc, log_file_path)
        .await
        .context("failed to set log file")?;

    // Set boot source
    set_boot_source(&fcc, &kernel_path)
        .await
        .context("failed to set boot source")?;

    // Set rootfs
    set_rootfs(&fcc, &rootfs)
        .await
        .context("failed to set rootfs drive")?;

    // Set network interface
    set_network_interface(&fcc, tap_name)
        .await
        .context("failed to set network interface")?;

    // API requests are handled asynchronously; it is important the
    // configuration is set before `InstanceStart`.
    sleep(Duration::from_millis(150)).await;

    // Start microVM
    start_microvm(&fcc)
        .await
        .context("failed to start microVM")?;

    Ok(())
}

async fn start_microvm(fcc: &::wick::Client) -> Result<()> {
    use wick::models::instance_action_info::ActionType;

    fcc.create_sync_action(::wick::models::InstanceActionInfo {
        action_type: ActionType::InstanceStart,
    })
    .await
    .context("failed to put ActionType::InstanceStart")
}

async fn set_network_interface(fcc: &::wick::Client, tap_name: CompactString) -> Result<()> {
    const NET1_IFACE_ID: &str = "net1";

    fcc.put_guest_network_interface_by_id(
        NET1_IFACE_ID,
        ::wick::models::NetworkInterface {
            iface_id: NET1_IFACE_ID.to_compact_string(),
            guest_mac: Some(FC_MAC_ADDRESS.to_compact_string()),
            host_dev_name: tap_name,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        },
    )
    .await
    .context("failed to put network network interface")
}

async fn set_rootfs(fcc: &::wick::Client, rootfs_path: impl AsRef<Utf8Path>) -> Result<()> {
    const ROOTFS_DRIVE_ID: &str = "rootfs";

    fcc.put_guest_drive_by_id(
        ROOTFS_DRIVE_ID,
        ::wick::models::Drive {
            drive_id: ROOTFS_DRIVE_ID.to_compact_string(),
            path_on_host: Some(rootfs_path.as_ref().to_owned()),
            is_root_device: true,
            is_read_only: Some(false),
            partuuid: None,
            cache_type: None,
            rate_limiter: None,
            io_engine: None,
            socket: None,
        },
    )
    .await
    .context("failed to put guest drive")
}

async fn set_boot_source(fcc: &::wick::Client, kernel_path: impl AsRef<Utf8Path>) -> Result<()> {
    fcc.put_guest_boot_source(::wick::models::BootSource {
        kernel_image_path: kernel_path.as_ref().to_owned(),
        boot_args: Some(KERNEL_BOOT_ARGS.into()),
        initrd_path: None,
    })
    .await
    .context("failed to PUT guest boot source")
}

async fn set_log_file(fcc: &::wick::Client, log_file_path: Utf8PathBuf) -> Result<()> {
    let logger = ::wick::models::Logger {
        level: Some(::wick::models::logger::Level::Debug),
        log_path: Some(log_file_path),
        show_level: Some(true),
        show_log_origin: Some(true),
        module: None,
    };
    fcc.put_logger(logger).await.context("failed to PUT logger")
}

async fn touch_file(path: impl AsRef<Utf8Path>) -> Result<()> {
    let _file = ::tokio::fs::File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .append(true)
        .open(path.as_ref())
        .await
        .with_context(|| format!("failed to open({}, O_CREAT|O_WRONLY)", path.as_ref()))?;
    Ok(())
}
