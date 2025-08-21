use std::rc::Rc;
use std::cell::RefCell;
use embedded_sdmmc::{Error, Mode, SdCard, SdCardError, TimeSource, VolumeIdx, VolumeManager};
use embedded_sdmmc::sdcard::AcquireOpts;
use crate::sdcard::utils;
use embedded_sdmmc::sdcard::proto::crc7;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::{Delay, SpidevDevice};

// pub fn new_sd_card<S, D>(spi: Rc<RefCell<S>>, delay: D) -> Result<(), Error<SdCardError>>
pub fn new_sd_card<S, D>(spi: S, delay: D) -> Result<(), Error<SdCardError>>
where
    S: embedded_hal::spi::SpiDevice,
    D: embedded_hal::delay::DelayNs,
{
    println!("In this SDCard function");
    // let sdcard = SdCard::new(*spi.borrow_mut(), delay);
    let sdcard = SdCard::new(spi, delay);
    // init_74_cycles(spi, delay).expect("Error doing the 74 cycles initialization");

    let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
    // let do_line = chip.get_line(22).expect("Did not get the DO line").request(LineRequestFlags::OUTPUT, 1, "sd-card-do").unwrap();
    let cs_line = chip.get_line(18).expect("Did not get the CS line").request(LineRequestFlags::OUTPUT, 1, "sd-card-cs").unwrap();
    let mut buf = [0xFF; 10];

    cs_line.set_value(1).unwrap();
    sdcard.spi(|spi| {
        spi.write(&buf).expect("Error in SPI write");
    }); // .map_err(|_e| Error::Transport)?;

    cs_line.set_value(0).unwrap();
    println!("DONE");

    // Send the 6 byte CMD 0 command

    println!("Card size is {} bytes", sdcard.num_bytes().expect("Error reading the card size"));
    Ok(())
}





// pub fn init_74_cycles<S, D>(spi: S, delay: D) -> Result<(), ()>
// where
//     S: embedded_hal::spi::SpiDevice,
//     D: embedded_hal::delay::DelayNs,
// {
    
//     // let mut buf = [
//     //     0x40 | command,
//     //     (arg >> 24) as u8,
//     //     (arg >> 16) as u8,
//     //     (arg >> 8) as u8,
//     //     arg as u8,
//     //     0,
//     // ];
//     // buf[5] = crc7(&buf[0..5]);

//     let mut chip = Chip::new("/dev/gpiochip4").expect("Failed to open GPIO chip");
//     let do_line = chip.get_line(22).expect("Did not get the DO line").request(LineRequestFlags::OUTPUT, 1, "sd-card-do").unwrap();
//     let cs_line = chip.get_line(18).expect("Did not get the CS line").request(LineRequestFlags::OUTPUT, 1, "sd-card-cs").unwrap();

//     let mut buf = [0xFF; 10];

//     for i in 1..75 {
//         // set CS and DO lines high 
//         // self.write_bytes(&buf)?; includes the follows
//         do_line.set_value(1).unwrap();
//         cs_line.set_value(1).unwrap();

//         spi.write(&buf).expect("Error in SPI write"); // .map_err(|_e| Error::Transport)?;
//     }

//     cs_line.set_value(0).unwrap();
    
//     Ok(())
// }