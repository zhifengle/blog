use windows::{
    core::{PCWSTR, PWSTR},
    w,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Threading::{
                CreateProcessW, TerminateProcess, CREATE_NO_WINDOW,
                PROCESS_INFORMATION, STARTF_USESHOWWINDOW, STARTUPINFOW,
            },
        },
    },
};

fn main() {
    let mut info = STARTUPINFOW::default();
    info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    info.dwFlags = STARTF_USESHOWWINDOW;
    info.wShowWindow = 0;
    let mut process = PROCESS_INFORMATION::default();
    unsafe {
        if CreateProcessW(
            w!("C:\\Windows\\System32\\notepad.exe"),
            PWSTR::null(),
            std::ptr::null(),
            std::ptr::null(),
            false,
            CREATE_NO_WINDOW,
            std::ptr::null(),
            PCWSTR::null(),
            &info,
            &mut process,
        ) == false
        {
            return;
        }
        println!("pid: {}", process.dwProcessId);
        println!("thread id: {}", process.dwThreadId);
        TerminateProcess(process.hProcess, 0);
        CloseHandle(process.hProcess);
        CloseHandle(process.hThread);
    }
}
