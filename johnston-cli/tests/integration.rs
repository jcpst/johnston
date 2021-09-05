use assert_cmd::Command;

#[test]
fn it_runs() {
    let mut cmd = Command::cargo_bin("johnston-cli").unwrap();
    cmd.assert().success();
}
