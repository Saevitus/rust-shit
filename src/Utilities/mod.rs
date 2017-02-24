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

pub struct Process
{
    m_hProcess: winapi::HANDLE
}

pub struct Module
{
    pub m_dwBase: *mut u8,
    pub m_dwSize: u32
}

impl Process
{
    fn NewHandle(hProcess: winapi::HANDLE) -> Process
    {
        return Process { m_hProcess: hProcess }
    }
}

impl Module
{
    pub fn GetModule(procName: &str, moduleName: &str) -> Module 
    {
        let dwPID = GetProcId(procName);
        let hModule = unsafe
        {
            kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPMODULE, dwPID)
        };

        let mut Entry: winapi::MODULEENTRY32W = unsafe
        {
            mem::uninitialized()
        };

        Entry.dwSize = mem::size_of::<winapi::MODULEENTRY32W>() as u32;

        if unsafe { kernel32::Module32NextW(hModule, &mut Entry) } != 0
        {

            while unsafe { kernel32::Module32NextW(hModule, &mut Entry) } != 0
            {
                let modName = OsString::from_wide(&Entry.szModule);

                match modName.into_string()
                {
                    Ok(s) =>
                    {
                        if s.contains(moduleName) 
                        {
                            unsafe { kernel32::CloseHandle(hModule) };
                            return Module { m_dwBase: Entry.modBaseAddr, m_dwSize: Entry.modBaseSize}
                        }
                    }, Err(_) => { println!("fuck off"); }
                }
            }
        }
       return Module { m_dwBase: ptr::null_mut(), m_dwSize: 0 }
    }
}

pub fn Attach(procName: &str) -> Process 
{
    let dwPID = GetProcId(procName);
    return Process::NewHandle(unsafe
        {
            kernel32::OpenProcess(winapi::PROCESS_VM_READ, 0, dwPID)
        }
    );
}

pub fn GetProcId(name: &str) -> u32
{
    let hProcess = unsafe
    {
        kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0)
    };

    let mut Entry: winapi::PROCESSENTRY32W = unsafe
    {
        mem::uninitialized()
    };

    Entry.dwSize = mem::size_of::<winapi::PROCESSENTRY32W>() as u32;

    if unsafe { kernel32::Process32NextW(hProcess, &mut Entry) } != 0
    {

        while unsafe { kernel32::Process32NextW(hProcess, &mut Entry) } != 0
        {

            let procName = OsString::from_wide(&Entry.szExeFile);

            match procName.into_string()
            {
                Ok(s) => 
                { 
                    if s.contains(name) 
                    {  
                        unsafe { kernel32::CloseHandle(hProcess) };
                        return Entry.th32ProcessID;
                    } 
                }, Err(_) => { println!("fuck off"); }
            }
        }
    }
    return 0;
}