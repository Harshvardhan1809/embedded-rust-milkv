use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> std::result::Result<(), linux_embedded_hal::gpio_cdev::Error> {

    println!("Check the status for LED pin");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed on open GPIO chip");
    let line = chip.get_line(2).expect("Got the line");
    println!("The LED pin is held by the kernel: {:?}", line.info()?.is_kernel());
    println!("");

    let handle = line.request(LineRequestFlags::OUTPUT, 1, "sample")?;
    println!("START BLINKING");
    let duration = Duration::from_millis(10000);
    let start_time = Instant::now();
    while start_time.elapsed() < duration {
        sleep(Duration::from_millis(1000));
        handle.set_value(0)?;
        println!("OFF");
        sleep(Duration::from_millis(1000));
        handle.set_value(1)?;
        println!("ON");
    }
    println!("END BLINKING");

    Ok(())
}