pub mod process_record_handling;
pub mod process_records;

use crate::process::ProcessRecord;
use crate::shared_def::IOMessage;
use crate::worker::process_record_handling::{Exepath, ExepathLive};
use crate::worker::process_records::ProcessRecords;
use std::path::Path;

#[derive(Debug)]
pub struct Worker {
    process_records: ProcessRecords,
    exepath_handler: Box<dyn Exepath>,
}

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            process_records: ProcessRecords::new(),
            exepath_handler: Box::new(ExepathLive::default()),
        }
    }

    pub fn exepath_handler(mut self, exepath: Box<dyn Exepath>) -> Worker {
        self.exepath_handler = exepath;
        self
    }

    pub fn build(self) -> Worker {
        self
    }

    pub fn process_io(&mut self, iomsg: &mut IOMessage) {
        self.register_precord(iomsg);
        if let Some(precord) = self.process_records.get_precord_mut_by_gid(iomsg.gid) {
            precord.add_irp_record(iomsg);
            iomsg.runtime_features.exepath = precord.exepath.clone();
            iomsg.runtime_features.exe_still_exists = true;
        }
    }

    fn register_precord(&mut self, iomsg: &mut IOMessage) {
        // dbg!(&iomsg);
        match self.process_records.get_precord_by_gid(iomsg.gid) {
            None => {
                if let Some(exepath) = &self.exepath_handler.exepath(iomsg) {
                    let appname = self
                        .appname_from_exepath(exepath)
                        .unwrap_or_else(|| String::from("DEFAULT"));
                    if !exepath
                        .parent()
                        .unwrap_or_else(|| Path::new("/"))
                        .starts_with(r"C:\Windows\System32")
                    {
                        let precord = ProcessRecord::from(iomsg, appname, exepath.clone());
                        self.process_records.insert_precord(iomsg.gid, precord);
                    }
                }
            }
            Some(_) => {}
        }
    }

    fn appname_from_exepath(&self, exepath: &Path) -> Option<String> {
        exepath
            .file_name()
            .map(|filename| filename.to_string_lossy().to_string())
    }
}
