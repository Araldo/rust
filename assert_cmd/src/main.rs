use std::process::Command;
use std::str::from_utf8;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn main() {
    let res = Command::new("ls").output().unwrap();
    let res2 = from_utf8(&res.stdout).unwrap();
    dbg!(res2);
}

#[test]
fn runs() -> TestResult {
    let res = Command::new("ls").output()?;
    let res2 = from_utf8(&res.stdout)?;
    let cmp = "Cargo.toml";
    assert!(res2.contains(&cmp) == true);
    Ok(())
}
