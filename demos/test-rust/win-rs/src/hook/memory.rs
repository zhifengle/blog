use std::ffi::c_void;

use windows::{
    core::{HSTRING, PCWSTR, PWSTR},
    w,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::Debug::{
                GetThreadContext, ReadProcessMemory, WriteProcessMemory, CONTEXT,
            },
            Memory::{VirtualProtectEx, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
            Threading::{
                CreateProcessW, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOW,
            },
        },
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
};

/*
[dependencies.windows]
version = "0.39.0"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_Security",
    "Win32_System_Diagnostics_Debug",
    "Win32_System_Kernel",
    "Win32_System_Memory",
    "Win32_UI_WindowsAndMessaging"
]
 */
fn main() {
    let mut info = STARTUPINFOW::default();
    info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut process = PROCESS_INFORMATION::default();
    let region_bytes: [u8; 3] = [0xb0, 0x01, 0xc3];
    let noop_call_bytes: [u8; 1] = [0xc3];
    unsafe {
        if CreateProcessW(
            w!("C:\\game\\ab\\1\\SiglusEngine.exe"),
            PWSTR::null(),
            std::ptr::null(),
            std::ptr::null(),
            false,
            CREATE_SUSPENDED,
            // CREATE_NEW_CONSOLE,
            std::ptr::null(),
            PCWSTR::null(),
            &info,
            &mut process,
        ) == false
        {
            alert(w!("Start failed!"));
            return;
        }
        let base_addr = get_base_addr(&process);
        if base_addr.is_none() {
            clean(&process);
            return;
        }
        // {:x}
        let addr: u32 = base_addr.unwrap() + 0x22CB20;
        if write_ins(&process, addr, &region_bytes) == false {
            clean(&process);
            alert(w!("Write Failed!!"));
            return;
        }
        let addr: u32 = base_addr.unwrap() + 0x360D80;
        println!("{:x}", addr);
        pause();
        if write_ins(&process, addr, &noop_call_bytes) == false {
            clean(&process);
            alert(w!("Write Failed!!"));
            return;
        }
        // println!("pid: {}", process.dwProcessId);
        // println!("thread id: {}", process.dwThreadId);
        clean(&process);
    }
}

unsafe fn alert(text: &HSTRING) {
    MessageBoxW(None, text, w!("World"), MB_OK);
}
unsafe fn clean(process: &PROCESS_INFORMATION) {
    ResumeThread(process.hThread);
    CloseHandle(process.hThread);
    CloseHandle(process.hProcess);
}

unsafe fn get_base_addr(pi: &PROCESS_INFORMATION) -> Option<u32> {
    let mut ctx = CONTEXT::default();
    ctx.ContextFlags = 65599;
    if GetThreadContext(pi.hThread, &mut ctx) == false {
        alert(w!("Get Context Failed!!"));
        return None;
    }
    let mut base: u32 = 0;
    let base = &mut base as *mut u32 as *mut c_void;
    // 32位程序的写法。64??
    let p = (ctx.Ebx + 0x8) as *mut c_void;
    let result = ReadProcessMemory(
        pi.hProcess,
        p,
        base,
        32,
        // 传空是不关心读取多少??
        std::ptr::null_mut(),
    );
    if result == false {
        alert(w!("Get BaseImage Failed!!"));
        return None;
    }
    Some(*(base as *const u32))
}

unsafe fn write_ins(pi: &PROCESS_INFORMATION, addr: u32, bytes: &[u8]) -> bool {
    let size = bytes.len() * 8;
    let mut pf_protect = PAGE_PROTECTION_FLAGS::default();
    let pf_protect = &mut pf_protect as *mut PAGE_PROTECTION_FLAGS;
    if VirtualProtectEx(
        pi.hProcess,
        addr as *mut c_void,
        size,
        PAGE_EXECUTE_READWRITE,
        pf_protect,
    ) == false
    {
        alert(w!("Access Failed!!"));
        return false;
    }
    WriteProcessMemory(
        pi.hProcess,
        addr as *mut c_void,
        bytes.as_ptr() as *const c_void,
        size,
        std::ptr::null_mut(),
    )
    .as_bool()
}

pub fn pause() {
    let _ = std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status();
}
