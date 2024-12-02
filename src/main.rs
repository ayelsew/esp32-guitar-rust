use std::{
    ffi::{c_void, CString},
    num::NonZero,
    ptr::{self},
    sync::{Arc, Mutex},
};

use esp32_nimble::{
    enums::{AuthReq, SecurityIOCap},
    BLEAdvertisementData, BLEDevice, BLEHIDDevice,
};

use esp_idf_hal::{
    adc::{
        attenuation,
        oneshot::{config::AdcChannelConfig, AdcChannelDriver, AdcDriver},
        Resolution
    },
    delay::FreeRtos,
    gpio::{InterruptType::*, *},
    peripherals::Peripherals,
    task::{self, notification::Notification},
};
use esp_idf_sys::{self as _};

mod report_descriptor;

struct Button<'a, SU, SD, G, R, Y, B, O, S>
where
    SU: Pin + esp_idf_hal::gpio::InputPin,
    SD: Pin + esp_idf_hal::gpio::InputPin,
    G: Pin + esp_idf_hal::gpio::InputPin,
    R: Pin + esp_idf_hal::gpio::InputPin,
    Y: Pin + esp_idf_hal::gpio::InputPin,
    B: Pin + esp_idf_hal::gpio::InputPin,
    O: Pin + esp_idf_hal::gpio::InputPin,
    S: Pin + esp_idf_hal::gpio::InputPin,
{
    strum_up: PinDriver<'a, SU, Input>,
    strum_down: PinDriver<'a, SD, Input>,
    green: PinDriver<'a, G, Input>,
    red: PinDriver<'a, R, Input>,
    yellow: PinDriver<'a, Y, Input>,
    blue: PinDriver<'a, B, Input>,
    orange: PinDriver<'a, O, Input>,
    start: PinDriver<'a, S, Input>,
}

static mut REPORT: u32 = 0x000000;

fn update_buttons_report<T>(pin: &mut PinDriver<T, Input>, index: u8)
where
    T: Pin + esp_idf_hal::gpio::InputPin,
{
    unsafe {
        if pin.is_low() {
            *ptr::addr_of_mut!(REPORT) |= 1 << index;
        } else {
            *ptr::addr_of_mut!(REPORT) &= !(1 << index);
        }
    }
}

extern "C" fn bluetooth_task(_: *mut c_void) {
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

    let mut cache: u32 = 0x000000;

    loop {
        unsafe {
            if cache != REPORT {
                //println!("Número em binário: {:016b}", REPORT);
                input_keyboard
                    .lock()
                    .set_from(&*ptr::addr_of_mut!(REPORT))
                    .notify();
                cache = *ptr::addr_of_mut!(REPORT);
            } else {
                FreeRtos::delay_ms(1);
            }
        }
    }
}

