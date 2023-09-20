use core::time::Duration;
use std::thread;

#[test]
fn long_integration_test() {
    thread::sleep(Duration::from_secs(62));
}
