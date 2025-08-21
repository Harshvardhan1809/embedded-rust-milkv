use std::thread;
use std::time::Duration;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use chrono::Local;
use linux_embedded_hal::gpio_cdev::Chip;
use linux_embedded_hal::{Delay, SpidevDevice};
use linux_embedded_hal::spidev::{SpidevOptions, SpiModeFlags};
// use linux_embedded_hal::{CdevPin, Delay};

use linux_embedded_hal::i2cdev::linux::{LinuxI2CError, LinuxI2CDevice};
// use linux_embedded_hal::spidev::{Spidev};

pub mod ina219;
use ina219::ina219::{INA219,Opts};
use ina219::physic;

pub mod mpu6050;
use mpu6050::utils::*;
use mpu6050::device::{AccelRange, ACCEL_CONFIG, GyroRange, GYRO_CONFIG, ACCEL_HPF, WHOAMI, PWR_MGMT_1, ACCEL_SENS, GYRO_SENS};

// pub mod one_wire_bus;
// use one_wire_bus::crc::*;
// use crate::one_wire_bus::one_wire::OneWire;
// use crate::one_wire_bus::error::{OneWireError};
// use crate::one_wire_bus::address::{Address};

// pub mod ds18b20;
// use crate::ds18b20::ds18b20::Ds18b20; // , resolution::Resolution};
// use crate::ds18b20::ds18b20::start_simultaneous_temp_measurement;

use serialport;

pub mod neo6m;
use crate::neo6m::device::*;

use owo_colors::OwoColorize;

use embedded_sdmmc::{Error, Mode, SdCard, SdCardError, TimeSource, ShortFileName, VolumeIdx};
pub mod sdcard;
use crate::sdcard::linux::*;
use crate::sdcard::{utils as SDCard};
use embedded_sdmmc::VolumeManager;

// type Directory<'a> = embedded_sdmmc::Directory<'a, LinuxBlockDevice, Clock, 8, 4, 4>;
// type VolumeManager = embedded_sdmmc::VolumeManager<LinuxBlockDevice, Clock, 8, 4, 4>;

// use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

