#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[cfg(windows)]
mod imp {
    use std::process::Command;
    use std::sync::Mutex;
    use super::CREATE_NO_WINDOW;
    use std::os::windows::process::CommandExt;

    const ES_CONTINUOUS: u32 = 0x8000_0000;
    const ES_SYSTEM_REQUIRED: u32 = 0x0000_0001;
    const ES_DISPLAY_REQUIRED: u32 = 0x0000_0002;
    const ES_AWAYMODE_REQUIRED: u32 = 0x0000_0040;

    const POWER_REQUEST_CONTEXT_VERSION: u32 = 0;
    const POWER_REQUEST_CONTEXT_SIMPLE_STRING: u32 = 1;

    const POWER_REQUEST_DISPLAY_REQUIRED: u32 = 0;
    const POWER_REQUEST_SYSTEM_REQUIRED: u32 = 1;
    const POWER_REQUEST_AWAYMODE_REQUIRED: u32 = 2;

    #[repr(C)]
    struct ReasonContext {
        version: u32,
        flags: u32,
        simple_reason_string: *const u16,
        _pad1: usize,
        _pad2: usize,
    }

    const HKEY_CURRENT_USER: isize = -2147483647; // 0x80000001
    const KEY_READ: u32 = 0x20019;
    const MUI_LANGUAGE_NAME: u32 = 0x08;
    const LOCALE_NAME_MAX_LENGTH: usize = 85;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Luid {
        low_part: u32,
        high_part: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct LuidAndAttributes {
        luid: Luid,
        attributes: u32,
    }

    #[repr(C)]
    struct TokenPrivileges {
        privilege_count: u32,
        privileges: [LuidAndAttributes; 1],
    }

    const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0020;
    const TOKEN_QUERY: u32 = 0x0008;
    const SE_PRIVILEGE_ENABLED: u32 = 0x0000_0002;
    const ERROR_SHUTDOWN_NOT_IN_PROGRESS: u32 = 1116;

    const SHTDN_REASON_MAJOR_OTHER: u32 = 0x0000_0000;
    const SHTDN_REASON_MINOR_OTHER: u32 = 0x0000_0000;
    const SHTDN_REASON_FLAG_PLANNED: u32 = 0x4000_0000;

    #[link(name = "kernel32")]
    extern "system" {
        fn SetThreadExecutionState(es_flags: u32) -> u32;
        fn PowerCreateRequest(context: *const ReasonContext) -> isize;
        fn PowerSetRequest(power_request: isize, request_type: u32) -> i32;
        fn PowerClearRequest(power_request: isize, request_type: u32) -> i32;
        fn CloseHandle(handle: isize) -> i32;
        fn GetUserPreferredUILanguages(
            dw_flags: u32,
            pul_num_languages: *mut u32,
            pwsz_languages_buffer: *mut u16,
            pcch_languages_buffer: *mut u32,
        ) -> i32;
        fn GetUserDefaultLocaleName(
            lp_locale_name: *mut u16,
            cch_locale_name: i32,
        ) -> i32;
        fn GetCurrentProcess() -> isize;
        fn GetLastError() -> u32;
    }

    #[link(name = "advapi32")]
    extern "system" {
        fn RegOpenKeyExW(
            h_key: isize,
            lp_sub_key: *const u16,
            ul_options: u32,
            sam_desired: u32,
            phk_result: *mut isize,
        ) -> i32;

        fn RegQueryValueExW(
            h_key: isize,
            lp_value_name: *const u16,
            lp_reserved: *const u32,
            lp_type: *mut u32,
            lp_data: *mut u8,
            lpcb_data: *mut u32,
        ) -> i32;

        fn RegCloseKey(h_key: isize) -> i32;

        fn OpenProcessToken(
            process_handle: isize,
            desired_access: u32,
            token_handle: *mut isize,
        ) -> i32;

        fn LookupPrivilegeValueW(
            lp_system_name: *const u16,
            lp_name: *const u16,
            lp_luid: *mut Luid,
        ) -> i32;

        fn AdjustTokenPrivileges(
            token_handle: isize,
            disable_all_privileges: i32,
            new_state: *const TokenPrivileges,
            buffer_length: u32,
            previous_state: *mut TokenPrivileges,
            return_length: *mut u32,
        ) -> i32;

        fn InitiateSystemShutdownExW(
            lp_machine_name: *const u16,
            lp_message: *const u16,
            dw_timeout: u32,
            b_force_apps_closed: i32,
            b_reboot_after_shutdown: i32,
            dw_reason: u32,
        ) -> i32;

        fn AbortSystemShutdownW(
            lp_machine_name: *const u16,
        ) -> i32;
    }

    static POWER_REQUEST: Mutex<Option<isize>> = Mutex::new(None);
    static REASON_UTF16: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    static SUB_KEY_UTF16: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    static VALUE_NAME_UTF16: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    static SE_SHUTDOWN_NAME_UTF16: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();

    /// Enables the SeShutdownPrivilege on the current process token to authorize native Win32 shutdown calls.
    unsafe fn enable_shutdown_privilege() -> bool {
        let mut token_handle: isize = 0;
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            return false;
        }

        let priv_name = SE_SHUTDOWN_NAME_UTF16.get_or_init(|| {
            "SeShutdownPrivilege\0".encode_utf16().collect()
        });

        let mut luid = Luid {
            low_part: 0,
            high_part: 0,
        };
        if LookupPrivilegeValueW(std::ptr::null(), priv_name.as_ptr(), &mut luid) == 0 {
            CloseHandle(token_handle);
            return false;
        }

        let tp = TokenPrivileges {
            privilege_count: 1,
            privileges: [LuidAndAttributes {
                luid,
                attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        let res = AdjustTokenPrivileges(
            token_handle,
            0,
            &tp,
            std::mem::size_of::<TokenPrivileges>() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        CloseHandle(token_handle);
        res != 0
    }


    /// Prevents Windows from sleeping or suspending while the timer is active.
    /// Combines SetThreadExecutionState with PowerCreateRequest for maximum compatibility.
    pub fn set_keepawake(keep_screen_on: bool) {
        unsafe {
            // 1. Thread Execution State (Continuous + System Required + Away Mode)
            let mut flags = ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED;
            if keep_screen_on {
                flags |= ES_DISPLAY_REQUIRED;
            }
            SetThreadExecutionState(flags);

            // 2. Windows Power Manager Request (System + Away Mode + Display)
            let mut lock = POWER_REQUEST.lock().unwrap();
            if lock.is_none() {
                let reason = REASON_UTF16
                    .get_or_init(|| "Shutdown Timer Active\0".encode_utf16().collect());
                let context = ReasonContext {
                    version: POWER_REQUEST_CONTEXT_VERSION,
                    flags: POWER_REQUEST_CONTEXT_SIMPLE_STRING,
                    simple_reason_string: reason.as_ptr(),
                    _pad1: 0,
                    _pad2: 0,
                };
                let handle = PowerCreateRequest(&context);
                if handle != 0 && handle != -1 {
                    PowerSetRequest(handle, POWER_REQUEST_SYSTEM_REQUIRED);
                    PowerSetRequest(handle, POWER_REQUEST_AWAYMODE_REQUIRED);
                    if keep_screen_on {
                        PowerSetRequest(handle, POWER_REQUEST_DISPLAY_REQUIRED);
                    }
                    *lock = Some(handle);
                }
            }
        }
    }

    /// Releases sleep prevention power requests, allowing normal Windows sleep policy.
    pub fn unset_keepawake() {
        unsafe {
            // 1. Reset Thread Execution State
            SetThreadExecutionState(ES_CONTINUOUS);

            // 2. Clear and release Windows Power Request handle
            let mut lock = POWER_REQUEST.lock().unwrap();
            if let Some(handle) = lock.take() {
                PowerClearRequest(handle, POWER_REQUEST_SYSTEM_REQUIRED);
                PowerClearRequest(handle, POWER_REQUEST_AWAYMODE_REQUIRED);
                PowerClearRequest(handle, POWER_REQUEST_DISPLAY_REQUIRED);
                CloseHandle(handle);
            }
        }
    }

    /// Queries Windows system theme directly via Win32 registry API without spawning processes.
    pub fn is_system_dark_mode() -> bool {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let sub_key = SUB_KEY_UTF16.get_or_init(|| {
            OsStr::new(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect()
        });
        let value_name = VALUE_NAME_UTF16.get_or_init(|| {
            OsStr::new("AppsUseLightTheme")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect()
        });

        let mut h_key: isize = 0;
        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                sub_key.as_ptr(),
                0,
                KEY_READ,
                &mut h_key,
            )
        };

        if status != 0 {
            return true;
        }

        let mut data: u32 = 0;
        let mut data_len = std::mem::size_of::<u32>() as u32;
        let mut val_type: u32 = 0;

        let query_status = unsafe {
            RegQueryValueExW(
                h_key,
                value_name.as_ptr(),
                std::ptr::null(),
                &mut val_type,
                &mut data as *mut u32 as *mut u8,
                &mut data_len,
            )
        };

        unsafe {
            RegCloseKey(h_key);
        }

        if query_status == 0 {
            // AppsUseLightTheme: 0 = Dark, 1 = Light
            data == 0
        } else {
            true
        }
    }

    /// Queries the user's preferred UI display languages and default locale names.
    pub fn get_system_locales() -> Vec<String> {
        let mut locales = Vec::new();

        // 1. GetUserPreferredUILanguages (ordered list of preferred UI display languages)
        unsafe {
            let mut num_langs: u32 = 0;
            let mut buffer_size: u32 = 0;
            let res = GetUserPreferredUILanguages(
                MUI_LANGUAGE_NAME,
                &mut num_langs,
                std::ptr::null_mut(),
                &mut buffer_size,
            );

            if res != 0 && buffer_size > 0 {
                let mut buffer: Vec<u16> = vec![0; buffer_size as usize];
                let res2 = GetUserPreferredUILanguages(
                    MUI_LANGUAGE_NAME,
                    &mut num_langs,
                    buffer.as_mut_ptr(),
                    &mut buffer_size,
                );

                if res2 != 0 {
                    let mut current = Vec::new();
                    for &ch in &buffer {
                        if ch == 0 {
                            if !current.is_empty() {
                                if let Ok(s) = String::from_utf16(&current) {
                                    if !locales.contains(&s) {
                                        locales.push(s);
                                    }
                                }
                                current.clear();
                            }
                        } else {
                            current.push(ch);
                        }
                    }
                }
            }
        }

        // 2. GetUserDefaultLocaleName (user default locale / regional format)
        unsafe {
            let mut buf = [0u16; LOCALE_NAME_MAX_LENGTH];
            let len = GetUserDefaultLocaleName(buf.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32);
            if len > 1 {
                if let Ok(s) = String::from_utf16(&buf[..(len as usize - 1)]) {
                    if !locales.contains(&s) {
                        locales.push(s);
                    }
                }
            }
        }

        // 3. Environment variable fallback
        if locales.is_empty() {
            for key in &["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
                if let Ok(val) = std::env::var(key) {
                    if !val.is_empty() && !locales.contains(&val) {
                        locales.push(val);
                    }
                }
            }
        }

        locales
    }

    /// Initiates Windows shutdown with a delay using native Win32 API, falling back to shutdown CLI.
    pub fn trigger_shutdown(timeout_seconds: u32) {
        let native_success = unsafe {
            if enable_shutdown_privilege() {
                InitiateSystemShutdownExW(
                    std::ptr::null(),
                    std::ptr::null(),
                    timeout_seconds,
                    1, // Force apps closed
                    0, // Power off (do not reboot)
                    SHTDN_REASON_MAJOR_OTHER | SHTDN_REASON_MINOR_OTHER | SHTDN_REASON_FLAG_PLANNED,
                ) != 0
            } else {
                false
            }
        };

        if !native_success {
            let _ = Command::new("shutdown")
                .args(["/s", "/t", &timeout_seconds.to_string()])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }
    }

    /// Aborts any scheduled Windows shutdown using native Win32 API, falling back to shutdown CLI.
    pub fn cancel_shutdown() {
        let native_success = unsafe {
            if enable_shutdown_privilege() {
                if AbortSystemShutdownW(std::ptr::null()) != 0 {
                    true
                } else {
                    // If no shutdown was in progress, consider it cleanly handled without fallback noise
                    GetLastError() == ERROR_SHUTDOWN_NOT_IN_PROGRESS
                }
            } else {
                false
            }
        };

        if !native_success {
            let _ = Command::new("shutdown")
                .args(["/a"])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();
        }
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use std::process::{Child, Command};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    static CAFFEINATE_CHILD: Mutex<Option<Child>> = Mutex::new(None);
    static SHUTDOWN_CANCEL_FLAG: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

    /// Prevents macOS from sleeping while the timer is running using the native `caffeinate` utility.
    pub fn set_keepawake(keep_screen_on: bool) {
        let mut lock = CAFFEINATE_CHILD.lock().unwrap();
        if lock.is_none() {
            let mut cmd = Command::new("caffeinate");
            cmd.arg("-i").arg("-s"); // prevent idle & system sleep
            if keep_screen_on {
                cmd.arg("-d"); // prevent display sleep
            }
            if let Ok(child) = cmd.spawn() {
                *lock = Some(child);
            }
        }
    }

    /// Releases caffeinate process, returning power management to standard macOS behavior.
    pub fn unset_keepawake() {
        let mut lock = CAFFEINATE_CHILD.lock().unwrap();
        if let Some(mut child) = lock.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// Queries macOS interface style using `defaults read -g AppleInterfaceStyle`.
    pub fn is_system_dark_mode() -> bool {
        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                return s.trim().eq_ignore_ascii_case("dark");
            }
        }
        false
    }

    /// Queries macOS preferred UI languages and locales via `defaults` and environment variables.
    pub fn get_system_locales() -> Vec<String> {
        let mut locales = Vec::new();

        // 1. AppleLocale (e.g. "it_IT" or "en_US")
        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleLocale"])
            .output()
        {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s.is_empty() && !locales.contains(&s) {
                    locales.push(s);
                }
            }
        }

        // 2. AppleLanguages (e.g. ("it-IT", "en-US", ...))
        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                for line in s.lines() {
                    let cleaned = line
                        .trim()
                        .trim_matches(|c| c == '"' || c == ',' || c == '(' || c == ')' || c == ' ');
                    if !cleaned.is_empty() && !locales.contains(&cleaned.to_string()) {
                        locales.push(cleaned.to_string());
                    }
                }
            }
        }

        // 3. Fallback to standard environment variables
        for key in &["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
            if let Ok(val) = std::env::var(key) {
                if !val.is_empty() && !locales.contains(&val) {
                    locales.push(val);
                }
            }
        }

        locales
    }

