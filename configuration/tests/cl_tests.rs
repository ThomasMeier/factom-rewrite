use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[ignore]
fn test_role() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();
    let mut cmd4 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--role").arg("FULL").assert().success();

    cmd2.arg("--role").arg("AUTHORITY").assert().success();

    cmd3.arg("--role").arg("LIGHT").assert().success();

    cmd4.arg("--role").arg("OTHER").assert().failure();
}

#[test]
#[ignore]
fn test_completions() {
    let elvish_output = "
edit:completion:arg-completer[factomd-configuration] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'factomd-configuration'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'factomd-configuration'= {
            cand -c 'Custom configuration file location'
            cand --config 'Custom configuration file location'
            cand -n 'Set network to join'
            cand --network 'Set network to join'
            cand -r 'Environment variable to source for your node_key'
            cand --role 'Environment variable to source for your node_key'
            cand -k 'Environment variable to source for your node_key'
            cand --node-key-env 'Environment variable to source for your node_key'
            cand --port 'Port to run node on for P2P'
            cand --bootnodes 'Bootnodes to get into the network'
            cand -l 'l'
            cand --log-level 'log-level'
            cand --rpc-addr 'HTTP-RPC listening interface'
            cand --rpc-port 'HTTP-RPC listening port'
            cand --walletd-user 'Set walletd user for authentication'
            cand --walletd-env-var 'Set env variable to get walletd password'
            cand --completions 'Generate completions'
            cand -d 'Disable RPC server'
            cand --disable-rpc 'Disable RPC server'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
";
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--completions").arg("bash").assert().success();

    cmd2.arg("--completions")
        .arg("not-a-shell")
        .assert()
        .failure();

    cmd3.arg("--completions")
        .arg("elvish")
        .assert()
        .stdout(elvish_output);
}

#[test]
#[ignore]
fn test_rpc() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();
    let mut cmd4 = Command::cargo_bin("factomd").unwrap();
    let mut cmd5 = Command::cargo_bin("factomd").unwrap();
    let mut cmd6 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--rpc-port").arg("8099").assert().success();

    cmd2.arg("--rpc-port").arg("forty-five").assert().failure();

    cmd3.arg("--rpc-port").arg("8099").assert().stdout("");

    cmd4.arg("--rpc-addr").arg("127.0.0.1").assert().success();

    cmd4.arg("--rpc-addr").arg("123456").assert().failure();

    cmd5.arg("--disable-rpc").assert().success();

    cmd6.arg("--disable-rpc")
        .arg("shouldn't-have-an-arg")
        .assert()
        .failure();
}

#[test]
#[ignore]
fn test_log_level() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();
    let mut cmd4 = Command::cargo_bin("factomd").unwrap();
    let mut cmd5 = Command::cargo_bin("factomd").unwrap();
    let mut cmd6 = Command::cargo_bin("factomd").unwrap();
    let mut cmd7 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--log-level").arg("CRITICAL").assert().success();
    cmd2.arg("--log-level").arg("ERROR").assert().success();
    cmd3.arg("--log-level").arg("WARN").assert().success();
    cmd4.arg("--log-level").arg("INFO").assert().success();
    cmd5.arg("--log-level").arg("DEBUG").assert().success();
    cmd6.arg("--log-level").arg("TRACE").assert().success();
    cmd7.arg("--log-level")
        .arg("NON-VARIANT")
        .assert()
        .failure();
}

#[test]
#[ignore]
fn test_walletd() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--walletd-user")
        .arg("testuser")
        .assert()
        .success();

    cmd2.arg("--walletd-env-var")
        .arg("factomd-password")
        .assert()
        .success();

    cmd3.arg("--walletd-user")
        .arg("testuser")
        .arg("--walletd-env-var")
        .arg("factom-walletd")
        .assert()
        .success();
}

#[test]
#[ignore]
fn test_network() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();
    let mut cmd3 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--network").arg("main").assert().success();

    cmd2.arg("--network")
        .arg("custom-network-123")
        .assert()
        .success();

    cmd3.arg("--network").arg("1234").assert().success();
}

#[test]
#[ignore]
fn test_node_key_env() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--node-key-env")
        .arg("FACTOMD_NODE_KEY")
        .assert()
        .success();
}

#[test]
#[ignore]
fn test_config_arg() {
    let mut cmd1 = Command::cargo_bin("factomd").unwrap();
    let mut cmd2 = Command::cargo_bin("factomd").unwrap();

    cmd1.arg("--config")
        .arg("tests/nondefaults.yml")
        .assert()
        .success();

    cmd2.arg("--config")
        .arg("non-existent-config.yml")
        .assert()
        .failure();
}
