use devices::{HatSwitch, JoystickState};
pub use num_traits::{FromPrimitive, ToPrimitive};

/// State of a Key or Button
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum State {
    Pressed,
    Released,
}

/// Windows Virtual Key Codes
#[derive(Eq, PartialEq, Hash, Clone, Debug, Primitive)]
pub enum KeyId {
    LeftButton = 0x01,
    RightButton = 0x02,
    Cancel = 0x03,
    MiddleButton = 0x04,
    ExtraButton1 = 0x05,
    ExtraButton2 = 0x06,
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Menu = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    Kana = 0x15,
    Junja = 0x17,
    Final = 0x18,
    Kanji = 0x19,
    Escape = 0x1B,
    Convert = 0x1C,
    NonConvert = 0x1D,
    Accept = 0x1E,
    ModeChange = 0x1F,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    N0 = 0x30,
    N1 = 0x31,
    N2 = 0x32,
    N3 = 0x33,
    N4 = 0x34,
    N5 = 0x35,
    N6 = 0x36,
    N7 = 0x37,
    N8 = 0x38,
    N9 = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    LeftWindows = 0x5B,
    RightWindows = 0x5C,
    Application = 0x5D,
    Sleep = 0x5F,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    NumpadMultiply = 0x6A,
    NumpadAdd = 0x6B,
    NumpadSeparator = 0x6C,
    NumpadSubtract = 0x6D,
    NumpadDecimal = 0x6E,
    NumpadDivide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    NumLock = 0x90,
    ScrollLock = 0x91,
    FujitsuJisho = 0x92,
    FujitsuMasshou = 0x93,
    FujitsuTouroku = 0x94,
    FujitsuLoya = 0x95,
    FujitsuRoya = 0x96,
    NumpadEnter = 0x97,
    LeftShift = 0xA0,
    RightShift = 0xA1,
    LeftControl = 0xA2,
    RightControl = 0xA3,
    LeftMenu = 0xA4,
    RightMenu = 0xA5,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,
    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,
    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApplication1 = 0xB6,
    LaunchApplication2 = 0xB7,
    OEM1 = 0xBA,
    OEMPlus = 0xBB,
    OEMComma = 0xBC,
    OEMMinus = 0xBD,
    OEMPeriod = 0xBE,
    OEM2 = 0xBF,
    OEM3 = 0xC0,
    OEM4 = 0xDB,
    OEM5 = 0xDC,
    OEM6 = 0xDD,
    OEM7 = 0xDE,
    OEM8 = 0xDF,
    OEMAX = 0xE1,
    OEM102 = 0xE2,
    ICOHelp = 0xE3,
    ICO00 = 0xE4,
    ProcessKey = 0xE5,
    ICOClear = 0xE6,
    Packet = 0xE7,
    OEMReset = 0xE9,
    OEMJump = 0xEA,
    OEMPA1 = 0xEB,
    OEMPA2 = 0xEC,
    OEMPA3 = 0xED,
    OEMWSCtrl = 0xEE,
    OEMCUSel = 0xEF,
    OEMATTN = 0xF0,
    OEMFinish = 0xF1,
    OEMCopy = 0xF2,
    OEMAuto = 0xF3,
    OEMENLW = 0xF4,
    OEMBackTab = 0xF5,
    ATTN = 0xF6,
    CRSel = 0xF7,
    EXSel = 0xF8,
    EREOF = 0xF9,
    Play = 0xFA,
    Zoom = 0xFB,
    Noname = 0xFC,
    PA1 = 0xFD,
    OEMClear = 0xFE,
}

/// Mouse Buttons
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    SLIDER,
}

/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid).
/// Keyboard press events repeat when a key is held down.
#[derive(Clone, Debug)]
pub enum RawEvent {
    MouseButtonEvent(usize, MouseButton, State),
    MouseMoveEvent(usize, i32, i32),
    MouseWheelEvent(usize, f32),
    KeyboardEvent(usize, KeyId, State),
    JoystickButtonEvent(usize, usize, State),
    JoystickAxisEvent(usize, Axis, f64),
    JoystickHatSwitchEvent(usize, HatSwitch),
}

impl JoystickState {
    pub fn compare_states(&self, other_state: JoystickState, id: usize) -> Vec<RawEvent> {
        let mut output: Vec<RawEvent> = Vec::new();
        for (index, (&press_state, _)) in self
            .button_states
            .iter()
            .zip(other_state.button_states.iter())
            .enumerate()
            .filter(|&(_, (&a, &b))| a != b)
        {
            output.push(RawEvent::JoystickButtonEvent(
                id,
                index,
                if press_state {
                    State::Released
                } else {
                    State::Pressed
                },
            ));
        }
        if self.raw_axis_states.x != other_state.raw_axis_states.x {
            if let Some(value) = other_state.axis_states.x {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::X, value));
            }
        }
        if self.raw_axis_states.y != other_state.raw_axis_states.y {
            if let Some(value) = other_state.axis_states.y {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Y, value));
            }
        }
        if self.raw_axis_states.z != other_state.raw_axis_states.z {
            if let Some(value) = other_state.axis_states.z {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Z, value));
            }
        }
        if self.raw_axis_states.rx != other_state.raw_axis_states.rx {
            if let Some(value) = other_state.axis_states.rx {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RX, value));
            }
        }
        if self.raw_axis_states.ry != other_state.raw_axis_states.ry {
            if let Some(value) = other_state.axis_states.ry {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RY, value));
            }
        }
        if self.raw_axis_states.rz != other_state.raw_axis_states.rz {
            if let Some(value) = other_state.axis_states.rz {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RZ, value));
            }
        }
        if self.raw_axis_states.slider != other_state.raw_axis_states.slider {
            if let Some(value) = other_state.axis_states.slider {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::SLIDER, value));
            }
        }
        if let Some(value_other) = other_state.hatswitch {
            if let Some(value_self) = self.hatswitch.clone() {
                if value_self != value_other {
                    output.push(RawEvent::JoystickHatSwitchEvent(id, value_other));
                }
            }
        }
        output
    }
}
