use std::{fs, path::Path};

fn check_layer(path: &Path, forbidden: &[&str]) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            check_layer(&path, forbidden);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = fs::read_to_string(&path).unwrap();
            for dependency in forbidden {
                assert!(
                    !source.contains(dependency),
                    "{} must not depend on {dependency}",
                    path.display()
                );
            }
        }
    }
}
#[test]
fn domain_and_application_keep_their_boundaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    check_layer(
        &root.join("domain"),
        &[
            "sqlx",
            "serde",
            "axum",
            "crate::application",
            "crate::infrastructure",
            "crate::presentation",
        ],
    );
    check_layer(
        &root.join("application"),
        &[
            "sqlx",
            "serde",
            "axum",
            "crate::infrastructure",
            "crate::presentation",
        ],
    );
    check_layer(
        &root.join("presentation"),
        &["sqlx", "crate::infrastructure"],
    );
    assert!(!root.join("domain/enums").exists());
    assert!(!root.join("auth").exists());
    assert!(!root.join("dtos").exists());
}
