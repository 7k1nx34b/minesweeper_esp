use winapi::shared::ntdef::{HANDLE, NULL};
use winapi::shared::minwindef::{FALSE, LPDWORD, LPVOID};

use winapi::um::winnt::*;
use winapi::um::processthreadsapi::*;
use winapi::um::handleapi::*;
use winapi::um::winuser::*;
use winapi::um::memoryapi::*;

use std::{thread, time};
use std::ffi::{c_char, CString};
use std::io;

fn main() {

    let mut pid: u32 = 0;

    let w_name = CString::new("Minesweeper").unwrap();

    let h_wnd = unsafe {
        FindWindowA(
            NULL as LPCSTR, w_name.into_raw() as *mut c_char
        )
    };

    unsafe  {
        GetWindowThreadProcessId(h_wnd, &mut pid as *mut _ as LPDWORD);
    }

    println!("{}", pid);

    let h_proc: HANDLE = unsafe {
        OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)
    };

    if h_proc == NULL {
        println!("{:?}", io::Error::last_os_error());
    }

    for h in 0..16 { // 16x16 Intermediate
        for w in 0..16 {
            let mut v: u8 = Default::default();

            let address = 0x1005361 + w + h * 32; //

            unsafe {
                let r = ReadProcessMemory(
                    h_proc,
                    address as *mut _,
                    &mut v as *mut _ as LPVOID,
                    std::mem::size_of::<c_char>(),
                    NULL as *mut usize,
                );
                if r == FALSE {
                    println!("{:?}", std::io::Error::last_os_error());
                }
            }
            if v == 0x8f {
                print!("1");
            }
            else if v == 0x0f {
                print!("0");
            }
        }

        println!();
    }

    unsafe { CloseHandle(h_proc) };
    thread::sleep(time::Duration::from_secs(120));

    return;
}

