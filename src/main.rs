
mod Utilities;

fn main() 
{
    println!("damn syn is gay lol");

    let pid = Utilities::GetProcId("Discord.exe");
    let module = Utilities::Module::GetModule("Discord.exe", "Discord.exe");

    println!("PID: {}, Size of module: {:?}", pid, module.m_dwSize);

}