fn main() -> Result<(), (linux_embedded_hal::gpio_cdev::Error, LinuxI2CError, std::io::Error, embedded_sdmmc::Error<std::io::Error>)> {

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

    println!("{}", format!("Instantiate MPU6050").blue());
    let i2c: Rc<RefCell<LinuxI2CDevice>> = Rc::new(RefCell::new(LinuxI2CDevice::new("/dev/i2c-1", 0x68).expect("")));
    let mpu = Mpu6050::new(i2c.clone());
    mpu.wake().unwrap();
    mpu.read_slave_addr().unwrap();
    mpu.init();
    
    println!("{}", format!("Instantiate INA219").blue());
    let device: LinuxI2CDevice = LinuxI2CDevice::new("/dev/i2c-2", 0x40).expect("");
    let opt = Opts::new(0x40,100 * physic::MilliOhm,1 * physic::Ampere);
    let mut ina = INA219::new(device, opt);
    ina.init().unwrap();

    println!("{}", format!("Instantiate NEO6M").blue());
    let port = serialport::new("/dev/ttyS1", 9600).timeout(Duration::from_millis(250)).open().unwrap();
    let mut neo6m = NEO6M::new(port);

    println!("{}", format!("Instantiate SD Card").blue());
    // let mut spi: Rc<RefCell<SpidevDevice>> = Rc::new(RefCell::new(SpidevDevice::open("/dev/spidev0.0").unwrap()));
    let mut spi = SpidevDevice::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new().bits_per_word(8).max_speed_hz(400000).lsb_first(false).mode(SpiModeFlags::SPI_MODE_0).build();
    spi.configure(&options).unwrap();
    // SDCard::new_sd_card(spi, Delay);
    let sdcard = SdCard::new(spi, Delay);
    println!("{}", format!("Card size is {} bytes", sdcard.num_bytes().expect("Error reading the card size")).on_cyan());
    let volume_mgr = VolumeManager::new(sdcard, Clock);

    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).expect("Error opening volume");
    println!("{}", format!("Volume 0: {:?}", volume0).on_cyan());
    // println!("Volume 0: {:?}", volume0);
    // First see if the file exists or not, then create it
    let mut file_exists = false;
    const FILE_TO_CREATE: &str = "SENSOR.csv";

    let root_dir = volume0.open_root_dir().expect("Error opening dir");
    let mut children = Vec::new();
    root_dir.iterate_dir(|entry| {
        println!("{:12} {:9} {} {}", entry.name, entry.size, entry.mtime,
            if entry.attributes.is_directory() {
                "<DIR>"
            } else {
                ""
            }
        );
        if entry.attributes.is_directory() && entry.name != ShortFileName::parent_dir() && entry.name != ShortFileName::this_dir() {
            children.push(entry.name.clone());
        }

        if entry.name == ShortFileName::create_from_str(FILE_TO_CREATE).expect("REASON") {
            file_exists = true;
        }
    }).expect("Error iterating over directory");

    println!("Sensor log {}", file_exists);
    if !file_exists {
        println!("{}", format!("Create sensor log file").on_cyan());
        let f = root_dir.open_file_in_dir(FILE_TO_CREATE, Mode::ReadWriteCreate).expect("as");
        f.close(); // volume_mgr.close_file(f).expect("Closing file");
    }

    let sensor_log_csv = root_dir.open_file_in_dir(FILE_TO_CREATE, Mode::ReadWriteAppend).expect("WD");
    println!("{:?}", sensor_log_csv);

    // GP22 PIN2
    // println!("Instantiate OneWire and DS18B20");
    // let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    // let line = chip.get_line(4).unwrap();
    // let line_handle = line.request(LineRequestFlags::OUTPUT, 1, "one-wire-ds18b20").unwrap();
    // let one_wire_pin = CdevPin::new(line_handle).unwrap();
    // let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();
    // let mut ds18b = Ds18b20::new(Address(0x28)).unwrap();
    // if let Some((device_address, _)) = one_wire_bus.device_search(None, false, &mut Delay).unwrap() {
    //     println!("Print address is : {}", device_address.0);
    //     ds18b = Ds18b20::new(device_address).unwrap();
    // }

    println!("{}", format!("Take readings\n").green());
    let mut timestamp = Local::now().format("%Y-%m-%dT%H:%M:%SZ");
    println!("{}", format!("[{}]mpu6050,roll,pitch,temp,gyrox,gyroy,gyroz,accx,accy,accz,statuscode", timestamp).yellow());
    println!("{}", format!("[{}]ina219,voltage,shunt_voltage,current,power,statuscode", timestamp).yellow());
    println!("{}", format!("[{}]gps-gga,source,latitude,longitude,statuscode", timestamp).yellow());
    println!("{}", format!("[{}]gps-rmc,source,speed,bearing", timestamp).yellow());
    loop {
        // get roll and pitch estimate
        let accrp = mpu.get_acc_angles().expect("");
        let temp = mpu.get_temp().expect("");
        let gyro = mpu.get_gyro().expect("");
        let acc = mpu.get_acc().expect("");
        timestamp = Local::now().format("%Y-%m-%dT%H:%M:%SZ");
        let mpu6050_log = format!("[{}]mpu6050,{:?},{:?},{:?}c,{:?},{:?},{:?},{:?},{:?},{:?},1\n", timestamp, accrp.x, accrp.y, temp, gyro.x, gyro.y, gyro.z, acc.x, acc.y, acc.z);
        print!("{}", mpu6050_log.green());
        println!("Bytes written {:?}", sensor_log_csv.write(mpu6050_log.as_bytes()).unwrap());
        thread::sleep(Duration::from_millis(500));

        let pm = ina.sense().unwrap();
        timestamp = Local::now().format("%Y-%m-%dT%H:%M:%SZ");
        let ina219_log = format!("[{}]ina219,{},{},{},{},1\n", timestamp, pm.Shunt, pm.Voltage, pm.Current, pm.Power);
        print!("{}", ina219_log.green());
        sensor_log_csv.write(ina219_log.as_bytes());
        println!("Bytes written {:?}", sensor_log_csv.write(ina219_log.as_bytes()).unwrap());
        thread::sleep(Duration::from_millis(500));

        // let sensor_data = match ds18b.read_data(&mut one_wire_bus, &mut Delay) {
        //     Ok(sensor_data) =>  { println!("Temperature is {}°C", sensor_data.temperature); }
        //     Err(_) => println!("Error in measuring temperature")
        // };

        timestamp = Local::now().format("%Y-%m-%dT%H:%M:%SZ");
        let gps: Vec<String> = neo6m.read_gps_data().unwrap();  
        for sentence in gps {
            if sentence.contains("kts") {
                let gps_kts_log = format!("[{}]gps-rmc,{},1\n", timestamp, sentence);
                print!("{}", gps_kts_log.green());
                // sensor_log_csv.write(gps_kts_log.as_bytes());
                println!("Bytes written {:?}", sensor_log_csv.write(gps_kts_log.as_bytes()).unwrap());

            } else {
                let gps_gga_log = format!("[{}]gps-gga,{},1\n", timestamp, sentence);
                print!("{}", gps_gga_log.green());
                // sensor_log_csv.write(gps_gga_log.as_bytes());
                println!("Bytes written {:?}", sensor_log_csv.write(gps_gga_log.as_bytes()).unwrap());
            }
        }    

        thread::sleep(Duration::from_millis(5000));
    }
}
