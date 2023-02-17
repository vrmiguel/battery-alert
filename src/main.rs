use std::fs::File;

use daemonize::Daemonize;

mod battery_reader;
mod monitor;
mod utils;

fn main() {
    let username = unsafe { utils::get_username() }.unwrap_or_else(|| "user".into());

    let stdout = File::create("/tmp/battery-alertd.out").unwrap();
    let stderr = File::create("/tmp/battery-alertd.err").unwrap();

    let daemonize = Daemonize::new()
        .user(&*username)
        .pid_file("/tmp/test.pid")
        .chown_pid_file(false)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    if let Err(err) = daemonize.start() {
        println!("Failed to daemonize process: {}", err);
        std::process::exit(1);
    }

    println!("User {} started the daemon.", username);

    monitor::Monitor::new().start_loop()
}
