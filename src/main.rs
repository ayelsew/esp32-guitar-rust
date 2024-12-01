use std::num::NonZero;

use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    BLEAdvertisementData, BLEDevice, BLEHIDDevice,
};

use esp_idf_hal::{gpio::*, peripherals::Peripherals, task::notification::Notification};
use esp_idf_sys::{self as _};

mod report_descriptor;

#[repr(packed)]
struct Report {
    pub buttons: u8,
}

fn generate_report(buttons: &[bool; 8], report: &mut Report) {
    for (i, &button) in buttons.iter().enumerate() {
        if button {
            report.buttons |= 1 << i;
        } else {
            report.buttons &= !(1 << i);
        }
    }
}

fn main() {
    let _ = BLEDevice::set_device_name("GuittarPad").unwrap();
    esp_idf_sys::link_patches();
    let ble_device = BLEDevice::take();

    ble_device
        .security()
        .set_auth(AuthReq::Bond)
        .set_io_cap(SecurityIOCap::NoInputNoOutput)
        .resolve_rpa();

    let server = ble_device.get_server();

    server.on_connect(|_s, _c| {
        println!("Connected {:#?}", _c);
    });

    server.on_disconnect(|_c, _r| {
        println!("Disconnected {:#?} | reason: {:#?}", _c, _r.err().unwrap());
    });

    let mut hid = BLEHIDDevice::new(server);

    let input_keyboard = hid.input_report(report_descriptor::REPORT_ID);

    hid.manufacturer("Leydev");
    hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
    hid.hid_info(0x00, 0x01);

    hid.report_map(report_descriptor::REPORT_MAP);

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

    let mut keys_report = Report { buttons: 0x00 };

    // MAP KEY
    let peripherals = Peripherals::take().unwrap();

    let mut strum_down_btn = PinDriver::input(peripherals.pins.gpio33).unwrap();
    let mut strum_up_btn = PinDriver::input(peripherals.pins.gpio25).unwrap();

    let mut green_btn = PinDriver::input(peripherals.pins.gpio26).unwrap();
    let mut red_btn = PinDriver::input(peripherals.pins.gpio27).unwrap();
    let mut yellow_btn = PinDriver::input(peripherals.pins.gpio14).unwrap();
    let mut blue_btn = PinDriver::input(peripherals.pins.gpio12).unwrap();
    let mut orange_btn = PinDriver::input(peripherals.pins.gpio13).unwrap();

    let mut start_btn = PinDriver::input(peripherals.pins.gpio4).unwrap();

    strum_down_btn.set_pull(Pull::Up).unwrap();
    strum_up_btn.set_pull(Pull::Up).unwrap();
    green_btn.set_pull(Pull::Up).unwrap();
    red_btn.set_pull(Pull::Up).unwrap();
    yellow_btn.set_pull(Pull::Up).unwrap();
    blue_btn.set_pull(Pull::Up).unwrap();
    orange_btn.set_pull(Pull::Up).unwrap();
    start_btn.set_pull(Pull::Up).unwrap();

    // SETUP BUTTON INTERRUPT

    strum_down_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    strum_up_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    green_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    red_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    yellow_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    blue_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    orange_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    start_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();

    let notification = Notification::new();
    let mut buttons: [bool; 8] = [false; 8];

    loop {
        let waker_1 = notification.notifier();
        let waker_2 = notification.notifier();
        let waker_3 = notification.notifier();
        let waker_4 = notification.notifier();
        let waker_5 = notification.notifier();
        let waker_6 = notification.notifier();
        let waker_7 = notification.notifier();
        let waker_8 = notification.notifier();

        unsafe {
            strum_down_btn
                .subscribe_nonstatic(move || {
                    waker_1.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            strum_up_btn
                .subscribe_nonstatic(move || {
                    waker_2.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            green_btn
                .subscribe_nonstatic(move || {
                    waker_3.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            red_btn
                .subscribe_nonstatic(move || {
                    waker_4.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            yellow_btn
                .subscribe_nonstatic(move || {
                    waker_5.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            blue_btn
                .subscribe_nonstatic(move || {
                    waker_6.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            orange_btn
                .subscribe_nonstatic(move || {
                    waker_7.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            start_btn
                .subscribe_nonstatic(move || {
                    waker_8.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
        }

        // Enable interrupts
        strum_down_btn.enable_interrupt().unwrap();
        strum_up_btn.enable_interrupt().unwrap();
        green_btn.enable_interrupt().unwrap();
        red_btn.enable_interrupt().unwrap();
        yellow_btn.enable_interrupt().unwrap();
        blue_btn.enable_interrupt().unwrap();
        orange_btn.enable_interrupt().unwrap();
        start_btn.enable_interrupt().unwrap();

        buttons[0] = green_btn.is_low();
        buttons[1] = red_btn.is_low();
        buttons[2] = yellow_btn.is_low();
        buttons[3] = blue_btn.is_low();
        buttons[4] = orange_btn.is_low();
        buttons[5] = strum_down_btn.is_low();
        buttons[6] = strum_up_btn.is_low();
        buttons[7] = start_btn.is_low();

        generate_report(&buttons, &mut keys_report);

        input_keyboard.lock().set_from(&keys_report).notify();

        // Block until any waker.notify() call
        notification.wait_any();

        // It check value and send again to partially fix a bug
        // Some times the "release button" is't sent. So it is for that
        buttons[0] = green_btn.is_low();
        buttons[1] = red_btn.is_low();
        buttons[2] = yellow_btn.is_low();
        buttons[3] = blue_btn.is_low();
        buttons[4] = orange_btn.is_low();
        buttons[5] = strum_down_btn.is_low();
        buttons[6] = strum_up_btn.is_low();
        buttons[7] = start_btn.is_low();

        generate_report(&buttons, &mut keys_report);

        input_keyboard.lock().set_from(&keys_report).notify();
    }
}
