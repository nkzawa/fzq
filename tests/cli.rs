use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn filter_content_in_file() -> Result<(), Box<std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\ntest 1\nActual content\nMore content\ntest 2\nAnother test"
    )?;
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "A test\ntest 1\nActual content\nMore content\nAnother test",
    ));
    Ok(())
}

#[test]
fn filter_content_on_stdin() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.with_stdin()
        .buffer("test 1\ntest 2\ntest 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test 1\nend"));
    Ok(())
}

#[test]
fn metric_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-m")
        .arg("DamerauLevenshtein")
        .with_stdin()
        .buffer("test text 1\ntest text 2\ntest text 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test text 1\nend"));
    Ok(())
}

#[test]
fn unsupported_metric_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-m")
        .arg("hi")
        .with_stdin()
        .buffer("test 1\ntest 2\ntest 3\nend")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn threshold_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-t")
        .arg("0.99")
        .with_stdin()
        .buffer("test 1\ntest 2\ntest 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test 1\ntest 2\ntest 3\nend"));
    Ok(())
}

#[test]
fn buffer_size_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-b")
        .arg("1")
        .with_stdin()
        .buffer("test 1\ntest 2\nfoobar 1\nfoobar 2\ntest 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test 1\nfoobar 1\ntest 3\nend"));
    Ok(())
}

#[test]
fn line_number_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-n")
        .with_stdin()
        .buffer("test 1\ntest 2\ntest 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("1:test 1\n4:end"));
    Ok(())
}

#[test]
fn check_chars_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-w")
        .arg("5")
        .with_stdin()
        .buffer("test asdf 1\ntest foobar 2\ntest hahaha 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test asdf 1\nend"));
    Ok(())
}

#[test]
fn check_chars_option_multi_byte() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-w")
        .arg("4")
        .with_stdin()
        .buffer("ははは　asdf 1\nははは　foobar 2\nははは　hahaha 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("ははは　asdf 1\nend"));
    Ok(())
}

#[test]
fn skip_chars_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-s")
        .arg("5")
        .with_stdin()
        .buffer("asdfg test text 1\nhahah test text 2\nlorem test text 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("asdfg test text 1\nend"));
    Ok(())
}

#[test]
fn skip_chars_option_multi_byte() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-s")
        .arg("6")
        .with_stdin()
        .buffer("あいうえお　テスト　テキスト　１\nかきくけこ　テスト　テキスト　２\nさしすせそ　テスト　テキスト　３\n終わり")
        .assert()
        .success()
        .stdout(predicate::str::contains("あいうえお　テスト　テキスト　１\n終わり"));
    Ok(())
}

#[test]
fn skip_fields_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-f")
        .arg("1")
        .with_stdin()
        .buffer("asdf test text 1\nhahaha test text 2\nlorem test text 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("asdf test text 1\nend"));
    Ok(())
}

#[test]
fn skip_fields_option_multi_byte() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-f")
        .arg("1")
        .with_stdin()
        .buffer("あいうえお テスト テキスト １\nかきく テスト テキスト ２\nさしすせ テスト テキスト ３\n終わり")
        .assert()
        .success()
        .stdout(predicate::str::contains("あいうえお テスト テキスト １\n終わり"));
    Ok(())
}

#[test]
fn ignore_case_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-i")
        .with_stdin()
        .buffer("test 1\nTEST 2\ntEsT 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test 1\nend"));
    Ok(())
}

#[test]
fn all_similar_option() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME)?;
    cmd.arg("-D")
        .with_stdin()
        .buffer("test 1\ntest 2\ntest 3\nend")
        .assert()
        .success()
        .stdout(predicate::str::contains("test 2\ntest 3"));
    Ok(())
}
