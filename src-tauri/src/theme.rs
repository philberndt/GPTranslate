use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTheme {
    Light,
    Dark,
}

impl SystemTheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemTheme::Light => "light",
            SystemTheme::Dark => "dark",
        }
    }
}

#[cfg(target_os = "windows")]
pub fn get_system_theme() -> Result<SystemTheme> {
    use std::mem;
    use std::ptr;
    use winapi::shared::minwindef::{DWORD, HKEY};
    use winapi::um::winnt::{KEY_READ, REG_DWORD};
    use winapi::um::winreg::{HKEY_CURRENT_USER, RegOpenKeyExW, RegQueryValueExW};

    log::info!("Attempting to detect Windows system theme...");

    unsafe {
        let mut key: HKEY = ptr::null_mut();
        let subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let result = RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut key);

        if result != 0 {
            log::warn!(
                "Failed to open registry key for theme detection (error code: {}), defaulting to light theme",
                result
            );
            return Ok(SystemTheme::Light);
        }

        let value_name = "SystemUsesLightTheme\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let mut value: DWORD = 0;
        let mut value_size: DWORD = mem::size_of::<DWORD>() as DWORD;
        let mut value_type: DWORD = 0;

        let result = RegQueryValueExW(
            key,
            value_name.as_ptr(),
            ptr::null_mut(),
            &mut value_type,
            &mut value as *mut DWORD as *mut u8,
            &mut value_size,
        );

        winapi::um::winreg::RegCloseKey(key);

        if result != 0 {
            log::warn!(
                "Failed to read theme registry value (error code: {}), defaulting to light theme",
                result
            );
            return Ok(SystemTheme::Light);
        }

        if value_type != REG_DWORD {
            log::warn!(
                "Registry value type is not DWORD (got type: {}), defaulting to light theme",
                value_type
            );
            return Ok(SystemTheme::Light);
        }

        // Registry value: 1 = Light theme, 0 = Dark theme
        let detected_theme = if value == 1 {
            SystemTheme::Light
        } else {
            SystemTheme::Dark
        };

        log::info!(
            "Successfully detected system theme: {:?} (registry value: {})",
            detected_theme,
            value
        );
        Ok(detected_theme)
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_system_theme() -> Result<SystemTheme> {
    // Default to light theme on non-Windows platforms
    // This can be extended for macOS/Linux theme detection later
    Ok(SystemTheme::Light)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_theme() {
        let theme = get_system_theme();
        assert!(theme.is_ok());
        println!("Current system theme: {:?}", theme.unwrap());
    }
}
