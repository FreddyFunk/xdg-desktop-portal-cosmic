// SPDX-License-Identifier: GPL-3.0-only
//
// Exercises the Background portal: requests permission to run in the background and,
// optionally, to be started automatically on login.

use ashpd::desktop::background::Background;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ashpd::Result<()> {
    env_logger::Builder::from_default_env().init();

    let executable = std::env::args().next().unwrap();

    let response = Background::request()
        .reason("Testing the background portal")
        .auto_start(false)
        .dbus_activatable(false)
        .command(&[executable])
        .send()
        .await?
        .response()?;

    println!("run_in_background: {}", response.run_in_background());
    println!("auto_start: {}", response.auto_start());

    Ok(())
}
