use std::cell::RefCell;
use tauri::AppHandle;
use tray_icon::{TrayIconBuilder, Icon};
use image::load_from_memory;


enum IconState {
    Idle,
    Active,
}

// Fix file names to match your actual icons
const ICON_IDLE: &[u8] = include_bytes!("../icons/icon-idle.png");
const ICON_ACTIVE: &[u8] = include_bytes!("../icons/icon-active.png");

fn load_icon(state: IconState) -> Icon {
    let data = match state {
        IconState::Idle => ICON_IDLE,
        IconState::Active => ICON_ACTIVE,
    };

    let img = load_from_memory(data)
        .expect("Failed to decode icon image")
        .to_rgba8();
    let (width, height) = img.dimensions();

    println!("Loaded icon size: {}x{}", width, height);  // Debug

    Icon::from_rgba(img.into_raw(), width, height).expect("Failed to create icon")
}

thread_local! {
    static TRAY_HANDLE: RefCell<Option<tray_icon::TrayIcon>> = RefCell::new(None);
}
pub fn create_tray(_app: AppHandle) {
    let icon = load_icon(IconState::Idle);

    let tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Juju is running")
        .build()
        .expect("Failed to build tray");

    TRAY_HANDLE.with(|handle| {
        *handle.borrow_mut() = Some(tray);
    });
}
