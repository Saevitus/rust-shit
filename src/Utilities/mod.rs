// disable cancerous warnings
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

// crates
extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate wio;

// crate libraries
use self::wio::wide::FromWide;

// standard library
use std::ptr;
use std::mem;
use std::ffi::OsString;

#[derive(Clone, Copy)]
pub struct Process {
    m_hProcess: winapi::HANDLE,
}

pub struct Module {
    pub m_dwBase: u32,
    pub m_dwSize: u32,
}

impl Process {
    fn NewHandle(hProcess: winapi::HANDLE) -> Process {
        return Process { m_hProcess: hProcess };
    }

    pub fn ReadMemory(self, address: u32) -> u32 {
        let mut number = unsafe { mem::zeroed() };
        let rpm = unsafe {
            kernel32::ReadProcessMemory(self.m_hProcess,
                                        address as *const _,
                                        &mut number as *mut _ as *mut _,
                                        mem::size_of::<u32>() as winapi::SIZE_T,
                                        ptr::null_mut())
        };
        return number;
    }
}

impl Module {
    pub fn GetModule(procName: &str, moduleName: &str) -> Module {
        let dwPID = GetProcId(procName);
        println!("found {} with a PID of: {}", procName, dwPID);

        let hModule =
            unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPMODULE, dwPID) };

        if hModule != winapi::INVALID_HANDLE_VALUE {

            let mut Entry: winapi::MODULEENTRY32W = unsafe { mem::zeroed() };
            Entry.dwSize = mem::size_of::<winapi::MODULEENTRY32W>() as u32;

            while unsafe { kernel32::Module32NextW(hModule, &mut Entry) } != 0 {

                let modName = OsString::from_wide_null(&Entry.szModule);
                match modName.into_string() {
                    Ok(s) => {
                        if s.contains(moduleName) {
                            unsafe { kernel32::CloseHandle(hModule) };

                            println!("Base Address of {}: 0x{:X} Size of module: 0x{:X}",
                                     moduleName,
                                     Entry.modBaseAddr as u32,
                                     Entry.modBaseSize);

                            return Module {
                                m_dwBase: Entry.modBaseAddr as u32,
                                m_dwSize: Entry.modBaseSize,
                            };
                        }
                    }
                    Err(_) => {
                        println!("fuck off");
                    }
                }
            }
        }

        println!("couldn't get module");
        return Module {
            m_dwBase: 0x0,
            m_dwSize: 0x0,
        };
    }
}

pub fn Attach(procName: &str) -> Process {
    let dwPID = GetProcId(procName);
    return Process::NewHandle(unsafe { kernel32::OpenProcess(winapi::PROCESS_VM_READ, 0, dwPID) });
}

fn GetProcId(name: &str) -> u32 {
    let hProcess = unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0) };

    let mut Entry: winapi::PROCESSENTRY32W = unsafe { mem::zeroed() };

    Entry.dwSize = mem::size_of::<winapi::PROCESSENTRY32W>() as u32;

    while unsafe { kernel32::Process32NextW(hProcess, &mut Entry) } != 0 {

        let procName = OsString::from_wide(&Entry.szExeFile);

        match procName.into_string() {
            Ok(s) => {
                if s.contains(name) {
                    unsafe { kernel32::CloseHandle(hProcess) };
                    return Entry.th32ProcessID;
                }
            }
            Err(_) => {
                println!("fuck off");
            }
        }
    }
    return 0;
}