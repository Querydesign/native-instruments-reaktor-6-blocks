//! Windows registry read helpers for Native Instruments Reaktor 6 Blocks.

pub const APP_KEY: &str = "Software\\Native\\Native Instruments Reaktor 6 Blocks";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Native Instruments Reaktor 6 Blocks";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Native Instruments Reaktor 6 Blocks") },
        RegValue { name: "Version",     value: String::from("v2.6.22") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Native Instruments Reaktor 6 Blocks") },
        RegValue { name: "Publisher",       value: String::from("Native") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Native Instruments Reaktor 6 Blocks\\uninstall.exe\"") },
    ]
}
