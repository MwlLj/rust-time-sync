#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="linux")]
mod linux;

pub fn set_system_time(sec: i64, us: i64) {
    #[cfg(target_os="windows")]
    windows::set_system_time(sec, us);
    #[cfg(target_os="linux")]
    linux::set_system_time(sec, us);
}
