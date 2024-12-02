use esp32_nimble::hid::hid;

pub const REPORT_ID: u8 = 0x01;

pub const REPORT_MAP: &[u8] = hid!(
    (0x05, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (0x09, 0x04), // USAGE (Gamepad)
    (0xa1, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Gamepad
    // Buttons
    (0x05, 0x09), //     USAGE_PAGE (Button)
    (0x19, 0x01), //     USAGE_MINIMUM (First button 1)
    (0x29, 0x08), //     USAGE_MAXIMUM (Last button 8)
    (0x15, 0x00), //     LOGICAL_MINIMUM (Released 0)
    (0x25, 0x01), //     LOGICAL_MAXIMUM (Pressed 1)
    (0x75, 0x01), //     REPORT_SIZE (1 byte)
    (0x95, 0x08), //     REPORT_COUNT (8 buttons per byte)
    (0x81, 0x02), //     INPUT (Data,Var,Abs)
    //Whammy
    (0x05, 0x01), //     USAGE_PAGE (Generic Desktop Controls)
    (0x09, 0x30), //     Usage (X) - Isso seria o valor do potenciômetro
    (0x09, 0x31), //     USAGE (Y)
    (0x15, 0x00), //     Logical Minimum (0)
    (0x26, 0x3E, 0x0C), //     Logical Maximum (255)
    (0x75, 0x10), //     Report Size (8 bits)
    (0x95, 0x01), //     Report Count (1 byte)
    (0x81, 0x02), //     Input (Data, Variable, Absolute)
    (0xc0),       // END_COLLECTION
);
