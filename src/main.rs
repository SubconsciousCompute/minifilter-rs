// use std::thread::sleep;
// use std::time::Duration;
use minifilter_rs::driver_comm;

fn main() {
    let (driver, k) = driver_comm::Driver::open_kernel_driver_com();

    dbg!(&driver, k);

    let k = driver.driver_set_app_pid();

    dbg!(&driver, k);

    let k = driver.try_kill(10184);

    dbg!(&driver, k);

    // sleep(Duration::from_secs(20));
}
