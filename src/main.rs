// activate spi, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

use anyhow::Result;
use core::time::Duration;
use nidus::*;
use std::thread;

fn main() {
    if let Err(e) = run() {
        eprintln!("Program exited early with error: {}", e);
    }
}

#[cfg(not(feature = "simulator"))]
fn run() -> Result<()> {
    use bme680::*;
    #[cfg(feature = "epd2in9")]
    use epd_waveshare::epd2in9::{Display2in9 as DisplayEPD, EPD2in9 as EPD};
    #[cfg(feature = "epd4in2")]
    use epd_waveshare::epd4in2::{Display4in2 as DisplayEPD, EPD4in2 as EPD};
    use epd_waveshare::prelude::*;
    use linux_embedded_hal::*;
    use linux_embedded_hal::{
        spidev::{self, SpidevOptions},
        sysfs_gpio::Direction,
        Delay, Pin, Spidev,
    };
    use log::*;

    env_logger::init();

    status_influx(Status::STARTUP, None);

    // Configure SPI
    // Settings are taken from
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs = Pin::new(26); //BCM7 CE0
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(5); //pin 29
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");
    //busy.set_value(1).expect("busy Value set to 1");

    let dc = Pin::new(6); //pin 31 //bcm6
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let rst = Pin::new(16); //pin 36 //bcm16
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    let mut delay = Delay {};

    let mut epd = EPD::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");

    println!("Test all the rotations");
    let mut display = DisplayEPD::default();

    let i2c = I2cdev::new("/dev/i2c-1").expect("i2cdev device");
    let mut bme = Bme680::init(i2c, Delay {}, I2CAddress::Secondary).expect("i2cdev device");

    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_gas_measurement(Duration::from_millis(1500), 320, 25)
        .with_temperature_offset(-2.2)
        .with_run_gas(true)
        .build();

    let profile_dur = bme.get_profile_dur(&settings.0).expect("i2cdev device");
    info!("Profile duration {:?}", profile_dur);
    info!("Setting sensor settings");
    bme.set_sensor_settings(settings).expect("i2cdev device");
    info!("Setting forced power modes");
    bme.set_sensor_mode(PowerMode::ForcedMode)
        .expect("i2cdev device");

    let sensor_settings = bme.get_sensor_settings(settings.1);
    info!("Sensor settings: {:?}", sensor_settings);

    let power_mode = bme.get_sensor_mode();
    info!("Sensor power mode: {:?}", power_mode);
    info!("Setting forced power modes");
    bme.set_sensor_mode(PowerMode::ForcedMode)
        .expect("i2cdev device");
    info!("Retrieving sensor data");
    let (data, _state) = bme.get_sensor_data().expect("i2cdev device");
    info!("Sensor Data {:?}", data);
    info!("Temperature {}°C", data.temperature_celsius());
    info!("Pressure {}hPa", data.pressure_hpa());
    info!("Humidity {}%", data.humidity_percent());
    info!("Gas Resistence {}Ω", data.gas_resistance_ohm());

    #[cfg(feature = "epd2in9")]
    display.set_rotation(DisplayRotation::Rotate90);

    loop {
        weather(&mut display);

        time(&mut display);

        if let Err(e) = sensor(&mut display, &mut bme) {
            error("sensor reading", e);
        }

        if let Err(e) = epd.update_and_display_frame(&mut spi, &display.buffer()) {
            error("epd update & display", e);
        }

        //thread::sleep(Duration::from_millis(3000));
        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(feature = "simulator")]
pub fn run() -> Result<()> {
    use embedded_graphics::geometry::Size;
    use embedded_graphics_simulator::*;

    status_influx(Status::STARTUP, None);

    let mut display = SimulatorDisplay::new(Size::new(width() as u32, height() as u32));
    let output_settings = OutputSettingsBuilder::new()
        //.theme(BinaryColorTheme::LcdWhite)
        .scale(1)
        .build();
    let mut window = Window::new("Nidus", &output_settings);

    'running: loop {
        weather(&mut display);

        time(&mut display);

        if let Err(e) = sensor(&mut display) {
            error("sensor reading", e);
        }

        window.update(&display);
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }

        thread::sleep(Duration::from_millis(3000));
    }

    Ok(())
}
