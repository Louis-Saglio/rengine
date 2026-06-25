use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;

pub fn open_input_event_devices(device_type: &str) -> Vec<File> {
    fs::read_dir("/dev/input/by-id")
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .filter(|entry| {
            entry
                .file_name()
                .as_bytes()
                .ends_with([b"event-", device_type.as_bytes()].concat().as_slice())
        })
        .filter_map(|entry| {
            OpenOptions::new()
                .read(true)
                .custom_flags(0x800)
                .open(entry.path())
                .ok()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyCode {
    Left,
    Right,
    Up,
    Down,
    T,
    U,
    Q,
    R,
    MouseLeft,
    Unknown(u16),
}

impl From<u16> for KeyCode {
    fn from(code: u16) -> Self {
        match code {
            105 => KeyCode::Left,
            106 => KeyCode::Right,
            103 => KeyCode::Up,
            108 => KeyCode::Down,
            20 => KeyCode::T,
            22 => KeyCode::U,
            16 => KeyCode::Q,
            19 => KeyCode::R,
            272 => KeyCode::MouseLeft,
            _ => KeyCode::Unknown(code),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyState {
    Released,
    Pressed,
    Autorepeat,
    Unknown(i32),
}

impl From<i32> for KeyState {
    fn from(value: i32) -> Self {
        match value {
            0 => KeyState::Released,
            1 => KeyState::Pressed,
            2 => KeyState::Autorepeat,
            _ => KeyState::Unknown(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelativeAxis {
    X,
    Y,
    Wheel,
    Unknown(u16),
}

impl From<u16> for RelativeAxis {
    fn from(code: u16) -> Self {
        match code {
            0 => RelativeAxis::X,
            1 => RelativeAxis::Y,
            8 => RelativeAxis::Wheel,
            _ => RelativeAxis::Unknown(code),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyPressed { key: KeyCode, state: KeyState },
    Relative { axis: RelativeAxis, value: i32 },
    Unknown { type_: u16, code: u16, value: i32 },
}

impl InputEvent {
    pub fn from_bytes(chunk: &[u8; 24]) -> Self {
        let type_ = u16::from_ne_bytes(chunk[16..18].try_into().unwrap());
        let code = u16::from_ne_bytes(chunk[18..20].try_into().unwrap());
        let value = i32::from_ne_bytes(chunk[20..24].try_into().unwrap());
        match type_ {
            1 => InputEvent::KeyPressed {
                key: KeyCode::from(code),
                state: KeyState::from(value),
            },
            2 => InputEvent::Relative {
                axis: RelativeAxis::from(code),
                value,
            },
            _ => InputEvent::Unknown {
                type_,
                code,
                value,
            },
        }
    }
}

pub fn read_input_events(files: &mut [File]) -> Vec<InputEvent> {
    let mut events = vec![];
    for file in files {
        let mut buffer = [0u8; 24];
        loop {
            match file.read(&mut buffer) {
                Ok(24) => events.push(InputEvent::from_bytes(&buffer)),
                Ok(_) => {}
                Err(err) => {
                    if err.kind() == ErrorKind::WouldBlock {
                        break;
                    }
                }
            }
        }
    }
    events
}
