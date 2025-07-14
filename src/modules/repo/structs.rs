use crate::utils::url::Url;
use chrono::{DateTime, Local};
use ipak::modules::pkg::PackageData;
use sha2::Sha256;
pub struct RepoPackageData {
    data: PackageData,
    last_modified: DateTime<Local>,
    hash: Sha256,
    size: usize,
    path: Url,
}
