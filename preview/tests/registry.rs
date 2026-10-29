use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn members() -> Vec<String> {
    let manifest = fs::read_to_string(root().join("component.json")).unwrap();
    manifest
        .lines()
        .filter_map(|line| line.trim().strip_prefix("\"preview/src/components/"))
        .map(|line| line.trim_end_matches(',').trim_end_matches('"').to_string())
        .collect()
}

#[test]
fn every_component_directory_is_a_member_except_the_copied_button() {
    let mut found: Vec<String> = fs::read_dir(root().join("preview/src/components"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .filter(|name| name != "button")
        .collect();
    found.sort();
    assert_eq!(found, members());
}

#[test]
fn the_preview_compiles_each_component_as_it_will_be_installed() {
    let preview = fs::read_to_string(root().join("preview/src/components/mod.rs")).unwrap();
    for name in members() {
        let block = preview
            .split(&format!("pub mod {name} {{"))
            .nth(1)
            .unwrap_or_else(|| panic!("{name} is missing from preview/src/components/mod.rs"));
        let installed =
            fs::read_to_string(root().join(format!("preview/src/components/{name}/mod.rs")))
                .unwrap();
        for line in installed.lines().filter(|line| !line.is_empty()) {
            assert!(
                block.contains(line),
                "{name}/mod.rs has `{line}` that the preview does not compile"
            );
        }
    }
}

#[test]
fn every_member_has_its_files_and_names_itself() {
    for name in members() {
        let dir = root().join(format!("preview/src/components/{name}"));
        for file in ["component.json", "component.rs", "mod.rs", "docs.md"] {
            assert!(dir.join(file).exists(), "{name} has no {file}");
        }
        let manifest = fs::read_to_string(dir.join("component.json")).unwrap();
        assert!(
            manifest.contains(&format!("\"name\": \"{name}\"")),
            "{name}/component.json names another component"
        );
    }
}
