use assert_cmd::Command;
use rustympkglib::pkgdata::PkgData;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    package: Option<PackageConfig>,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    version: Option<String>,
    description: Option<String>,
}

#[test]
/// We have the version number in quite a few places. This test ensures that all
/// version numbers are updated as appropriate when a new version is released.
fn versions_are_the_same() {
    let pkgbuild = &fs::read_to_string("PKGBUILD").unwrap();
    let pkgbuild = PkgData::from_source(pkgbuild).unwrap();
    let pkgbuild = pkgbuild.pkgver;

    let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo: Config = toml::from_str(cargo).unwrap();
    let cargo = cargo.package.unwrap().version.unwrap();

    assert_eq!(
        pkgbuild,
        cargo.as_str(),
        "Cargo.toml and PKGBUILD should have the same version"
    );
}

#[test]
/// Ensures that the description is the same across the project
///
/// Note that the PKGBUILD should not have a period at the end.
fn descriptions_are_the_same() {
    let pkgbuild = &fs::read_to_string("PKGBUILD").unwrap();
    let pkgbuild = PkgData::from_source(pkgbuild).unwrap();
    let pkgbuild = pkgbuild.pkgdesc.unwrap() + ".";

    let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo: Config = toml::from_str(cargo).unwrap();
    let cargo = cargo.package.unwrap().description.unwrap();

    assert_eq!(
        pkgbuild,
        cargo.as_str(),
        "Cargo.toml and PKGBUILD should have the same description"
    );
}

#[test]
/// Ensures that the usage code block in the README is the same as the output of hyprnome -h
fn usage_is_the_same() {
    let cargo_description = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo_description: Config = toml::from_str(cargo_description).unwrap();
    let cargo_description = cargo_description.package.unwrap().description.unwrap();

    let readme = &fs::read_to_string("README.md").unwrap();
    let mut inside_code_block = false;

    // Initialize with cargo_description since we don't duplicate this in the README
    let mut readme_usage: String = format!("{cargo_description}\n\n");

    for line in readme.lines() {
        if line == "```" {
            inside_code_block = false;
            continue;
        }

        if inside_code_block {
            readme_usage.push_str(&(line.to_owned() + "\n"));
            continue;
        }

        if line == "```man" {
            inside_code_block = true;
        }
    }

    let mut cmd = Command::cargo_bin("hyprnome").unwrap();
    cmd.arg("-h").assert().stdout(readme_usage);
}

#[test]
/// Ensures that the correct version of hyprnome is found in the README
fn current_version_is_used() {
    let cargo_version = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo_version: Config = toml::from_str(cargo_version).unwrap();
    let cargo_version = cargo_version.package.unwrap().version.unwrap();

    let readme = &fs::read_to_string("README.md").unwrap();

    assert!(
        readme.contains(&("--tag ".to_owned() + cargo_version.as_str())),
        "should have the correct tag version in the README"
    );

    assert!(
        readme.contains(&("-b ".to_owned() + cargo_version.as_str())),
        "should have the correct branch version in the README"
    )
}
