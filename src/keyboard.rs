use event::{FromPrimitive, KeyId, RawEvent, State};
use winapi::um::winuser::*;

// #[derive(Clone, Debug, PartialEq, Eq)]
// enum KeyPos {
//     Left,
//     Right,
// }

pub fn process_keyboard_data(raw_data: &RAWKEYBOARD, id: usize) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    let flags = raw_data.Flags as u32;
    let key = raw_data.VKey;
    // let mut key_opt: Option<KeyId> = None;
    let key_state: State;
    // let key_pos: KeyPos;
    if flags & RI_KEY_BREAK != 0 {
        key_state = State::Released;
    } else {
        key_state = State::Pressed;
    }
    let is_e0 = flags & RI_KEY_E0 != 0;
    let is_e1 = flags & RI_KEY_E1 != 0;
    // if !is_e0 {
    //     key_pos = KeyPos::Left;
    // } else {
    //     key_pos = KeyPos::Right;
    // }

    let key_opt = match (key as i32, is_e0, is_e1) {
        (VK_SHIFT, _, _) => {
            KeyId::from_u32(unsafe { MapVirtualKeyA(raw_data.MakeCode.into(), 0x03) })
        } //MapvkVscToVkEx
        (VK_CONTROL, e0, _) => Some(if e0 {
            KeyId::RightControl
        } else {
            KeyId::LeftControl
        }),
        (VK_MENU, e0, _) => Some(if e0 {
            KeyId::RightMenu
        } else {
            KeyId::LeftMenu
        }),
        (VK_RETURN, true, _) => Some(KeyId::NumpadEnter),
        (VK_DELETE, false, _) => Some(KeyId::NumpadDecimal),
        (VK_INSERT, false, _) => Some(KeyId::Numpad0),
        (VK_HOME, false, _) => Some(KeyId::Numpad7),
        (VK_END, false, _) => Some(KeyId::Numpad1),
        (VK_PRIOR, false, _) => Some(KeyId::Numpad9),
        (VK_NEXT, false, _) => Some(KeyId::Numpad3),
        (VK_LEFT, false, _) => Some(KeyId::Numpad4),
        (VK_RIGHT, false, _) => Some(KeyId::Numpad6),
        (VK_UP, false, _) => Some(KeyId::Numpad8),
        (VK_DOWN, false, _) => Some(KeyId::Numpad2),
        (VK_CLEAR, false, _) => Some(KeyId::Numpad5),
        (code, _, _) => KeyId::from_i32(code),
    };

    if let Some(key_id) = key_opt {
        output.push(RawEvent::KeyboardEvent(id, key_id, key_state));
    }
    output
}