    /// Schedules a delayed macOS shutdown via AppleScript (System Events) without requiring root password.
    pub fn trigger_shutdown(timeout_seconds: u32) {
        cancel_shutdown();

        if timeout_seconds == 0 {
            let _ = Command::new("osascript")
                .args(["-e", "tell application \"System Events\" to shut down"])
                .spawn();
            return;
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));
        {
            let mut lock = SHUTDOWN_CANCEL_FLAG.lock().unwrap();
            *lock = Some(cancel_flag.clone());
        }

        let flag = cancel_flag;
        thread::spawn(move || {
            let total_ms = (timeout_seconds as u64) * 1000;
            let mut elapsed_ms = 0;
            while elapsed_ms < total_ms {
                if flag.load(Ordering::SeqCst) {
                    return;
                }
                let step = 100.min(total_ms - elapsed_ms);
                thread::sleep(Duration::from_millis(step));
                elapsed_ms += step;
            }
            if !flag.load(Ordering::SeqCst) {
                let _ = Command::new("osascript")
                    .args(["-e", "tell application \"System Events\" to shut down"])
                    .spawn();
            }
        });
    }

    /// Aborts any scheduled macOS shutdown.
    pub fn cancel_shutdown() {
        let mut lock = SHUTDOWN_CANCEL_FLAG.lock().unwrap();
        if let Some(flag) = lock.take() {
            flag.store(true, Ordering::SeqCst);
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod imp {
    use std::process::{Child, Command};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    static INHIBIT_CHILD: Mutex<Option<Child>> = Mutex::new(None);
    static SHUTDOWN_CANCEL_FLAG: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

    /// Prevents Linux from sleeping while the timer is running via `systemd-inhibit`.
    pub fn set_keepawake(_keep_screen_on: bool) {
        let mut lock = INHIBIT_CHILD.lock().unwrap();
        if lock.is_none() {
            if let Ok(child) = Command::new("systemd-inhibit")
                .args([
                    "--what=idle:sleep",
                    "--who=ShutdownTimer",
                    "--why=Shutdown Timer Active",
                    "--mode=block",
                    "sleep",
                    "infinity",
                ])
                .spawn()
            {
                *lock = Some(child);
            }
        }
    }

    /// Releases sleep inhibition child process.
    pub fn unset_keepawake() {
        let mut lock = INHIBIT_CHILD.lock().unwrap();
        if let Some(mut child) = lock.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// Queries Linux dark mode preference via GNOME/freedesktop gsettings.
    pub fn is_system_dark_mode() -> bool {
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "color-scheme"])
            .output()
        {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                return s.to_lowercase().contains("dark");
            }
        }
        true
    }

    /// Queries Linux locale settings from standard environment variables.
    pub fn get_system_locales() -> Vec<String> {
        let mut locales = Vec::new();
        for key in &["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
            if let Ok(val) = std::env::var(key) {
                if !val.is_empty() && !locales.contains(&val) {
                    locales.push(val);
                }
            }
        }
        locales
    }

    fn execute_linux_shutdown() {
        // 1. Try D-Bus call to org.freedesktop.login1 via gdbus (standard in Flatpak runtime and GNOME/freedesktop)
        if Command::new("gdbus")
            .args([
                "call",
                "--system",
                "--dest",
                "org.freedesktop.login1",
                "--object-path",
                "/org/freedesktop/login1",
                "--method",
                "org.freedesktop.login1.Manager.PowerOff",
                "true",
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return;
        }

        // 2. Try D-Bus call via dbus-send (standard fallback for D-Bus without GLib)
        if Command::new("dbus-send")
            .args([
                "--system",
                "--print-reply",
                "--dest=org.freedesktop.login1",
                "/org/freedesktop/login1",
                "org.freedesktop.login1.Manager.PowerOff",
                "boolean:true",
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return;
        }

        // 3. Try busctl (systemd DBus CLI)
        if Command::new("busctl")
            .args([
                "call",
                "org.freedesktop.login1",
                "/org/freedesktop/login1",
                "org.freedesktop.login1.Manager",
                "PowerOff",
                "b",
                "true",
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return;
        }

        // 4. Try systemctl poweroff (standard on host systemd-based distributions)
        if Command::new("systemctl")
            .arg("poweroff")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return;
        }

        // 5. Try shutdown -h now
        if Command::new("shutdown")
            .args(["-h", "now"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return;
        }

        // 6. Fallback to poweroff
        let _ = Command::new("poweroff").status();
    }

    /// Schedules a delayed Linux shutdown, with accurate millisecond cancel capability.
    pub fn trigger_shutdown(timeout_seconds: u32) {
        cancel_shutdown();

        if timeout_seconds == 0 {
            execute_linux_shutdown();
            return;
        }

        let cancel_flag = Arc::new(AtomicBool::new(false));
        {
            let mut lock = SHUTDOWN_CANCEL_FLAG.lock().unwrap();
            *lock = Some(cancel_flag.clone());
        }

        let flag = cancel_flag;
        thread::spawn(move || {
            let total_ms = (timeout_seconds as u64) * 1000;
            let mut elapsed_ms = 0;
            while elapsed_ms < total_ms {
                if flag.load(Ordering::SeqCst) {
                    return;
                }
                let step = 100.min(total_ms - elapsed_ms);
                thread::sleep(Duration::from_millis(step));
                elapsed_ms += step;
            }
            if !flag.load(Ordering::SeqCst) {
                execute_linux_shutdown();
            }
        });
    }

    /// Aborts any scheduled Linux shutdown.
    pub fn cancel_shutdown() {
        let mut lock = SHUTDOWN_CANCEL_FLAG.lock().unwrap();
        if let Some(flag) = lock.take() {
            flag.store(true, Ordering::SeqCst);
        }
        // In case system-level shutdown was scheduled, try canceling it
        let _ = Command::new("shutdown").arg("-c").status();
    }
}

#[cfg(not(any(windows, unix)))]
mod imp {
    pub fn set_keepawake(_keep_screen_on: bool) {}
    pub fn unset_keepawake() {}
    pub fn is_system_dark_mode() -> bool {
        true
    }
    pub fn get_system_locales() -> Vec<String> {
        let mut locales = Vec::new();
        for key in &["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
            if let Ok(val) = std::env::var(key) {
                if !val.is_empty() && !locales.contains(&val) {
                    locales.push(val);
                }
            }
        }
        locales
    }
    pub fn trigger_shutdown(timeout_seconds: u32) {
        eprintln!("Shutdown not implemented on this OS ({}s timeout)", timeout_seconds);
    }
    pub fn cancel_shutdown() {
        eprintln!("Cancel shutdown not implemented on this OS");
    }
}

pub fn set_keepawake(keep_screen_on: bool) {
    imp::set_keepawake(keep_screen_on);
}

pub fn unset_keepawake() {
    imp::unset_keepawake();
}

/// Returns whether the system theme is currently dark.
pub fn is_system_dark_mode() -> bool {
    imp::is_system_dark_mode()
}

/// Returns system locale / language codes in order of preference.
pub fn get_system_locales() -> Vec<String> {
    imp::get_system_locales()
}

/// Initiates OS shutdown with a timeout (in seconds).
pub fn trigger_shutdown(timeout_seconds: u32) {
    imp::trigger_shutdown(timeout_seconds);
}

/// Aborts any scheduled OS shutdown.
pub fn cancel_shutdown() {
    imp::cancel_shutdown();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keepawake_lifecycle() {
        set_keepawake(true);
        // Repeated calls while already active should be idempotent and safe
        set_keepawake(true);
        unset_keepawake();
        // Repeated calls when inactive should also be safe
        unset_keepawake();
    }

    #[test]
    fn test_get_system_locales() {
        let locales = get_system_locales();
        #[cfg(windows)]
        {
            assert!(!locales.is_empty(), "Expected at least one locale on Windows");
            println!("Detected system locales: {:?}", locales);
        }
        #[cfg(not(windows))]
        {
            println!("Detected system locales: {:?}", locales);
        }
    }

    #[test]
    fn test_shutdown_and_cancel_lifecycle() {
        cancel_shutdown();
        trigger_shutdown(9999);
        cancel_shutdown();
    }
}
