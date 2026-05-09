use std::env;
use std::fs::File;
use std::io::Write;
use std::mem::size_of;
use std::os::fd::AsRawFd;
use std::os::raw::{c_int, c_long, c_ulong};
use std::slice;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const EV_REL: u16 = 0x02;
const SYN_REPORT: u16 = 0;

const REL_X: u16 = 0x00;
const REL_Y: u16 = 0x01;
const REL_HWHEEL: u16 = 0x06;
const REL_WHEEL: u16 = 0x08;

const BTN_LEFT: u16 = 0x110;
const BTN_RIGHT: u16 = 0x111;
const BTN_MIDDLE: u16 = 0x112;

const BUS_USB: u16 = 0x03;

const KEY_LEFTCTRL: u16 = 29;
const KEY_LEFTSHIFT: u16 = 42;
const KEY_LEFTALT: u16 = 56;
const KEY_LEFTMETA: u16 = 125;

const UI_DEV_CREATE: c_ulong = 0x5501;
const UI_DEV_DESTROY: c_ulong = 0x5502;
const UI_DEV_SETUP: c_ulong = ioctl_iow(b'U', 3, size_of::<UInputSetup>());
const UI_SET_EVBIT: c_ulong = ioctl_iow(b'U', 100, size_of::<c_int>());
const UI_SET_KEYBIT: c_ulong = ioctl_iow(b'U', 101, size_of::<c_int>());
const UI_SET_RELBIT: c_ulong = ioctl_iow(b'U', 102, size_of::<c_int>());

unsafe extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

const fn ioctl_iow(type_char: u8, number: u8, size: usize) -> c_ulong {
    const IOC_NRBITS: u64 = 8;
    const IOC_TYPEBITS: u64 = 8;
    const IOC_SIZEBITS: u64 = 14;
    const IOC_NRSHIFT: u64 = 0;
    const IOC_TYPESHIFT: u64 = IOC_NRSHIFT + IOC_NRBITS;
    const IOC_SIZESHIFT: u64 = IOC_TYPESHIFT + IOC_TYPEBITS;
    const IOC_DIRSHIFT: u64 = IOC_SIZESHIFT + IOC_SIZEBITS;
    const IOC_WRITE: u64 = 1;

    ((IOC_WRITE << IOC_DIRSHIFT)
        | ((size as u64) << IOC_SIZESHIFT)
        | ((type_char as u64) << IOC_TYPESHIFT)
        | ((number as u64) << IOC_NRSHIFT)) as c_ulong
}

#[derive(Debug, Clone, Copy)]
pub struct InputTiming {
    pub device_ready: Duration,
    pub goto_pause: Duration,
    pub click_delay: Duration,
    pub tap: Duration,
    pub chord: Duration,
    pub type_gap: Duration,
}

impl InputTiming {
    pub fn runtime() -> Self {
        let device_ready = env::var("AGENT_LINUX_CONTROL_DEVICE_DELAY")
            .ok()
            .and_then(|value| value.parse::<f64>().ok())
            .map(Duration::from_secs_f64)
            .unwrap_or_else(|| Duration::from_millis(650));
        Self {
            device_ready,
            goto_pause: Duration::from_millis(50),
            click_delay: Duration::from_millis(50),
            tap: Duration::from_millis(30),
            chord: Duration::from_millis(20),
            type_gap: Duration::from_millis(15),
        }
    }

