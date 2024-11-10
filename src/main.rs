#![allow(dead_code)]
use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    BLEAdvertisementData, BLEDevice, BLEHIDDevice,
};
use esp_idf_hal::{delay::FreeRtos, gpio::*, peripherals::Peripherals};
use esp_idf_sys as _;
use game_pad::GamePad;

mod game_pad;
mod report_descriptor;

#[repr(packed)]
struct KeysReport {
    // Like: ctrl, alt e etc
    pub modifiers: u8,
    // I really don't fuck known
    pub reserved: u8,
    // Used to pass keys pressed (if all empty it means release)
    pub keys: [u8; 6],
}

fn main() {
    esp_idf_sys::link_patches();

    let ble_device = BLEDevice::take();

    ble_device
        .security()
        .set_auth(AuthReq::all())
        .set_io_cap(SecurityIOCap::NoInputNoOutput)
        .resolve_rpa();

    let server = ble_device.get_server();

    server.on_connect(|_s, _c| {
        println!("Connected {:#?}", _c);
    });

    server.on_disconnect(|_c, _r| {
        println!("Deconnected {:#?} | reason: {:#?}", _c, _r.err().unwrap());
    });

    let mut hid = BLEHIDDevice::new(server);

    let input_keyboard = hid.input_report(report_descriptor::KEYBOARD_ID);
    let _output_keyboard = hid.output_report(report_descriptor::KEYBOARD_ID);
    let _input_media_keys = hid.input_report(report_descriptor::MEDIA_KEYS_ID);

    hid.manufacturer("Leydev");
    hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
    hid.hid_info(0x00, 0x01);

    hid.report_map(report_descriptor::HID_REPORT_DISCRIPTOR);

    hid.set_battery_level(100);

    let ble_advertising = ble_device.get_advertising();
    ble_advertising
        .lock()
        .scan_response(false)
        .set_data(
            BLEAdvertisementData::new()
                .name("GuittarPad")
                .appearance(964)
                .add_service_uuid(hid.hid_service().lock().uuid()),
        )
        .unwrap();

    ble_advertising.lock().start().unwrap();

    let mut keys_report = KeysReport {
        modifiers: 0,
        reserved: 0,
        keys: [0; 6],
    };

    let peripherals = Peripherals::take().unwrap();

    let mut led_status = PinDriver::output(peripherals.pins.gpio32).unwrap();

    let mut pick_down_btn = PinDriver::input(peripherals.pins.gpio33).unwrap();
    let mut pick_up_btn = PinDriver::input(peripherals.pins.gpio25).unwrap();

    let mut cro_btn = PinDriver::input(peripherals.pins.gpio26).unwrap();
    let mut cir_btn = PinDriver::input(peripherals.pins.gpio27).unwrap();
    let mut srq_btn = PinDriver::input(peripherals.pins.gpio14).unwrap();
    let mut tri_btn = PinDriver::input(peripherals.pins.gpio12).unwrap();
    let mut l1_btn = PinDriver::input(peripherals.pins.gpio13).unwrap();

    pick_down_btn.set_pull(Pull::Up).unwrap();
    pick_up_btn.set_pull(Pull::Up).unwrap();

    cro_btn.set_pull(Pull::Up).unwrap();
    cir_btn.set_pull(Pull::Up).unwrap();
    srq_btn.set_pull(Pull::Up).unwrap();
    tri_btn.set_pull(Pull::Up).unwrap();
    l1_btn.set_pull(Pull::Up).unwrap();

    let mut game_pad = game_pad::GamePad::new();

    let mut cache_report: [u8; 6] = [0; 6];

    let mut send_report = |game_pad: &GamePad| {
        if *game_pad.strum_up.get() as u8 > 0x00 {
            keys_report.keys[0] = *game_pad.strum_up.get() as u8;
        } else {
            keys_report.keys[0] = *game_pad.strum_down.get() as u8;
        }
        
        keys_report.keys[1] = *game_pad.cross.get() as u8;
        keys_report.keys[2] = *game_pad.circle.get() as u8;
        keys_report.keys[3] = *game_pad.square.get() as u8;
        keys_report.keys[4] = *game_pad.triangule.get() as u8;
        keys_report.keys[5] = *game_pad.l1.get() as u8;

        if keys_report.keys != cache_report {
            cache_report = keys_report.keys;
            input_keyboard.lock().set_from(&keys_report).notify();
        }
    };

    loop {
        if server.connected_count() > 0 {
            led_status.set_high().unwrap();

            game_pad.strum_up.status = pick_up_btn.is_low();
            game_pad.strum_down.status = pick_down_btn.is_low();
            //game_pad.enter.status = pick_down_btn.is_low() || 
            game_pad.cross.status = cro_btn.is_low();
            game_pad.circle.status = cir_btn.is_low();
            game_pad.square.status = srq_btn.is_low();
            game_pad.triangule.status = tri_btn.is_low();
            game_pad.l1.status = l1_btn.is_low();
    
            send_report(&game_pad);
        }

        FreeRtos::delay_ms(1);
        
    }
}


