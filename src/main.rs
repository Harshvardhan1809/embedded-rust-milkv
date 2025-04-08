// use mpu6050::*;
// use linux_embedded_hal::{I2cdev, Delay};
// use i2cdev::linux::LinuxI2CError;

// fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
//     let i2c = I2cdev::new("/dev/i2c-1")
//         .map_err(Mpu6050Error::I2c).expect("");

//     let mut delay = Delay;
//     let mut mpu = Mpu6050::new(i2c);

//     mpu.init(&mut delay).expect("");

//     loop {
//         // get roll and pitch estimate
//         let acc = mpu.get_acc_angles().expect("");
//         println!("r/p: {:?}", acc);

//         // get sensor temp
//         let temp = mpu.get_temp().expect("");
//         println!("temp: {:?}c", temp);

//         // get gyro data, scaled with sensitivity
//         let gyro = mpu.get_gyro().expect("");
//         println!("gyro: {:?}", gyro);

//         // get accelerometer data, scaled with sensitivity
//         let acc = mpu.get_acc().expect("");
//         println!("acc: {:?}", acc);
//     }
// }

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() -> std::result::Result<(), linux_embedded_hal::gpio_cdev::Error> {

    println!("Check the status 6th pin");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed on open GPIO chip");
    let line = chip.get_line(19).expect("Got the line");
    println!("The LED pin is held by the kernel: {:?}", line.info()?.is_kernel());
    println!("");

    println!("Check the status for LED pin");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed on open GPIO chip");
    let line = chip.get_line(2).expect("Got the line");
    println!("The LED pin is held by the kernel: {:?}", line.info()?.is_kernel());
    println!("");

    let handle = line.request(LineRequestFlags::OUTPUT, 1, "sample")?;
    println!("Right here");
    let duration = Duration::from_millis(10000);
    let start_time = Instant::now();
    while start_time.elapsed() < duration {
        sleep(Duration::from_millis(1000));
        handle.set_value(0)?;
        sleep(Duration::from_millis(1000));
        handle.set_value(1)?;
    }
    println!("Over here");

    println!("Done.");
    Ok(())
}

//     println!("Check the status for PIN 19 (GP14)");
//     let mut chip = Chip::new("/dev/gpiochip0").expect("Failed on open GPIO chip");
//     let line = chip.get_line(14).expect("Got the line");
//     println!("The LED pin is held by the kernel: {:?}", line.info()?.is_kernel());
//     println!("");

//     println!("Check the status for PIN 14 (GP10)");
//     let mut chip = Chip::new("/dev/gpiochip2").expect("Failed on open GPIO chip");
//     let line = chip.get_line(14).expect("Got the line");
//     println!("The LED pin is held by the kernel: {:?}", line.info()?.is_kernel());
//     println!("");

//     println!("Let's get some information about the PIN 14 (GP10)");
//     println!("Used: {:?}", line.info()?.is_used());
//     println!("Consumer: {:?}", line.info()?.consumer().expect(""));
//     println!("Name: {:?}", line.info()?.name().expect(""));
//     println!("Is Active Low: {:?}", line.info()?.is_active_low());

//     let handle = line.request(LineRequestFlags::OUTPUT, 1, "sample")?;
//     println!("Right here");
//     let duration = Duration::from_millis(1000);
//     let start_time = Instant::now();
//     while start_time.elapsed() < duration {
//         sleep(Duration::from_millis(1000));
//         handle.set_value(0)?;
//         sleep(Duration::from_millis(1000));
//         handle.set_value(1)?;
//     }
//     println!("Over here");

//     println!("Done.");
//     Ok(())
// }