extern "C" fn buttons_task(arg: *mut c_void) {
    let buttons_shared = unsafe {
        &*(arg as *mut Arc<
            Mutex<Button<'static, Gpio25, Gpio33, Gpio26, Gpio27, Gpio14, Gpio12, Gpio13, Gpio4>>,
        >)
    };
    let mut button = buttons_shared.lock().unwrap();
    // MAP KEY

    button.strum_up.set_pull(Pull::Up).unwrap();
    button.strum_down.set_pull(Pull::Up).unwrap();
    button.green.set_pull(Pull::Up).unwrap();
    button.red.set_pull(Pull::Up).unwrap();
    button.yellow.set_pull(Pull::Up).unwrap();
    button.blue.set_pull(Pull::Up).unwrap();
    button.orange.set_pull(Pull::Up).unwrap();
    button.start.set_pull(Pull::Up).unwrap();

    // SETUP BUTTON INTERRUPT
    button.strum_up.set_interrupt_type(AnyEdge).unwrap();
    button.strum_down.set_interrupt_type(AnyEdge).unwrap();
    button.green.set_interrupt_type(AnyEdge).unwrap();
    button.red.set_interrupt_type(AnyEdge).unwrap();
    button.yellow.set_interrupt_type(AnyEdge).unwrap();
    button.blue.set_interrupt_type(AnyEdge).unwrap();
    button.orange.set_interrupt_type(AnyEdge).unwrap();
    button.start.set_interrupt_type(AnyEdge).unwrap();

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
            button
                .strum_up
                .subscribe_nonstatic(move || {
                    waker_1.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .strum_down
                .subscribe_nonstatic(move || {
                    waker_2.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .green
                .subscribe_nonstatic(move || {
                    waker_3.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .red
                .subscribe_nonstatic(move || {
                    waker_4.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .yellow
                .subscribe_nonstatic(move || {
                    waker_5.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .blue
                .subscribe_nonstatic(move || {
                    waker_6.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .orange
                .subscribe_nonstatic(move || {
                    waker_7.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
            button
                .start
                .subscribe_nonstatic(move || {
                    waker_8.notify(NonZero::new(1).unwrap());
                })
                .unwrap();
        }

        button.strum_up.enable_interrupt().unwrap();
        button.strum_down.enable_interrupt().unwrap();
        button.green.enable_interrupt().unwrap();
        button.red.enable_interrupt().unwrap();
        button.yellow.enable_interrupt().unwrap();
        button.blue.enable_interrupt().unwrap();
        button.orange.enable_interrupt().unwrap();
        button.start.enable_interrupt().unwrap();

        update_buttons_report(&mut button.green, 0);
        update_buttons_report(&mut button.red, 1);
        update_buttons_report(&mut button.yellow, 2);
        update_buttons_report(&mut button.blue, 3);
        update_buttons_report(&mut button.orange, 4);
        update_buttons_report(&mut button.strum_up, 5);
        update_buttons_report(&mut button.strum_down, 6);
        update_buttons_report(&mut button.start, 7);

        // Block until any waker.notify() call
        notification.wait_any();

        // It check value and send again to partially fix a bug
        // Some times the "release button" is't sent. So it is for that

        update_buttons_report(&mut button.green, 0);
        update_buttons_report(&mut button.red, 1);
        update_buttons_report(&mut button.yellow, 2);
        update_buttons_report(&mut button.blue, 3);
        update_buttons_report(&mut button.orange, 4);
        update_buttons_report(&mut button.strum_up, 5);
        update_buttons_report(&mut button.strum_down, 6);
        update_buttons_report(&mut button.start, 7);
    }
}

fn main() {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();

    let button = Arc::new(Mutex::new(Button {
        strum_up: PinDriver::input(peripherals.pins.gpio25).unwrap(),
        strum_down: PinDriver::input(peripherals.pins.gpio33).unwrap(),
        green: PinDriver::input(peripherals.pins.gpio26).unwrap(),
        red: PinDriver::input(peripherals.pins.gpio27).unwrap(),
        yellow: PinDriver::input(peripherals.pins.gpio14).unwrap(),
        blue: PinDriver::input(peripherals.pins.gpio12).unwrap(),
        orange: PinDriver::input(peripherals.pins.gpio13).unwrap(),
        start: PinDriver::input(peripherals.pins.gpio4).unwrap(),
    }));

    unsafe {
        task::create(
            bluetooth_task,
            CString::new("bluetooth_task").unwrap().as_c_str(),
            8192,
            ptr::null_mut(),
            10,
            Some(esp_idf_hal::cpu::Core::Core0),
        )
        .unwrap();

        task::create(
            buttons_task,
            CString::new("buttons_task").unwrap().as_c_str(),
            8192,
            Box::into_raw(Box::new(button)) as *mut c_void,
            10,
            Some(esp_idf_hal::cpu::Core::Core1),
        )
        .unwrap();

        /* task::create(
            whammy_task,
            CString::new("whammy_task").unwrap().as_c_str(),
            8192,
            ptr::null_mut(),
            10,
            Some(esp_idf_hal::cpu::Core::Core0),
        )
        .unwrap(); */
    }

    let adc = AdcDriver::new(peripherals.adc2).unwrap();
    let mut channel = AdcChannelDriver::new(
        &adc,
        peripherals.pins.gpio2,
        &AdcChannelConfig {
            resolution: Resolution::Resolution11Bit,
            attenuation: attenuation::DB_11,
            calibration: true,
        },
    )
    .unwrap();

    loop {

        let raw_value = adc.read(&mut channel).unwrap();

        unsafe {
            *ptr::addr_of_mut!(REPORT) = (*ptr::addr_of_mut!(REPORT) & 0x0000ff) | ((raw_value as u32) << 8);
        }
        
        FreeRtos::delay_ms(1);
    }
}
