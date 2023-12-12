use crate::config::*;
use crate::provisioner::Provisioner;
use color_eyre::Result;
use std::path::PathBuf;

/// Represents a BTRFS volume from the provisioner's perspective.
/// The volume doesn't necessarily need to exist yet.
pub struct BtrfsVolumeMetadata {
    pub path: PathBuf,
    pub host_path: PathBuf,
}

impl BtrfsVolumeMetadata {
    /// Return a BtrfsVolumeMetadata derived from a PV name
    pub fn from_pv_name(pv_name: &str) -> Result<BtrfsVolumeMetadata> {
        let path_parts = vec![VOLUMES_DIR.as_str(), pv_name];

        let path: PathBuf = path_parts.iter().collect();
        let host_path = Provisioner::get_host_path(&path_parts)?;

        Ok(BtrfsVolumeMetadata { path, host_path })
    }
}
