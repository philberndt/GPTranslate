use anyhow::Result;
use tauri::{
    AppHandle, Manager,
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

use crate::theme::{SystemTheme, get_system_theme};

fn get_optimal_icon_size() -> u32 {
    // On Windows, try to detect DPI scaling
    #[cfg(target_os = "windows")]
    {
        use std::ptr;
        use winapi::um::wingdi::{GetDeviceCaps, LOGPIXELSX};
        use winapi::um::winuser::{GetDC, ReleaseDC};

        unsafe {
            let hdc = GetDC(ptr::null_mut());
            if !hdc.is_null() {
                let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
                ReleaseDC(ptr::null_mut(), hdc);

                // Calculate scale factor (96 DPI is 100% scaling)
                let scale_factor = dpi as f32 / 96.0;
                log::info!("Detected DPI: {}, Scale factor: {:.1}", dpi, scale_factor);

                // Choose icon size based on scale factor
                match scale_factor {
                    x if x >= 2.0 => 32,  // 200% scaling or higher
                    x if x >= 1.5 => 24,  // 150% scaling
                    x if x >= 1.25 => 20, // 125% scaling
                    _ => 16,              // 100% scaling
                }
            } else {
                log::warn!("Failed to get device context, using default icon size");
                24 // Default to 24px for good quality
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        24 // Default size for non-Windows platforms
    }
}

fn get_theme_icon() -> Result<Image<'static>> {
    // Detect system theme and use appropriate icon
    let theme = get_system_theme().unwrap_or(SystemTheme::Light);
    let icon_size = get_optimal_icon_size();
    log::info!("Loading tray icon: theme={:?}, size={}px", theme, icon_size);

    // Use PNG files for better quality in system tray
    // Logic: Use icon variant that matches the theme
    let icon_bytes: &[u8] = match (theme, icon_size) {
        (SystemTheme::Dark, 16) => include_bytes!("../icons/tray_dark_16.png"),
        (SystemTheme::Dark, 20) => include_bytes!("../icons/tray_dark_20.png"),
        (SystemTheme::Dark, 24) => include_bytes!("../icons/tray_dark_24.png"),
        (SystemTheme::Dark, _) => include_bytes!("../icons/tray_dark_32.png"),
        (SystemTheme::Light, 16) => include_bytes!("../icons/tray_light_16.png"),
        (SystemTheme::Light, 20) => include_bytes!("../icons/tray_light_20.png"),
        (SystemTheme::Light, 24) => include_bytes!("../icons/tray_light_24.png"),
        (SystemTheme::Light, _) => include_bytes!("../icons/tray_light_32.png"),
    };

    // For Tauri v2, we need to use a different approach for loading images
    // Try using the image bytes directly as a Vec<u8>
    let icon_bytes = icon_bytes.to_vec();

    // Tauri v2 typically expects raw RGBA data, but since these are PNG files,
    // we might need to decode them first. For now, let's try creating an image
    // directly from the PNG bytes by using the image crate
    let img = image::load_from_memory(icon_bytes.as_slice())
        .map_err(|e| anyhow::anyhow!("Failed to decode icon image: {}", e))?
        .to_rgba8();
    let (width, height) = img.dimensions();
    Ok(Image::new_owned(img.into_raw(), width, height))
}

pub fn create_tray(app: &AppHandle) -> Result<()> {
    log::info!("Starting tray creation...");

    // Create tray menu
    let open_item = MenuItem::with_id(app, "open", "Open GPTranslate", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

    log::info!("Tray menu created successfully");

    // Get icon for tray
    let icon = match get_theme_icon() {
        Ok(icon) => {
            log::info!("Successfully loaded theme-appropriate tray icon");
            icon
        }
        Err(e) => {
            log::warn!("Failed to load tray icon: {}", e);
            // Fallback to default window icon
            app.default_window_icon()
                .ok_or_else(|| anyhow::anyhow!("No default window icon available"))?
                .clone()
        }
    };

    // Create tray icon
    let _tray = TrayIconBuilder::with_id("main-tray")
        .tooltip("GPTranslate")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let Some(app) = tray.app_handle().get_webview_window("main") {
                match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let _ = app.show();
                        let _ = app.set_focus();
                    }
                    TrayIconEvent::DoubleClick {
                        button: MouseButton::Left,
                        ..
                    } => {
                        let _ = app.show();
                        let _ = app.set_focus();
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    log::info!("✅ Tray icon created successfully!");
    Ok(())
}
