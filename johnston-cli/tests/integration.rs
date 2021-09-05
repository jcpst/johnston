use assert_cmd::Command;

#[test]
fn it_runs() {
    let mut cmd = Command::cargo_bin("ji").unwrap();
    cmd.assert().success();
}
