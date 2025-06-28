// use mpu6050::*;
// use linux_embedded_hal::{I2cdev, Delay};
// use i2cdev::linux::LinuxI2CError;
// // use embedded_hal::blocking::i2c::{WriteRead, Write};
// // use embedded_hal::blocking::delay::DelayMs;
// use shared_bus;
// use i2cdev::linux::{LinuxI2CDevice, LinuxI2CBus};
// use embedded_hal_bus::i2c;
// use embedded_hal_bus::util::AtomicCell;
// // use std::ops::{Deref, DerefMut};

// struct MyI2cdev(I2cdev);

// // https://docs.esp-rs.org/esp-idf-hal/src/esp_idf_hal/i2c.rs.html#202-205

// // impl WriteRead for MyI2cdev {
// //     type Error = LinuxI2CError;

// //     fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
// //         MyI2cdev::write_read(self, addr, bytes, buffer).map_err(to_i2c_err)
// //     }
// // }

// // impl Deref for MyI2cdev {
// //     type Target = I2cdev;
// //     fn deref(&self) -> &I2cdev {
// //         &self.0
// //     }
// // }

// // impl DerefMut for MyI2cdev {
// //     fn deref_mut(&mut self) -> &mut I2cdev {
// //         &mut self.0
// //     }
// // }

// // impl Write for MyI2cdev {
// //     type Error = LinuxI2CError;

// //     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
// //         MyI2cdev::write(self, addr, bytes).map_err(to_i2c_err)
// //     }
// // }

// // // Specific implementations of the Error can be done later
// // fn to_i2c_err(err: LinuxI2CError) -> LinuxI2CError {
// //     err
// // }


// fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {

//     // make linux driver, then shared bus, then mpu

//     // we can possibly define multiple i2cdev for multiple sensors since i2cdev is for a device
//     let i2c = I2cdev::new("/dev/i2c-1").map_err(Mpu6050Error::I2c).expect("");

//     let mpu = LinuxI2CDevice::new("/dev/i2c-1", 0x68);
//     // let mpu = LinuxI2CBus::new("/dev/i2c-1");


//     // println!("1\n");

//     // let mut delay = Delay;

//     // let i2c_bus = AtomicCell::new(i2c);
//     // let i2c_bus = shared_bus::BusManagerSimple::new(i2c);
//     // let i2c_new = i2c::AtomicDevice::new(&i2c_bus);
//     // try both 68 and 69
//     let mut mpu = Mpu6050::new(i2c);


//     println!("4\n");

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

//     Ok(())
// }

// fn main() {
//     println!("Hello World");
// }

// mod sensors;

// use crate::sensors::mpu6050::{lib::Mpu6050Error, lib::Mpu6050};

use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use embedded_hal::{delay::DelayNs, i2c::Error as I2CError};
use i2cdev::linux::{LinuxI2CError};

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut delay = Delay;
    let mut mpu = Mpu6050::new_with_addr(i2c, 0x68);
    
    mpu.init(&mut delay).unwrap();
    mpu.setup_motion_detection().unwrap();

    // let mut count: u8 = 0;

    // loop {
    //     if mpu.get_motion_detected().unwrap() {
    //         println!("YEAH BUDDY. Motion by axes: {:b}", mpu.read_byte(MOT_DETECT_STATUS::ADDR).unwrap());
    //         count += 1;
    //     }

    //     delay.delay_ms(10u32);

    //     if count > 5 {
    //         mpu.reset_device(&mut delay).unwrap();
    //         break;
    //     }
    // }

    Ok(())
}// use mpu6050::*;
// use linux_embedded_hal::{I2cdev, Delay};
// use i2cdev::linux::LinuxI2CError;
// // use embedded_hal::blocking::i2c::{WriteRead, Write};
// // use embedded_hal::blocking::delay::DelayMs;
// use shared_bus;
// use i2cdev::linux::{LinuxI2CDevice, LinuxI2CBus};
// use embedded_hal_bus::i2c;
// use embedded_hal_bus::util::AtomicCell;
// // use std::ops::{Deref, DerefMut};

// struct MyI2cdev(I2cdev);

// // https://docs.esp-rs.org/esp-idf-hal/src/esp_idf_hal/i2c.rs.html#202-205

// // impl WriteRead for MyI2cdev {
// //     type Error = LinuxI2CError;

// //     fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
// //         MyI2cdev::write_read(self, addr, bytes, buffer).map_err(to_i2c_err)
// //     }
// // }

// // impl Deref for MyI2cdev {
// //     type Target = I2cdev;
// //     fn deref(&self) -> &I2cdev {
// //         &self.0
// //     }
// // }

// // impl DerefMut for MyI2cdev {
// //     fn deref_mut(&mut self) -> &mut I2cdev {
// //         &mut self.0
// //     }
// // }

// // impl Write for MyI2cdev {
// //     type Error = LinuxI2CError;

// //     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
// //         MyI2cdev::write(self, addr, bytes).map_err(to_i2c_err)
// //     }
// // }

// // // Specific implementations of the Error can be done later
// // fn to_i2c_err(err: LinuxI2CError) -> LinuxI2CError {
// //     err
// // }


// fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {

//     // make linux driver, then shared bus, then mpu

//     // we can possibly define multiple i2cdev for multiple sensors since i2cdev is for a device
//     let i2c = I2cdev::new("/dev/i2c-1").map_err(Mpu6050Error::I2c).expect("");

//     let mpu = LinuxI2CDevice::new("/dev/i2c-1", 0x68);
//     // let mpu = LinuxI2CBus::new("/dev/i2c-1");


//     // println!("1\n");

//     // let mut delay = Delay;

//     // let i2c_bus = AtomicCell::new(i2c);
//     // let i2c_bus = shared_bus::BusManagerSimple::new(i2c);
//     // let i2c_new = i2c::AtomicDevice::new(&i2c_bus);
//     // try both 68 and 69
//     let mut mpu = Mpu6050::new(i2c);


//     println!("4\n");

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

//     Ok(())
// }

// fn main() {
//     println!("Hello World");
// }

// mod sensors;

// use crate::sensors::mpu6050::{lib::Mpu6050Error, lib::Mpu6050};

use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use embedded_hal::{delay::DelayNs, i2c::Error as I2CError};
use i2cdev::linux::{LinuxI2CError};

fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut delay = Delay;
    let mut mpu = Mpu6050::new_with_addr(i2c, 0x68);
    
    mpu.init(&mut delay).unwrap();
    mpu.setup_motion_detection().unwrap();

    // let mut count: u8 = 0;

    // loop {
    //     if mpu.get_motion_detected().unwrap() {
    //         println!("YEAH BUDDY. Motion by axes: {:b}", mpu.read_byte(MOT_DETECT_STATUS::ADDR).unwrap());
    //         count += 1;
    //     }

    //     delay.delay_ms(10u32);

    //     if count > 5 {
    //         mpu.reset_device(&mut delay).unwrap();
    //         break;
    //     }
    // }

    Ok(())
}