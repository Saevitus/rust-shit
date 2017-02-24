
mod Utilities;

fn main() {
    println!("damn syn is gay lol");

<<<<<<< HEAD
    let process = Utilities::Attach("program.exe");
    let module = Utilities::Module::GetModule("program.exe", "library.dll");
=======
    let pid = Utilities::GetProcId("Discord.exe");
    let module = Utilities::Module::GetModule("Discord.exe", "Discord.exe");
>>>>>>> origin/master

    let var1 = process.ReadMemory(module.m_dwBase as u32 + 0xAA66D4);
    let var2 = process.ReadMemory(var1 + 0x100);

    while true {
        println!("{:X}", var2);
    }
}
