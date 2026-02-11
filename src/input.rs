use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::mem::transmute;
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RawInputEvent {
    pub time: [u64; 2],
    pub type_: u16,
    pub code: u16,
    pub value: i32,
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
    Keyboard { key: KeyCode, state: KeyState },
    Relative { axis: RelativeAxis, value: i32 },
    Unknown { type_: u16, code: u16, value: i32 },
}

impl From<RawInputEvent> for InputEvent {
    fn from(raw: RawInputEvent) -> Self {
        match raw.type_ {
            1 => InputEvent::Keyboard {
                key: KeyCode::from(raw.code),
                state: KeyState::from(raw.value),
            },
            2 => InputEvent::Relative {
                axis: RelativeAxis::from(raw.code),
                value: raw.value,
            },
            _ => InputEvent::Unknown {
                type_: raw.type_,
                code: raw.code,
                value: raw.value,
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
                Ok(24) => events.extend(buffer.chunks_exact(24).map(|chunk| {
                    let chunk: [u8; 24] = chunk.try_into().unwrap();
                    let raw: RawInputEvent = unsafe { transmute(chunk) };
                    InputEvent::from(raw)
                })),
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
