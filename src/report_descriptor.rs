use esp32_nimble::hid::*;

pub const GAMEPAD_ID: u8 = 0x0;
pub const KEYBOARD_ID: u8 = 0x01;
pub const MEDIA_KEYS_ID: u8 = 0x02;

pub const KEYBOARD_DESCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x06),      // USAGE (Keyboard)
    (COLLECTION, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Keyboard
    (REPORT_ID, GAMEPAD_ID), //   REPORT_ID (0)
    (USAGE_PAGE, 0x07),      //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0xE0),   //   USAGE_MINIMUM (0xE0)
    (USAGE_MAXIMUM, 0xE7),   //   USAGE_MAXIMUM (0xE7)
    (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01), //   Logical Maximum (1)
    (REPORT_SIZE, 0x01),     //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x08),    //   REPORT_COUNT (8)
    (HIDINPUT, 0x02), //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 1 byte (Reserved)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE (8)
    (HIDINPUT, 0x01), //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (REPORT_COUNT, 0x05), //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
    (REPORT_SIZE, 0x01),  //   REPORT_SIZE (1)
    (USAGE_PAGE, 0x08),   //   USAGE_PAGE (LEDs)
    (USAGE_MINIMUM, 0x01), //   USAGE_MINIMUM (0x01) ; Num Lock
    (USAGE_MAXIMUM, 0x05), //   USAGE_MAXIMUM (0x05) ; Kana
    (HIDOUTPUT, 0x02), //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 3 bits (Padding)
    (REPORT_SIZE, 0x03), //   REPORT_SIZE (3)
    (HIDOUTPUT, 0x01), //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    (REPORT_COUNT, 0x07), //   REPORT_COUNT (6) ; 6 bytes (Keys)
    (REPORT_SIZE, 0x08), //   REPORT_SIZE(8)
    (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM(0)
    (LOGICAL_MAXIMUM, 0x65), //   LOGICAL_MAXIMUM(0x65) ; 101 keys
    (USAGE_PAGE, 0x07), //   USAGE_PAGE (Kbrd/Keypad)
    (USAGE_MINIMUM, 0x00), //   USAGE_MINIMUM (0)
    (USAGE_MAXIMUM, 0x65), //   USAGE_MAXIMUM (0x65)
    (HIDINPUT, 0x00),  //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION),  // END_COLLECTION
    // ------------------------------------------------- Media Keys
    (USAGE_PAGE, 0x0C),         // USAGE_PAGE (Consumer)
    (USAGE, 0x01),              // USAGE (Consumer Control)
    (COLLECTION, 0x01),         // COLLECTION (Application)
    (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (3)
    (USAGE_PAGE, 0x0C),         //   USAGE_PAGE (Consumer)
    (LOGICAL_MINIMUM, 0x00),    //   LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01),    //   LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),        //   REPORT_SIZE (1)
    (REPORT_COUNT, 0x10),       //   REPORT_COUNT (16)
    (USAGE, 0xB5),              //   USAGE (Scan Next Track)     ; bit 0: 1
    (USAGE, 0xB6),              //   USAGE (Scan Previous Track) ; bit 1: 2
    (USAGE, 0xB7),              //   USAGE (Stop)                ; bit 2: 4
    (USAGE, 0xCD),              //   USAGE (Play/Pause)          ; bit 3: 8
    (USAGE, 0xE2),              //   USAGE (Mute)                ; bit 4: 16
    (USAGE, 0xE9),              //   USAGE (Volume Increment)    ; bit 5: 32
    (USAGE, 0xEA),              //   USAGE (Volume Decrement)    ; bit 6: 64
    (USAGE, 0x23, 0x02),        //   Usage (WWW Home)            ; bit 7: 128
    (USAGE, 0x94, 0x01),        //   Usage (My Computer) ; bit 0: 1
    (USAGE, 0x92, 0x01),        //   Usage (Calculator)  ; bit 1: 2
    (USAGE, 0x2A, 0x02),        //   Usage (WWW fav)     ; bit 2: 4
    (USAGE, 0x21, 0x02),        //   Usage (WWW search)  ; bit 3: 8
    (USAGE, 0x26, 0x02),        //   Usage (WWW stop)    ; bit 4: 16
    (USAGE, 0x24, 0x02),        //   Usage (WWW back)    ; bit 5: 32
    (USAGE, 0x83, 0x01),        //   Usage (Media sel)   ; bit 6: 64
    (USAGE, 0x8A, 0x01),        //   Usage (Mail)        ; bit 7: 128
    (HIDINPUT, 0x02), // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    (END_COLLECTION), // END_COLLECTION
);

pub const GAMEPAD_REPORT_DESCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x05),      // USAGE (Gamepad)
    (COLLECTION, 0x01), // COLLECTION (Application)
    // ------------------------------------------------- Gamepad
    (REPORT_ID, 0x01),       // REPORT_ID (1)
    (USAGE_PAGE, 0x09),      // USAGE_PAGE (Button)
    (USAGE_MINIMUM, 0x01),   // USAGE_MINIMUM (Button 1)
    (USAGE_MAXIMUM, 0x80),   // USAGE_MAXIMUM (Button 16)
    (LOGICAL_MINIMUM, 0x00), // LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x08), // LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),     // REPORT_SIZE (1)
    (REPORT_COUNT, 0x08),    // REPORT_COUNT (5 buttons)
    (HIDINPUT, 0x02),        // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // // ------------------------------------------------- Joystick Axes
    // (USAGE_PAGE, 0x01),      // USAGE_PAGE (Generic Desktop Ctrls)
    // (USAGE, 0x30),           // USAGE (X Axis)
    // (USAGE, 0x31),           // USAGE (Y Axis)
    // (USAGE, 0x32),           // USAGE (Z Axis)
    // (LOGICAL_MINIMUM, 0x80), // LOGICAL_MINIMUM (-128)
    // (LOGICAL_MAXIMUM, 0x7F), // LOGICAL_MAXIMUM (127)
    // (REPORT_SIZE, 0x08),     // REPORT_SIZE (8 bits)
    // (REPORT_COUNT, 0x03),    // REPORT_COUNT (3 axes: X, Y, Z)
    // (HIDINPUT, 0x02),        // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // // ------------------------------------------------- D-pad (Hat Switch)
    // (USAGE_PAGE, 0x01),      // USAGE_PAGE (Generic Desktop Ctrls)
    // (USAGE, 0x39),           // USAGE (Hat Switch)
    // (LOGICAL_MINIMUM, 0x00), // LOGICAL_MINIMUM (0)
    // (LOGICAL_MAXIMUM, 0x08), // LOGICAL_MAXIMUM (8)
    // (REPORT_SIZE, 0x04),     // REPORT_SIZE (4 bits)
    // (REPORT_COUNT, 0x01),    // REPORT_COUNT (1 hat switch)
    // (HIDINPUT, 0x02),        // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // ------------------------------------------------- End Collection
    (END_COLLECTION), // END_COLLECTION
);
