use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("nonexistent/file/somewhere/deep");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn pattern_and_file_dont_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("").arg("nonexistent/file/somewhere/deep");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn pattern_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "A test\nActual content\nMore content\nAnother test",
    ));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
