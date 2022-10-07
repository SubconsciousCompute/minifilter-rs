use std::collections::HashMap;
use std::os::raw::c_ulonglong;

use crate::process::ProcessRecord;

#[derive(Debug)]
pub struct ProcessRecords {
    pub process_records: HashMap<c_ulonglong, ProcessRecord>,
}

impl Default for ProcessRecords {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessRecords {
    pub fn new() -> ProcessRecords {
        ProcessRecords {
            process_records: HashMap::new(),
        }
    }

    pub fn get_precord_by_gid(&self, gid: c_ulonglong) -> Option<&ProcessRecord> {
        self.process_records.get(&gid)
    }

    pub fn get_precord_mut_by_gid(&mut self, gid: c_ulonglong) -> Option<&mut ProcessRecord> {
        self.process_records.get_mut(&gid)
    }

    pub fn insert_precord(&mut self, gid: c_ulonglong, precord: ProcessRecord) {
        self.process_records.insert(gid, precord);
    }
}
