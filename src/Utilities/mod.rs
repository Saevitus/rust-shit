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
    pub m_dwBase: *mut u8,
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
<<<<<<< HEAD
        println!("found {} with a PID of: {}", procName, dwPID);

        let hModule =
            unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPMODULE, dwPID) };
=======
        println!("PID: {}", dwPID);
        
        let hModule = unsafe
        {
            kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPMODULE32, dwPID)
        };

        let mut Entry: winapi::MODULEENTRY32W = unsafe { mem::zeroed() };
>>>>>>> origin/master

        if hModule != winapi::INVALID_HANDLE_VALUE {

<<<<<<< HEAD
            let mut Entry: winapi::MODULEENTRY32W = unsafe { mem::zeroed() };
            Entry.dwSize = mem::size_of::<winapi::MODULEENTRY32W>() as u32;

            while unsafe { kernel32::Module32NextW(hModule, &mut Entry) } != 0 {

                let modName = OsString::from_wide_null(&Entry.szModule);
                match modName.into_string() {
                    Ok(s) => {
                        if s.contains(moduleName) {
                            unsafe { kernel32::CloseHandle(hModule) };

                            println!("Base Address of {}: {:?} Size of module: 0x{:X}",
                                     moduleName,
                                     Entry.modBaseAddr,
                                     Entry.modBaseSize);

                            return Module {
                                m_dwBase: Entry.modBaseAddr,
                                m_dwSize: Entry.modBaseSize,
                            };
                        }
                    }
                    Err(_) => {
                        println!("fuck off");
=======
        if hModule != winapi::INVALID_HANDLE_VALUE
        {
            if unsafe { kernel32::Module32FirstW(hModule, &mut Entry) } != 0
            {
                let strmod = OsString::from_wide(&Entry.szModule);
                println!("{:?}", strmod);
                println!("1");
                while unsafe { kernel32::Module32NextW(hModule, &mut Entry) } != 0
                {
                    println!("2");
                    let modName = OsString::from_wide(&Entry.szModule);
                    println!("{:?}", modName);

                    match modName.into_string()
                    {
                        Ok(s) =>
                        {
                            if s.contains(moduleName) 
                            {

                                unsafe { kernel32::CloseHandle(hModule) };
                                println!("returning base addr and size");
                                return Module { m_dwBase: Entry.modBaseAddr, m_dwSize: Entry.modBaseSize}
                            }
                        }, Err(_) => { println!("fuck off"); }
>>>>>>> origin/master
                    }
                }
            }
        }
<<<<<<< HEAD

        println!("couldn't get module");
        return Module {
            m_dwBase: ptr::null_mut(),
            m_dwSize: 0,
        };
=======
        //if unsafe { kernel32::GetLastError() } == winapi::ERROR_BAD_LENGTH { println!("bad length"); }
        let error = unsafe { kernel32::GetLastError() };
        println!("{:?}", error);

        println!("couldn't get module");
        return Module { m_dwBase: ptr::null_mut(), m_dwSize: 0 }
>>>>>>> origin/master
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

<<<<<<< HEAD
    while unsafe { kernel32::Process32NextW(hProcess, &mut Entry) } != 0 {
=======
    if unsafe { kernel32::Process32FirstW(hProcess, &mut Entry) } != 0
    {
>>>>>>> origin/master

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
