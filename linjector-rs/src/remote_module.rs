pub(crate) struct RemoteModule {
    pub name: String,
    pub vm_addr: usize,
    #[allow(dead_code)]
    pub bytes: Vec<u8>,
}

impl RemoteModule {
    pub fn new(name: &str, vm_addr: usize, bytes: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            vm_addr,
            bytes,
        }
    }

    pub fn dlsym_from_fs(&self, symbol_name: &str) -> usize {
        let bytes = std::fs::read(&self.name).unwrap();
        let elf = goblin::elf::Elf::parse(&bytes).unwrap();

        let result = elf
            .syms
            .iter()
            .find(|sym| {
                symbol_name == elf.strtab.get_at(sym.st_name).unwrap()
            });

        if !result.is_none() {
            let offset = result.unwrap().st_value as usize;
            return offset + self.vm_addr;
        }

        warn!("symbol not found in .symtab, trying .dynsym: {}", symbol_name);
            
        let result = elf
                .dynsyms
                .iter()
                .find(|sym| {
                    symbol_name == elf.dynstrtab.get_at(sym.st_name).unwrap()
                });

        if result.is_none() {
            error!("symbol not found: {}", symbol_name);
            panic!();
        }

        let offset = result.unwrap().st_value as usize;
        offset + self.vm_addr
    }

    /// This function is not yet fully implemented.
    #[warn(dead_code)]
    pub fn _dlsym_from_mem(&self, _symbol_name: &str) -> usize {
        let header = goblin::elf::Elf::parse_header(&self.bytes).unwrap();
        let _elf = goblin::elf::Elf::lazy_parse(header).unwrap();

        let ctx = goblin::container::Ctx::new(
            goblin::container::Container::Big,
            goblin::container::Endian::Little,
        );

        let program_headers = goblin::elf::program_header::ProgramHeader::parse(
            &self.bytes.as_slice(),
            header.e_phoff as usize,
            header.e_phnum as usize,
            ctx,
        )
        .unwrap();

        debug!("program_headers: {:?}", program_headers);

        // let mut dynrva: usize = 0;
        // let mut dynvsz: usize = 0;

        // for ph in &program_headers {
        //     if ph.p_type == goblin::elf::program_header::PT_DYNAMIC {
        //         dynrva = ph.p_vaddr as usize;
        //         dynvsz = ph.p_memsz as usize;
        //     }
        // }

        // if (dynrva == 0) || (dynvsz == 0) {
        //     panic!("no dynamic section found");
        // }

        let dyn_header = goblin::elf::dynamic::Dynamic::parse(
            &self.bytes.as_slice(),
            &program_headers.as_slice(),
            ctx,
        )
        .unwrap()
        .unwrap();

        debug!("dyn_header: {:?}", dyn_header);

        let strtab_addr: usize = dyn_header.info.strtab as usize;
        let strtab_sz: usize = dyn_header.info.strsz as usize;
        let _symtab_addr: usize = dyn_header.info.symtab as usize;
        let delim: u8 = 0;

        // for dyn_entry in dyn_header.dyns {
        //     if dyn_entry.d_tag == goblin::elf::dynamic::DT_STRTAB {
        //         strtab_addr = dyn_entry.d_val as usize;
        //     }
        //     if dyn_entry.d_tag == goblin::elf::dynamic::DT_SYMTAB {
        //         symtab_addr = dyn_entry.d_val as usize;
        //     }
        //     if dyn_entry.d_tag == goblin::elf::dynamic::DT_SYMENT {
        //         syment_addr = dyn_entry.d_val as usize;
        //     }
        // }

        // if (strtab_addr == 0) || (symtab_addr == 0) || (syment_addr == 0) {
        //     panic!("no strtab, symtab, or syment found");
        // }

        debug!("strtab_addr: 0x{:x}", strtab_addr);
        debug!("strtab_sz: 0x{:x}", strtab_sz);
        // debug!("symtab_addr: 0x{:x}", symtab_addr);
        // debug!("symtab_sz: 0x{:x}", symtab_sz);

        let strtab =
            goblin::strtab::Strtab::parse(&self.bytes.as_slice(), strtab_addr, strtab_sz, delim)
                .unwrap();

        debug!("strtab: {:?}", strtab);

        // let symtab = goblin::elf::sym::Symtab::parse(
        //     &self.bytes.as_slice(),
        //     symtab_addr,
        //     symtab_sz,
        //     ctx
        // ).unwrap();

        // debug!("symtab: {:?}", symtab);

        let gnu_hash_offset = dyn_header.info.gnu_hash.unwrap() as usize;
        // hexdump first two qwords
        let nbuckets = &self.bytes[gnu_hash_offset..gnu_hash_offset + 8];
        let symndx = &self.bytes[gnu_hash_offset + 8..gnu_hash_offset + 16];

        debug!("nbuckets: {:?}", nbuckets);
        debug!("symndx: {:?}", symndx);

        // let gnu_hash = goblin::elf64::gnu_hash::GnuHash::from_raw_table(
        //     &self.bytes.as_slice(),
        //     &program_headers.as_slice(),
        //     ctx
        // ).unwrap();

        let symbol_addr: usize = 0;

        // for sym in symtab.iter() {
        //     let sym_name = strtab.get_at(sym.st_name).unwrap();
        //     if sym_name == symbol_name {
        //         symbol_addr = sym.st_value as usize;
        //         break;
        //     }
        // }

        // if symbol_addr == 0 {
        //     panic!("symbol not found");
        // }

        // debug!("symbol_addr: 0x{:x}", symbol_addr);

        symbol_addr
    }
}
