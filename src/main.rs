use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::{CdevPin, Delay};
// use ina219::address::Address;
// use ina219::SyncIna219;

use i2cdev::linux::{LinuxI2CError, LinuxI2CDevice};
// use i2cdev::core::I2CDevice;

pub mod ina219;
use ina219::ina219::{INA219,Opts};
use ina219::physic;

pub mod mpu6050;
use mpu6050::utils::*;
use mpu6050::device::{AccelRange, ACCEL_CONFIG, GyroRange, GYRO_CONFIG, ACCEL_HPF, WHOAMI, PWR_MGMT_1, ACCEL_SENS, GYRO_SENS};

pub mod one_wire_bus;
// use one_wire_bus::crc::*;
use crate::one_wire_bus::one_wire::OneWire;
use crate::one_wire_bus::error::{OneWireError};
use crate::one_wire_bus::address::{Address};

pub mod ds18b20;
use crate::ds18b20::ds18b20::Ds18b20; // , resolution::Resolution};
// use crate::ds18b20::ds18b20::start_simultaneous_temp_measurement;

use serialport;

pub mod neo6m;
use crate::neo6m::device::*;
use ublox::*;
use chrono::prelude::*;

fn main() -> Result<(), (linux_embedded_hal::gpio_cdev::Error, LinuxI2CError, OneWireError, std::io::Error)> {

    // Let's start by checking the status of the I2C pins
    // and potentially opening them
    // If the line is held by the kernel,
    // it needs to be freed

    println!("Check the status of PIN7 (I2C1_SDA)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(20).expect("Got the line");
    println!("PIN7 (I2C1_SDA) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN7 (I2C_SDA) is held at low {:?}\n", line.info().expect("").is_active_low());

    println!("Check the status of PIN6 (I2C1_SCL)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(19).expect("Got the line");
    println!("PIN6 (I2C1_SCL) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN6 (I2C1_SCL) is held at low {:?}\n", line.info().expect("").is_active_low());

    println!("Check the status of PIN11 (I2C1_SDA)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(21).expect("Got the line");
    println!("PIN11 (I2C1_SDA) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN11 (I2C_SDA) is held at low {:?}\n", line.info().expect("").is_active_low());

    println!("Check the status of PIN12 (I2C1_SCL)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(18).expect("Got the line");
    println!("PIN12 (I2C1_SCL) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN12 (I2C1_SCL) is held at low {:?}\n", line.info().expect("").is_active_low());

    println!("Check the status of PIN29 (GP22)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(4).expect("Got the line");
    println!("PIN29 (GP22) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN29 (GP22) is held at low {:?}", line.info().expect("").is_active_low());
    println!("PIN29 (GP22) is held at open drain {:?}", line.info().expect("").is_open_drain());
    println!("PIN29 (GP22) is held at open source {:?}\n", line.info().expect("").is_open_source());

    println!("Instantiate MPU6050");
    let i2c: Rc<RefCell<LinuxI2CDevice>> = Rc::new(RefCell::new(LinuxI2CDevice::new("/dev/i2c-1", 0x68).expect("")));
    let mpu = Mpu6050::new(i2c.clone());
    mpu.wake();
    mpu.read_slave_addr();
    mpu.init();
    
    println!("Instantiate INA219");
    let device: LinuxI2CDevice = LinuxI2CDevice::new("/dev/i2c-2", 0x40).expect("");
    let opt = Opts::new(0x40,100 * physic::MilliOhm,1 * physic::Ampere);
    let mut ina = INA219::new(device, opt);
    ina.init().unwrap();

    println!("Instantiate NEO6M");
    let mut port = serialport::new("/dev/ttyS1", 9600).timeout(Duration::from_millis(250)).open().unwrap();
    let mut neo6m = NEO6M::new(port);

    // GP22 PIN2
    // println!("Instantiate OneWire and DS18B20");
    // let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    // let line = chip.get_line(4).unwrap();
    // let line_handle = line.request(LineRequestFlags::OUTPUT, 1, "one-wire-ds18b20").unwrap();
    // println!("PIN29 (GP22) is held at low {:?}", line.info().expect("").is_active_low());
    // println!("PIN29 (GP22) is held at open drain {:?}", line.info().expect("").is_open_drain());
    // println!("PIN29 (GP22) is held at open source {:?}\n", line.info().expect("").is_open_source());
    // let one_wire_pin = CdevPin::new(line_handle).unwrap();
    // let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
    // let mut ds18b = Ds18b20::new(Address(0x28)).unwrap();
    // println!("Find device now");
    // if let Some((device_address, _)) = one_wire_bus.device_search(None, false, &mut Delay).unwrap() {
    //     println!("Print address is : {}", device_address.0);
    //     ds18b = Ds18b20::new(device_address).unwrap();
    // }

    // thread::sleep(Duration::from_millis(1000));

    println!("Take readings");
    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().expect("");
        println!("r/p: {:?}", acc);
        // get sensor temp
        let temp = mpu.get_temp().expect("");
        println!("temp: {:?}c", temp);
        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().expect("");
        println!("gyro: {:?}", gyro);
        // // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().expect("");
        println!("acc: {:?}", acc);

        thread::sleep(Duration::from_millis(500));

        let pm = ina.sense().unwrap();
        println!("{:?}", pm);

        thread::sleep(Duration::from_millis(500));

        // let sensor_data = match ds18b.read_data(&mut one_wire_bus, &mut Delay) {
        //     Ok(sensor_data) =>  { println!("Temperature is {}°C", sensor_data.temperature); }
        //     Err(_) => println!("Error in measuring temperature")
        // };

        neo6m.read_gps_data();         

        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}