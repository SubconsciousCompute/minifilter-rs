use std::fmt::Debug;
use std::path::PathBuf;
use windows::Win32::Foundation::{CloseHandle, GetLastError};
use windows::Win32::System::Diagnostics::Debug::DebugActiveProcess;
use windows::Win32::System::ProcessStatus::K32GetProcessImageFileNameA;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use crate::process::{ProcessRecord, ProcessState};
use crate::shared_def::IOMessage;

pub trait Exepath: Debug {
    fn exepath(&self, iomsg: &IOMessage) -> Option<PathBuf>;
}

#[derive(Default, Debug)]
pub struct ExepathLive;

impl Exepath for ExepathLive {
    fn exepath(&self, iomsg: &IOMessage) -> Option<PathBuf> {
        let pid = iomsg.pid as u32;
        unsafe {
            let r_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
            if let Ok(handle) = r_handle {
                if !(handle.is_invalid() || handle.0 == 0) {
                    let mut buffer: Vec<u8> = Vec::new();
                    buffer.resize(1024, 0);
                    let res = K32GetProcessImageFileNameA(handle, buffer.as_mut_slice());

                    CloseHandle(handle);
                    if res == 0 {
                        let _errorcode = GetLastError().0;
                    } else {
                        let pathbuf = PathBuf::from(
                            String::from_utf8_unchecked(buffer).trim_matches(char::from(0)),
                        );
                        return Some(pathbuf);
                    }
                    // dbg!(is_closed_handle);
                }
            }
            None
        }
    }
}

fn try_suspend(proc: &mut ProcessRecord) {
    proc.process_state = ProcessState::Suspended;
    for pid in &proc.pids {
        unsafe {
            DebugActiveProcess(*pid as u32);
        }
    }
}
