use clipboard::{ClipboardContext, ClipboardProvider};

use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::thread;
use std::time::Duration;

pub fn send_text_to_context(text: &str, enigo: &mut Enigo) {
    // Wait briefly to ensure the target window is focused
    let _ = enigo.key(Key::Unicode('\n'), Click);
    thread::sleep(Duration::from_millis(100));
    let _ = enigo.text(&("\r".to_string() + text));
}

pub fn get_selected_text(enigo: &mut Enigo) -> Option<String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().ok()?;

    // Backup current clipboard
    let old = ctx.get_contents().ok();
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Simulate select all
    let _ = enigo.key(Key::Meta, Press);
    let _ = enigo.key(Key::Unicode('a'), Click);
    let _ = enigo.key(Key::Meta, Release);
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Simulate copy command
    let _ = enigo.key(Key::Meta, Press);
    let _ = enigo.key(Key::Unicode('c'), Click);
    let _ = enigo.key(Key::Meta, Release);

    std::thread::sleep(std::time::Duration::from_millis(100));

    let selected = ctx.get_contents().ok();

    // Restore clipboard
    if let Some(old) = old {
        let _ = ctx.set_contents(old);
    }

    selected
}
