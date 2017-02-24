extern crate winapi;

mod Utilities;

fn main() {
    println!("damn syn is gay lol");

    let process = Utilities::Attach("process.exe");
    let module = Utilities::Module::GetModule("process.exe", "library.dll");

    let mut var1 = process.ReadMemory(module.m_dwBase as u32 + 0xAA66D4);

    let mut once = true;

    while true {
        let mut var2 = process.ReadMemory(var1 + 0x100);
        if once {
            println!("{}", var1);
            println!("{}", var2);
            once = false;
        }
    }
}
