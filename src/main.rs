use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use linux_embedded_hal::gpio_cdev::Chip;
use i2cdev::linux::{LinuxI2CError, LinuxI2CDevice};
use i2cdev::core::I2CDevice;

pub mod mpu6050;
use mpu6050::utils::*;
use mpu6050::device::{AccelRange, ACCEL_CONFIG, GyroRange, GYRO_CONFIG, ACCEL_HPF, INT_STATUS, WHOAMI, PWR_MGMT_1, MOT_DETECT_STATUS, ACCEL_SENS, GYRO_SENS};

fn main() -> Result<(), (linux_embedded_hal::gpio_cdev::Error, LinuxI2CError)> {

    // Let's start by checking the status of the I2C pins
    // and potentially opening them
    // If the line is held by the kernel,
    // it needs to be freed

    println!("Check the status of PIN7 (I2C1_SDA)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(20).expect("Got the line");
    println!("PIN11 (I2C1_SDA) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN11 (I2C_SDA) is held at low {:?}\n", line.info().expect("").is_active_low());

    println!("Check the status of PIN6 (I2C1_SCL)");
    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    let line = chip.get_line(19).expect("Got the line");
    println!("PIN12 (I2C1_SCL) is held by the kernel {:?}", line.info().expect("").is_kernel());
    println!("PIN12 (I2C1_SCL) is held at low {:?}\n", line.info().expect("").is_active_low());

    // Now we instantiate the sensor,
    // wake it up, initialize its registers
    // and take readings

    println!("Instantiate device");
    let i2c: Rc<RefCell<LinuxI2CDevice>> = Rc::new(RefCell::new(LinuxI2CDevice::new("/dev/i2c-1", 0x68).expect("")));
    let mpu = Mpu6050::new(i2c.clone());

    println!("Wake up the device");
    let _ = mpu.wake();

    println!("Read the slave address");
    let _ = mpu.read_slave_addr();

    println!("Set ranges");
    mpu.init();

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

        println!("\n");

        thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}