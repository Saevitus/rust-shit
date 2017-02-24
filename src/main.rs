
mod Utilities;

fn main() 
{
    println!("damn syn is gay lol");

    let pid = Utilities::GetProcId("Discord.exe");
    let module = Utilities::Module::GetModule("Discord.exe", "Discord.dll");

    println!("PID: {}, Size of module: {:?}", pid, module.m_dwSize);

}