    #[cfg(test)]
    pub fn zero() -> Self {
        Self {
            device_ready: Duration::ZERO,
            goto_pause: Duration::ZERO,
            click_delay: Duration::ZERO,
            tap: Duration::ZERO,
            chord: Duration::ZERO,
            type_gap: Duration::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct InputStepResult {
    pub index: usize,
    pub action: &'static str,
    pub ok: bool,
}

pub trait InputDevice {
    fn move_rel(&mut self, dx: i32, dy: i32) -> Result<()>;
    fn scroll(&mut self, vertical: i32, horizontal: i32) -> Result<()>;
    fn key(&mut self, code: u16, down: bool) -> Result<()>;
}

pub fn execute_input_steps(
    input: &mut dyn InputDevice,
    steps: &[alc_core::InputStep],
    timing: InputTiming,
) -> Result<Vec<InputStepResult>> {
    let mut results = Vec::with_capacity(steps.len());
    for (index, step) in steps.iter().enumerate() {
        execute_input_step(input, step, timing)
            .with_context(|| format!("input step {index} ({})", step.action_name()))?;
        results.push(InputStepResult {
            index,
            action: step.action_name(),
            ok: true,
        });
    }
    Ok(results)
}

fn execute_input_step(
    input: &mut dyn InputDevice,
    step: &alc_core::InputStep,
    timing: InputTiming,
) -> Result<()> {
    match step {
        alc_core::InputStep::Move { dx, dy } => input.move_rel(*dx, *dy),
        alc_core::InputStep::Goto { x, y } => goto(input, *x, *y, timing),
        alc_core::InputStep::Click {
            button,
            x,
            y,
            delay_ms,
        } => {
            if let (Some(x), Some(y)) = (x, y) {
                goto(input, *x, *y, timing)?;
                sleep(
                    delay_ms
                        .map(Duration::from_millis)
                        .unwrap_or(timing.click_delay),
                );
            } else if x.is_some() || y.is_some() {
                bail!("x and y must be provided together");
            }
            tap(
                input,
                button_code(button.as_deref().unwrap_or("left"))?,
                timing,
            )
        }
        alc_core::InputStep::Scroll {
            vertical,
            horizontal,
        } => input.scroll(*vertical, horizontal.unwrap_or(0)),
        alc_core::InputStep::Key { name } => tap(input, key_code(name)?, timing),
        alc_core::InputStep::Hotkey { chord } => hotkey(input, chord, timing),
        alc_core::InputStep::Type { text } => type_text(input, text, timing),
    }
}

fn goto(input: &mut dyn InputDevice, x: i32, y: i32, timing: InputTiming) -> Result<()> {
    input.move_rel(-100000, -100000)?;
    sleep(timing.goto_pause);
    input.move_rel(x, y)
}

fn tap(input: &mut dyn InputDevice, code: u16, timing: InputTiming) -> Result<()> {
    input.key(code, true)?;
    sleep(timing.tap);
    input.key(code, false)
}

fn hotkey(input: &mut dyn InputDevice, chord: &str, timing: InputTiming) -> Result<()> {
    let keys = chord
        .split('+')
        .filter(|part| !part.is_empty())
        .map(key_code)
        .collect::<Result<Vec<_>>>()?;
    if keys.is_empty() {
        return Ok(());
    }
    for code in &keys[..keys.len() - 1] {
        input.key(*code, true)?;
        sleep(timing.chord);
    }
    tap(input, keys[keys.len() - 1], timing)?;
    for code in keys[..keys.len() - 1].iter().rev() {
        input.key(*code, false)?;
        sleep(timing.chord);
    }
    Ok(())
}

fn type_text(input: &mut dyn InputDevice, text: &str, timing: InputTiming) -> Result<()> {
    for ch in text.chars() {
        let (code, shift) = char_key(ch)?;
        if shift {
            input.key(KEY_LEFTSHIFT, true)?;
        }
        tap(input, code, timing)?;
        if shift {
            input.key(KEY_LEFTSHIFT, false)?;
        }
        sleep(timing.type_gap);
    }
    Ok(())
}

fn sleep(duration: Duration) {
    if !duration.is_zero() {
        thread::sleep(duration);
    }
}

pub struct UInputDevice {
    file: File,
    created: bool,
}

impl UInputDevice {
    pub fn create(timing: InputTiming) -> Result<Self> {
        let file = File::options()
            .write(true)
            .open("/dev/uinput")
            .context("open /dev/uinput")?;
        let mut device = Self {
            file,
            created: false,
        };
        device.setup()?;
        device.created = true;
        sleep(timing.device_ready);
        Ok(device)
    }

    fn setup(&mut self) -> Result<()> {
        self.ioctl_int(UI_SET_EVBIT, EV_KEY)?;
        self.ioctl_int(UI_SET_EVBIT, EV_REL)?;
        for code in [REL_X, REL_Y, REL_WHEEL, REL_HWHEEL] {
            self.ioctl_int(UI_SET_RELBIT, code)?;
        }
        for code in all_key_codes() {
            self.ioctl_int(UI_SET_KEYBIT, code)?;
        }

        let mut setup = UInputSetup::default();
        setup.id.bustype = BUS_USB;
        setup.id.vendor = 0x4c43;
        setup.id.product = 0x0001;
        setup.id.version = 0x0001;
        let name = b"agent-linux-control-rust";
        setup.name[..name.len()].copy_from_slice(name);

        self.ioctl_setup(UI_DEV_SETUP, &setup)?;
        self.ioctl_noarg(UI_DEV_CREATE)?;
        Ok(())
    }

    fn ioctl_noarg(&self, request: c_ulong) -> Result<()> {
        let rc = unsafe { ioctl(self.file.as_raw_fd(), request) };
        errno_result(rc)
    }

    fn ioctl_int(&self, request: c_ulong, value: u16) -> Result<()> {
        let rc = unsafe { ioctl(self.file.as_raw_fd(), request, value as c_int) };
        errno_result(rc)
    }

    fn ioctl_setup(&self, request: c_ulong, setup: &UInputSetup) -> Result<()> {
        let rc = unsafe { ioctl(self.file.as_raw_fd(), request, setup as *const UInputSetup) };
        errno_result(rc)
    }

    fn event(&mut self, ev_type: u16, code: u16, value: i32) -> Result<()> {
        let event = InputEvent {
            time_sec: 0,
            time_usec: 0,
            ev_type,
            code,
            value,
        };
        let bytes = unsafe {
            slice::from_raw_parts(
                (&event as *const InputEvent).cast::<u8>(),
                size_of::<InputEvent>(),
            )
        };
        self.file.write_all(bytes).context("write input event")
    }

    fn sync(&mut self) -> Result<()> {
        self.event(EV_SYN, SYN_REPORT, 0)
    }
}

impl InputDevice for UInputDevice {
    fn move_rel(&mut self, dx: i32, dy: i32) -> Result<()> {
        if dx != 0 {
            self.event(EV_REL, REL_X, dx)?;
        }
        if dy != 0 {
            self.event(EV_REL, REL_Y, dy)?;
        }
        self.sync()
    }

    fn scroll(&mut self, vertical: i32, horizontal: i32) -> Result<()> {
        if vertical != 0 {
            self.event(EV_REL, REL_WHEEL, vertical)?;
        }
        if horizontal != 0 {
            self.event(EV_REL, REL_HWHEEL, horizontal)?;
        }
        self.sync()
    }

    fn key(&mut self, code: u16, down: bool) -> Result<()> {
        self.event(EV_KEY, code, if down { 1 } else { 0 })?;
        self.sync()
    }
}

impl Drop for UInputDevice {
    fn drop(&mut self) {
        if self.created {
            sleep(Duration::from_millis(150));
            let _ = self.ioctl_noarg(UI_DEV_DESTROY);
            self.created = false;
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct InputId {
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[repr(C)]
struct UInputSetup {
    id: InputId,
    name: [u8; 80],
    ff_effects_max: u32,
}

impl Default for UInputSetup {
    fn default() -> Self {
        Self {
            id: InputId::default(),
            name: [0; 80],
            ff_effects_max: 0,
        }
    }
}

#[repr(C)]
struct InputEvent {
    time_sec: c_long,
    time_usec: c_long,
    ev_type: u16,
    code: u16,
    value: i32,
}

fn errno_result(rc: c_int) -> Result<()> {
    if rc < 0 {
        Err(std::io::Error::last_os_error()).context("uinput ioctl")
    } else {
        Ok(())
    }
}

fn all_key_codes() -> Vec<u16> {
    let mut codes = vec![
        BTN_LEFT,
        BTN_RIGHT,
        BTN_MIDDLE,
        KEY_LEFTCTRL,
        KEY_LEFTSHIFT,
        KEY_LEFTALT,
        KEY_LEFTMETA,
    ];
    for ch in "abcdefghijklmnopqrstuvwxyz0123456789-=[]\\;'`,./ \n\t".chars() {
        if let Ok((code, _shift)) = char_key(ch) {
            codes.push(code);
        }
    }
    codes.extend((59..=68).chain([87, 88]));
    codes.extend([
        1, 14, 15, 28, 57, 102, 103, 104, 105, 106, 107, 108, 109, 111,
    ]);
    codes.sort_unstable();
    codes.dedup();
    codes
}

fn button_code(button: &str) -> Result<u16> {
    match button.to_ascii_lowercase().as_str() {
        "left" => Ok(BTN_LEFT),
        "right" => Ok(BTN_RIGHT),
        "middle" => Ok(BTN_MIDDLE),
        other => bail!("unknown button: {other}"),
    }
}

fn key_code(name: &str) -> Result<u16> {
    let lower = name.to_ascii_lowercase();
    if lower.chars().count() == 1 {
        return char_key(lower.chars().next().unwrap()).map(|(code, _shift)| code);
    }
    match lower.as_str() {
        "enter" | "return" => Ok(28),
        "space" => Ok(57),
        "tab" => Ok(15),
        "backspace" => Ok(14),
        "esc" | "escape" => Ok(1),
        "delete" | "del" => Ok(111),
        "home" => Ok(102),
        "end" => Ok(107),
        "pageup" => Ok(104),
        "pagedown" => Ok(109),
        "up" => Ok(103),
        "down" => Ok(108),
        "left" => Ok(105),
        "right" => Ok(106),
        "ctrl" | "control" => Ok(KEY_LEFTCTRL),
        "shift" => Ok(KEY_LEFTSHIFT),
        "alt" => Ok(KEY_LEFTALT),
        "meta" | "super" | "cmd" | "win" => Ok(KEY_LEFTMETA),
        "f1" => Ok(59),
        "f2" => Ok(60),
        "f3" => Ok(61),
        "f4" => Ok(62),
        "f5" => Ok(63),
        "f6" => Ok(64),
        "f7" => Ok(65),
        "f8" => Ok(66),
        "f9" => Ok(67),
        "f10" => Ok(68),
        "f11" => Ok(87),
        "f12" => Ok(88),
        other => bail!("unknown key: {other}"),
    }
}

fn char_key(ch: char) -> Result<(u16, bool)> {
    if ch.is_ascii_uppercase() {
        return char_key(ch.to_ascii_lowercase()).map(|(code, _shift)| (code, true));
    }
    let direct = match ch {
        'a' => Some(30),
        'b' => Some(48),
        'c' => Some(46),
        'd' => Some(32),
        'e' => Some(18),
        'f' => Some(33),
        'g' => Some(34),
        'h' => Some(35),
        'i' => Some(23),
        'j' => Some(36),
        'k' => Some(37),
        'l' => Some(38),
        'm' => Some(50),
        'n' => Some(49),
        'o' => Some(24),
        'p' => Some(25),
        'q' => Some(16),
        'r' => Some(19),
        's' => Some(31),
        't' => Some(20),
        'u' => Some(22),
        'v' => Some(47),
        'w' => Some(17),
        'x' => Some(45),
        'y' => Some(21),
        'z' => Some(44),
        '1' => Some(2),
        '2' => Some(3),
        '3' => Some(4),
        '4' => Some(5),
        '5' => Some(6),
        '6' => Some(7),
        '7' => Some(8),
        '8' => Some(9),
        '9' => Some(10),
        '0' => Some(11),
        '-' => Some(12),
        '=' => Some(13),
        '[' => Some(26),
        ']' => Some(27),
        '\\' => Some(43),
        ';' => Some(39),
        '\'' => Some(40),
        '`' => Some(41),
        ',' => Some(51),
        '.' => Some(52),
        '/' => Some(53),
        ' ' => Some(57),
        '\n' => Some(28),
        '\t' => Some(15),
        _ => None,
    };
    if let Some(code) = direct {
        return Ok((code, false));
    }
    let shifted = match ch {
        '!' => Some('1'),
        '@' => Some('2'),
        '#' => Some('3'),
        '$' => Some('4'),
        '%' => Some('5'),
        '^' => Some('6'),
        '&' => Some('7'),
        '*' => Some('8'),
        '(' => Some('9'),
        ')' => Some('0'),
        '_' => Some('-'),
        '+' => Some('='),
        '{' => Some('['),
        '}' => Some(']'),
        '|' => Some('\\'),
        ':' => Some(';'),
        '"' => Some('\''),
        '~' => Some('`'),
        '<' => Some(','),
        '>' => Some('.'),
        '?' => Some('/'),
        _ => None,
    };
    if let Some(base) = shifted {
        return char_key(base).map(|(code, _shift)| (code, true));
    }
    Err(anyhow!("unsupported character for typing: {ch:?}"))
}
