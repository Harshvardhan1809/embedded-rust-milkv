use embedded_hal::spi::SpiDevice;

pub struct SDCardDriver<SPI> {
    spi: SPI
}

impl<SPI> MyDriver<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn read_foo(&mut self) -> Result<[u8; 2], MyError<SPI::Error>> {
        let mut buf = [0; 2];

        // `transaction` asserts and deasserts CS for us. No need to do it manually!
        self.spi.transaction(&mut [
            Operation::Write(&[0x90]),
            Operation::Read(&mut buf),
        ]).map_err(MyError::Spi)?;

        Ok(buf)
    }
}

#[derive(Copy, Clone, Debug)]
enum MyError<SPI> {
    Spi(SPI),
    // Add other errors for your driver here.
}
