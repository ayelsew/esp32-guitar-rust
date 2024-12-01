#![allow(dead_code)]

use std::num::NonZero;

use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    BLEAdvertisementData, BLEDevice, BLEHIDDevice,
};

use esp_idf_hal::{gpio::*, peripherals::Peripherals, task::notification::Notification};
use esp_idf_sys::{self as _};

mod report_descriptor;

const STRUM: u8 = 0x51;
const ENTER: u8 = 0x28;
const STRUM_UP: u8 = 0x52;
const STRUM_DOWN: u8 = 0x51;
const Q: u8 = 0x14;
const W: u8 = 0x1a;
const E: u8 = 0x08;
const R: u8 = 0x15;
const T: u8 = 0x17;
const H: u8 = 0x0b;
const NULL: u8 = 0x00;

#[repr(packed)]
struct Report {
    pub buttons: u8,
}

fn generate_report(buttons: &[bool; 8], report: &mut Report) {
    println!("{:08b}", report.buttons);
    for (i, &button) in buttons.iter().enumerate() {
        if button {
            report.buttons |= 1 << i;
        } else {
            report.buttons &= !(1 << i);
        }
    }
}

fn main() {
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
        println!("Deconnected {:#?} | reason: {:#?}", _c, _r.err().unwrap());
    });

    let mut hid = BLEHIDDevice::new(server);

    let input_keyboard = hid.input_report(report_descriptor::KEYBOARD_ID);
    let _output_keyboard = hid.output_report(report_descriptor::KEYBOARD_ID);

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

    let mut keys_report = Report { buttons: 0x00 };

    // MAP KEY
    let peripherals = Peripherals::take().unwrap();

    let mut pick_down_btn = PinDriver::input(peripherals.pins.gpio33).unwrap();
    let mut pick_up_btn = PinDriver::input(peripherals.pins.gpio25).unwrap();

    let mut cro_btn = PinDriver::input(peripherals.pins.gpio26).unwrap();
    let mut cir_btn = PinDriver::input(peripherals.pins.gpio27).unwrap();
    let mut srq_btn = PinDriver::input(peripherals.pins.gpio14).unwrap();
    let mut tri_btn = PinDriver::input(peripherals.pins.gpio12).unwrap();
    let mut l1_btn = PinDriver::input(peripherals.pins.gpio13).unwrap();

    let mut start_btn = PinDriver::input(peripherals.pins.gpio4).unwrap();

    pick_down_btn.set_pull(Pull::Up).unwrap();
    pick_up_btn.set_pull(Pull::Up).unwrap();
    cro_btn.set_pull(Pull::Up).unwrap();
    cir_btn.set_pull(Pull::Up).unwrap();
    srq_btn.set_pull(Pull::Up).unwrap();
    tri_btn.set_pull(Pull::Up).unwrap();
    l1_btn.set_pull(Pull::Up).unwrap();
    start_btn.set_pull(Pull::Up).unwrap();

    // SETUP BUTTON INTERRUPT

    pick_down_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    pick_up_btn
        .set_interrupt_type(InterruptType::AnyEdge)
        .unwrap();
    cro_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    cir_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    srq_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    tri_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
    l1_btn.set_interrupt_type(InterruptType::AnyEdge).unwrap();
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
            pick_down_btn
                .subscribe_nonstatic(move || {
                    waker_1.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            pick_up_btn
                .subscribe_nonstatic(move || {
                    waker_2.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            cro_btn
                .subscribe_nonstatic(move || {
                    waker_3.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            cir_btn
                .subscribe_nonstatic(move || {
                    waker_4.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            srq_btn
                .subscribe_nonstatic(move || {
                    waker_5.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            tri_btn
                .subscribe_nonstatic(move || {
                    waker_6.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            l1_btn
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
        pick_down_btn.enable_interrupt().unwrap();
        pick_up_btn.enable_interrupt().unwrap();
        cro_btn.enable_interrupt().unwrap();
        cir_btn.enable_interrupt().unwrap();
        srq_btn.enable_interrupt().unwrap();
        tri_btn.enable_interrupt().unwrap();
        l1_btn.enable_interrupt().unwrap();
        start_btn.enable_interrupt().unwrap();

        buttons[0] = cro_btn.is_low();
        buttons[1] = cir_btn.is_low();
        buttons[2] = srq_btn.is_low();
        buttons[3] = tri_btn.is_low();
        buttons[4] = l1_btn.is_low();
        buttons[5] = pick_up_btn.is_low();
        buttons[6] = pick_up_btn.is_low();
        buttons[7] = start_btn.is_low();

        generate_report(&buttons, &mut keys_report);

        input_keyboard.lock().set_from(&keys_report).notify();

        // Block until any waker.notify() call
        notification.wait_any();

        // It check value and send again to partially fix a bug
        // Some times the "release button" is't sent. So it is for that
        buttons[0] = cro_btn.is_low();
        buttons[1] = cir_btn.is_low();
        buttons[2] = srq_btn.is_low();
        buttons[3] = tri_btn.is_low();
        buttons[4] = l1_btn.is_low();
        buttons[5] = pick_up_btn.is_low();
        buttons[6] = pick_up_btn.is_low();
        buttons[7] = start_btn.is_low();

        generate_report(&buttons, &mut keys_report);

        input_keyboard.lock().set_from(&keys_report).notify();
    }
}
