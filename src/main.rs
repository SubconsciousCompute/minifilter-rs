extern crate core;

// use std::thread::sleep;
// use std::time::Duration;
use minifilter_rs::driver_comm;

fn main() {
    let driver = driver_comm::Driver::open_kernel_driver_com().expect("life is Pain");

    dbg!(&driver);
    println!();

    let k = driver.driver_set_app_pid();

    dbg!(&driver, k);
    println!();

    let k = driver
        .try_kill(1128)
        .expect("unable to kill");

    dbg!(&driver, k);
    println!("{}", k.message());
    // sleep(Duration::from_secs(20));
}
