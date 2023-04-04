use std::ffi::*;
use std::mem;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    executable: PathBuf,

    #[arg(short, long)]
    arguments: Option<String>,

    #[arg(short, long)]
    library: PathBuf,

    #[arg(short, long)]
    delay: Option<f32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let executable = cli.executable;
    let arguments = cli.arguments;
    let library = cli.library;

    let mut process = Command::new(executable);

    if let Some(arguments) = arguments {
        process.raw_arg(arguments);
    }

    let process = process.spawn()?;

    if let Some(delay) = cli.delay {
        thread::sleep(Duration::from_secs_f32(delay));
    }

    unsafe {
        let process = OpenProcess(PROCESS_ALL_ACCESS, false, process.id())?;

        let kernel32 = GetModuleHandleA(s!("kernel32.dll"))?;
        let load_library = GetProcAddress(kernel32, s!("LoadLibraryA")).unwrap();

        let local_buffer = CString::new(library.canonicalize()?.to_str().unwrap())?;

        let remote_buffer = VirtualAllocEx(
            process,
            None,
            local_buffer.as_bytes().len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        WriteProcessMemory(
            process,
            remote_buffer,
            local_buffer.as_ptr() as _,
            local_buffer.as_bytes().len(),
            None,
        );

        CreateRemoteThread(
            process,
            None,
            0,
            Some(mem::transmute(load_library)),
            Some(remote_buffer),
            0,
            None,
        )?;

        thread::sleep(Duration::from_secs(1));
        VirtualFreeEx(process, remote_buffer, 0, MEM_RELEASE);
        CloseHandle(process);
    }

    Ok(())
}
