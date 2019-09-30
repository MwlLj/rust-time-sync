pub fn set_system_time(sec: i64, us: i64) {
    unsafe {
        println!("settimeofday, sec: {}, us: {}", sec, us);
        libc::settimeofday(&libc::timeval{tv_sec: sec, tv_usec: us} as *const libc::timeval, std::ptr::null());
    }
}
