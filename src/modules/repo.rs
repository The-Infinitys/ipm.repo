use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::error::{RepositoryError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Repository {
    pub path: PathBuf,
    pub config: RepositoryConfig,
}

impl Repository {
    pub fn init(name: String, path: PathBuf) -> Result<Self> {
        if path.exists() {
            return Err(RepositoryError::AlreadyExists(
                path.display().to_string(),
            ));
        }

        fs::create_dir_all(&path)?;
        fs::create_dir_all(path.join("packages"))?;

        let config = RepositoryConfig {
            name,
            version: "0.1.0".to_string(),
        };

        let config_path = path.join("config.toml");
        let toml_string = toml::to_string(&config)?;
        fs::write(config_path, toml_string)?;

        Ok(Self { path, config })
    }

    pub fn load(path: PathBuf) -> Result<Self> {
        let mut current_path = path.clone();
        loop {
            let config_path = current_path.join("config.toml");
            if config_path.exists() {
                let config_content =
                    fs::read_to_string(config_path)?;
                let config: RepositoryConfig =
                    toml::from_str(&config_content)?;
                return Ok(Self { path: current_path, config });
            }
            match current_path.parent() {
                Some(parent) => {
                    current_path = parent.to_path_buf()
                }
                None => {
                    return Err(RepositoryError::ConfigNotFound);
                }
            }
        }
    }

    pub fn add_package(
        &self,
        package_path: PathBuf,
    ) -> Result<()> {
        let package =
            ipak::modules::pkg::metadata::get(&package_path)?;
        let package_dir =
            self.path.join("packages").join(format!(
                "{}-{}",
                package.about.package.name,
                package.about.package.version
            ));

        if package_dir.exists() {
            return Err(RepositoryError::PackageAlreadyExists(
                package.about.package.name,
                package.about.package.version.to_string(),
            ));
        }

        fs::create_dir_all(&package_dir)?;
        fs::copy(
            &package_path,
            package_dir.join(package_path.file_name().unwrap()),
        )?;

        Ok(())
    }

    pub fn remove_package(
        &self,
        name: String,
        version: String,
    ) -> Result<()> {
        let package_dir = self
            .path
            .join("packages")
            .join(format!("{}-{}", name, version));

        if !package_dir.exists() {
            return Err(RepositoryError::PackageNotFound(
                format!("{}-{}", name, version),
            ));
        }

        fs::remove_dir_all(&package_dir)?;

        Ok(())
    }

    pub fn list_packages(
        &self,
    ) -> Result<Vec<ipak::modules::pkg::PackageData>> {
        let packages_dir = self.path.join("packages");
        let mut packages = Vec::new();

        for entry in fs::read_dir(packages_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Assuming each package directory contains a single .ipak file
                for package_file in fs::read_dir(&path)? {
                    let package_file = package_file?;
                    let package_path = package_file.path();
                    if package_path
                        .extension()
                        .map_or(false, |ext| ext == "ipak")
                    {
                        packages.push(
                            ipak::modules::pkg::metadata::get(
                                &package_path,
                            )?,
                        );
                    }
                }
            }
        }
        Ok(packages)
    }
}
