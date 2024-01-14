use nix::sys::uio::{pread, pwrite};
use std::fs::{File, OpenOptions};

#[derive(Debug)]
pub(crate) struct RemoteMem {
    pid: u16,
    mem_path: String,
    fd: File,
}

impl RemoteMem {
    pub fn new(pid: u16) -> Self {
        let mem_path: String = format!("/proc/{}/mem", pid);
        // open file in read-write mode
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&mem_path)
            .unwrap();
        Self { pid, mem_path, fd }
    }

    // fn open_largefile(&mut self) {
    //     let mode = (libc::O_RDWR | libc::O_LARGEFILE) as i32;
    //     let mut fd = OpenOptions::new().mode(mode).open();
    // }

    pub fn read_mem(&self, addr: usize, len: usize) -> Vec<u8> {
        let mut buf = vec![0; len];
        self.read_mem_vec(addr, &mut buf);
        return buf;
    }

    pub fn read_mem_vec(&self, addr: usize, buf: &mut Vec<u8>) {
        pread(&self.fd, buf, addr as i64).unwrap();
    }

    pub fn write_mem(&self, addr: usize, buf: &Vec<u8>) {
        match pwrite(&self.fd, &buf, addr as i64) {
            Ok(_) => {}
            Err(e) => {
                error!("error while writing into remote memory: {:?}", e);
            }
        }
    }

    /// Write code into remote memory, leaving the first `skip` instructions to last.
    pub fn write_code(&self, addr: usize, buf: &Vec<u8>, skip: usize) {
        let skip_offset = skip*4;
        match pwrite(&self.fd, &buf[skip_offset..], (addr+skip_offset) as i64) {
            Ok(_) => {}
            Err(e) => {
                error!("error while writing into remote memory: {:?}", e);
            }
        }

        match pwrite(&self.fd, &buf[..skip_offset], (addr) as i64) {
            Ok(_) => {}
            Err(e) => {
                error!("error while writing into remote memory: {:?}", e);
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use proc_maps::{get_process_maps, Pid};

    use super::*;

    #[test]
    fn test_read_mem() {
        let mut remote_mem = RemoteMem::new(std::process::id() as u16);
        let buf = remote_mem.read_mem(0x7f7f7f7f7f7f, 0x10);
        println!("{:?}", buf);
    }

    #[test]
    fn test_list_self_maps() {
        let pid: u32 = std::process::id();
        let maps = get_process_maps(Pid::from(pid as u16)).unwrap();
        for map in maps {
            println!("{:?}", map);
        }
    }
}
