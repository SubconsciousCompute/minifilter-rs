use minifilter_rs::driver_comm;
use minifilter_rs::shared_def::{CDriverMsgs, IOMessage};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let driver = driver_comm::Driver::open_kernel_driver_com()
        .expect("Cannot open driver communication (is the mini-filter started?)");
    driver
        .driver_set_app_pid()
        .expect("Cannot set driver app pid");
    let mut vecnew: Vec<u8> = Vec::with_capacity(65536);

    let (tx_iomsgs, rx_iomsgs) = channel::<IOMessage>();

    thread::spawn(move || loop {
        if let Some(reply_irp) = driver.get_irp(&mut vecnew) {
            if reply_irp.num_ops > 0 {
                let drivermsgs = CDriverMsgs::new(&reply_irp);
                for drivermsg in drivermsgs {
                    let iomsg = IOMessage::from(&drivermsg);
                    if tx_iomsgs.send(iomsg).is_ok() {
                    } else {
                        panic!("Cannot send iomsg");
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        } else {
            panic!("Can't receive Driver Message?");
        }
    });

    loop {
        if let Ok(io_message) = rx_iomsgs.recv() {
            println!("{:#?}\n", io_message);
        }
    }
}
