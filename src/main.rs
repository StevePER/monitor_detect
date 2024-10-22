use std::convert::TryFrom;
use std::io::Error;
use std::mem;
use std::ptr;

use winapi::shared::minwindef::{BOOL, LPARAM, TRUE};
use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};

struct MonitorInformation {}

impl MonitorInformation {
    pub fn print_monitor_info() {
        // Get monitor information, based on:
        // https://patriksvensson.se/posts/2020/06/enumerating-monitors-in-rust-using-win32-api

        // FIXME in the release build in rust 1.78.0 and later, I think `monitors` is being
        // optimised out causing undefined behaviour
        let mut monitors = Vec::<MONITORINFOEXW>::new();
        let userdata = std::ptr::addr_of_mut!(monitors);

        let success = unsafe {
            EnumDisplayMonitors(
                ptr::null_mut(),
                ptr::null(),
                Some(enumerate_monitors_callback),
                userdata as LPARAM,
            )
        };

        if success == TRUE {
            for monitor in monitors {
                // use working area
                let params = monitor.rcWork;
                println!(
                    "Display left {} right {} top {} bottom {}",
                    params.left, params.right, params.top, params.bottom
                );
            }
        } else {
            println!("Could not enumerate monitors: {}", Error::last_os_error());
        }
    }
}

/// The callback from EnumDisplayMonitors
unsafe extern "system" fn enumerate_monitors_callback(
    monitor: HMONITOR,
    _: HDC,
    _: LPRECT,
    userdata: LPARAM,
) -> BOOL {
    // Get the userdata where we will store the result
    let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);

    // Initialize the MONITORINFOEXW structure and get a pointer to it
    let mut monitor_info: MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = u32::try_from(mem::size_of::<MONITORINFOEXW>())
        .expect("Size of MONITORINFOEXW struct out of range.");
    let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

    // Call the GetMonitorInfoW win32 API
    let result = GetMonitorInfoW(monitor, monitor_info_ptr);
    println!("result={}, TRUE={}", result, TRUE);
    if result == TRUE {
        // Push the information we received to userdata
        monitors.push(monitor_info);
    } else {
        println!("Error enumerating monitor information. GetMonitorInfoW failed, result != TRUE");
    }

    TRUE
}

fn main() {
    MonitorInformation::print_monitor_info();
}
