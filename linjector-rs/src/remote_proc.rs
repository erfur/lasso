use proc_maps::{get_process_maps, MapRange, Pid};

use crate::{remote_mem::RemoteMem, remote_module::RemoteModule};

pub(crate) struct RemoteProc {
    pid: u16,
    pub rm: RemoteMem,
}

impl RemoteProc {
    pub fn new(pid: u16) -> Self {
        let rm = RemoteMem::new(pid);
        Self { pid, rm }
    }

    pub fn get_maps(&self) -> Vec<MapRange> {
        get_process_maps(self.pid as Pid).unwrap()
    }

    pub fn get_maps_by_name(&self, name: &str) -> Vec<MapRange> {
        let maps = self.get_maps();
        let mut maps_by_name: Vec<MapRange> = Vec::new();
        for map in maps {
            match map.filename() {
                None => continue,
                Some(filename) => {
                    if filename.ends_with(name) {
                        maps_by_name.push(map);
                    }
                }
            }
        }
        maps_by_name
    }

    pub fn get_module_bytes(&self, module_name: &str) -> Vec<u8> {
        let maps = self.get_maps_by_name(module_name);
        let mut module_bytes: Vec<u8> = Vec::new();
        for map in maps {
            // debug!("map: {:?}", map);
            module_bytes.resize(map.offset, 0);
            let mut buf = self.rm.read_mem(map.start(), map.size());
            module_bytes.append(&mut buf);
        }
        module_bytes
    }

    pub fn get_module(&self, module_name: &str) -> RemoteModule {
        let maps = self.get_maps_by_name(module_name);
        RemoteModule::new(
            maps[0].filename().unwrap().to_str().unwrap(),
            maps[0].start(),
            self.get_module_bytes(module_name),
        )
    }
}
