use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::env;
use winapi::um::shellapi::{ShellExecuteExW, SHELLEXECUTEINFOW, SEE_MASK_NOCLOSEPROCESS};
use winapi::um::winuser::SW_HIDE;
use winapi::um::processthreadsapi::GetProcessId;

fn to_wide<S: AsRef<OsStr>>(s: S) -> Vec<u16> {
    s.as_ref().encode_wide().chain(std::iter::once(0)).collect()
}

fn main() {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    let mp_mesh_path = exe_dir.join("yggdrasil_wrapper.exe");

    let verb = to_wide("runas");
    let file = to_wide(mp_mesh_path.to_str().unwrap());

    let mut sei = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        hwnd: ptr::null_mut(),
        lpVerb: verb.as_ptr(),
        lpFile: file.as_ptr(),
        lpParameters: ptr::null(),
        lpDirectory: ptr::null(),
        nShow: SW_HIDE,
        hInstApp: ptr::null_mut(),
        lpIDList: ptr::null_mut(),
        lpClass: ptr::null(),
        hkeyClass: ptr::null_mut(),
        dwHotKey: 0,
        hMonitor: ptr::null_mut(),  
        hProcess: ptr::null_mut(),
    };

    let success = unsafe { ShellExecuteExW(&mut sei) };
    if success == 0 {
        eprintln!("Failed to launch yggdrasil_wrapper.exe");
        std::process::exit(1);
    }

    let pid = unsafe { GetProcessId(sei.hProcess) };
    println!("{}", pid); // Godot can capture this output
}