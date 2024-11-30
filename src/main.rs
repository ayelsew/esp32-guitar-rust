#![allow(dead_code)]
use std::num::NonZero;

use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    BLEAdvertisementData, BLEDevice, BLEHIDDevice,
};

use esp_idf_hal::{gpio::*, peripherals::Peripherals, task::notification::Notification};
use esp_idf_sys::{self as _};

mod report_descriptor;

const STRUM_UP: u8 = 0x51;
const STRUM_DOWN: u8 = 0x52;
const Q: u8 = 0x14;
const W: u8 = 0x1a;
const E: u8 = 0x08;
const R: u8 = 0x15;
const T: u8 = 0x17;
const H: u8 = 0x0b;
const NULL: u8 = 0x00;

#[repr(packed)]
struct GamepadReport {
    // Gamepad buttons (16 buttons)
    pub buttons: u8, // 16 buttons, represented by a bitmask
                     // Gamepad axes (2 axes: X, Y)
                     // pub x_axis: u8, // X axis value (0-127)
                     // pub y_axis: u8, // Y axis value (0-127)
}

fn main() {
    let _ = BLEDevice::set_device_name("GuittarPad").unwrap();

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

    let input_gamepad = hid.input_report(report_descriptor::GAMEPAD_ID);
    let _output_gamepad = hid.output_report(report_descriptor::GAMEPAD_ID);

    hid.manufacturer("Leydev");
    hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
    hid.hid_info(0x00, 0x01);

    hid.report_map(report_descriptor::GAMEPAD_REPORT_DESCRIPTOR);

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

    let mut gamepad_report = GamepadReport {
        buttons: 0, // Start with all buttons released (0)
                    // x_axis: 0,  // Default X axis value (center position)
                    // y_axis: 0,  // Default Y axis value (center position)
    };

    // MAP BUTTONS
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

        // Update the gamepad report based on button presses
        gamepad_report.buttons = 0; // Reset button states

        if pick_up_btn.is_low() {
            gamepad_report.buttons |= 0x0001; // Set button 1 (Pick up)
        }
        if pick_down_btn.is_low() {
            gamepad_report.buttons |= 0x0002; // Set button 2 (Pick down)
        }
        if cro_btn.is_low() {
            gamepad_report.buttons |= 0x0004; // Set button 3 (Cro)
        }
        if cir_btn.is_low() {
            gamepad_report.buttons |= 0x0008; // Set button 4 (Cir)
        }
        if srq_btn.is_low() {
            gamepad_report.buttons |= 0x0010; // Set button 5 (Srq)
        }
        if tri_btn.is_low() {
            gamepad_report.buttons |= 0x0020; // Set button 6 (Tri)
        }
        if l1_btn.is_low() {
            gamepad_report.buttons |= 0x0040; // Set button 7 (L1)
        }
        if start_btn.is_low() {
            gamepad_report.buttons |= 0x0080; // Set button 8 (Start)
        }

        // Optionally, you can add joystick axis logic here:
        // gamepad_report.x_axis = 127; // Example: Centered X axis
        // gamepad_report.y_axis = 127; // Example: Centered Y axis

        // Send the gamepad report
        input_gamepad.lock().set_from(&gamepad_report).notify();

        print!("{}", gamepad_report.buttons);

        // Block until any waker.notify() call
        notification.wait_any();

        gamepad_report.buttons = 0; // Reset button states

        if pick_up_btn.is_low() {
            gamepad_report.buttons |= 0x0001; // Set button 1 (Pick up)
        }
        if pick_down_btn.is_low() {
            gamepad_report.buttons |= 0x0002; // Set button 2 (Pick down)
        }
        if cro_btn.is_low() {
            gamepad_report.buttons |= 0x0004; // Set button 3 (Cro)
        }
        if cir_btn.is_low() {
            gamepad_report.buttons |= 0x0008; // Set button 4 (Cir)
        }
        if srq_btn.is_low() {
            gamepad_report.buttons |= 0x0010; // Set button 5 (Srq)
        }
        if tri_btn.is_low() {
            gamepad_report.buttons |= 0x0020; // Set button 6 (Tri)
        }
        if l1_btn.is_low() {
            gamepad_report.buttons |= 0x0040; // Set button 7 (L1)
        }
        if start_btn.is_low() {
            gamepad_report.buttons |= 0x0080; // Set button 8 (Start)
        }

        // Optionally, you can check again and resend if needed.
        input_gamepad.lock().set_from(&gamepad_report).notify();
    }
}
