use strum::IntoEnumIterator;

use crate::alu::{alu_32, AluControl};
use crate::bus::{Bus32, Bus9};
use crate::decoders::decoder_4x9;
use crate::main_memory::{fast_encode, MainMemory, ReadState, fast_decode};
use crate::main_memory::ReadState::{NoRead, ReadInitialized, ReadInProgress};
use crate::memory::{Memory512x36, Register32, Register36, Register9};
use crate::microasm::MicroAsm;
use crate::processor_elements::{BBusControls, CBusControls};
use crate::shifter::{sll8, sra1};

impl Register36 {
    fn mir_jmpc(self) -> bool { self.get()[9] }
    fn mir_jamn(self) -> bool { self.get()[10] }
    fn mir_jamz(self) -> bool { self.get()[11] }

    fn mir_addr(self) -> [bool; 9] {
        let mut res = [false; 9];
        res.copy_from_slice(&self.get()[..9]);
        res
    }

    fn mir_b_bus_controls(self) -> [bool; 4] {
        let mut res = [false; 4];
        res.copy_from_slice(&self.get()[32..36]);
        res
    }

    fn mir_write(self) -> bool { self.get()[29] }
    fn mir_read(self) -> bool { self.get()[30] }
    fn mir_fetch(self) -> bool { self.get()[31] }

    fn mir_alu_controls(self) -> AluControl {
        let mut code = [false; 6];
        code.copy_from_slice(&self.get()[14..20]);
        AluControl::from(code)
    }

    fn mir_c_bus_controls(self) -> CBusControls {
        let mut code = [false; 9];
        code.copy_from_slice(&self.get()[20..29]);
        CBusControls::new(code)
    }

    fn mir_ssl8(self) -> bool { self.get()[12] }
    fn mir_sra1(self) -> bool { self.get()[13] }
}

pub struct Mic1 {
    mir: Register36,
    mpc: Register9,

    mar: Register32,
    mdr: Register32,
    pub pc: Register32,
    mbr: Register32,
    sp: Register32,
    lv: Register32,
    cpp: Register32,
    pub tos: Register32,
    opc: Register32,
    h: Register32,

    control_memory: Memory512x36,

    main_memory: MainMemory,
    read_state: ReadState,
    fetch_state: ReadState,

    mar_to_read: i32,
    pc_to_read: i32,
}

impl Mic1 {
    pub fn init(main_memory: MainMemory, control_memory: Memory512x36, tos: Register32, pc: Register32, sp: Register32, lv: Register32, mpc: Register9) -> Mic1 {
        Mic1 {
            mir: Register36::new(),
            mpc,
            mar: Register32::new(),
            mdr: Register32::new(),
            pc,
            mbr: Register32::new(),
            sp,
            lv,
            cpp: Register32::new(),
            tos,
            opc: Register32::new(),
            h: Register32::new(),
            control_memory,
            main_memory,
            read_state: ReadState::NoRead,
            fetch_state: ReadState::NoRead,
            mar_to_read: 0,
            pc_to_read: 0
        }
    }

    pub fn run(&mut self, len_of_command: usize, program_start: usize) {
        let last_command = len_of_command + 1 + program_start;
        let mut pc_counter = 0;
        while pc_counter < last_command {
            self.execute_command();
            pc_counter = fast_encode(&self.pc.get()) as usize;
        }
    }

    pub fn execute_command(&mut self) {
        Mic1::print_reg(&self.pc, "PC: ");
        if self.read_state == ReadInProgress {
            self.read_state = NoRead;
            self.mdr.update_from_bus(&Bus32::from(self.main_memory.read(fast_decode(self.mar_to_read))), true);
            Mic1::print_reg(&self.mdr, "MDR: ");
        } else if self.read_state == ReadInitialized {
            self.read_state = ReadInProgress;
        }

        if self.fetch_state == ReadInProgress {
            self.fetch_state = NoRead;
            self.mbr.update_from_bus(&Bus32::from(self.main_memory.read(fast_decode(self.pc_to_read))), true);
        } else if self.fetch_state == ReadInitialized {
            self.fetch_state = ReadInProgress;
        }

        // Read new command
        let mpc_bus = Bus9::from(self.mpc.get());
        let new_command = self.control_memory.get(mpc_bus);

        // Write new command to mir register
        self.mir.update_from_bus(&new_command, true);
        self.print_current_command();

        // Writing
        // XXX
        if self.mir.mir_write() {
            self.main_memory.write_data(fast_encode(&self.mdr.read(true)), fast_encode(&self.mar.read(true)) as usize)
        }

        // Create B bus
        let b_bus_controls = self.mir.mir_b_bus_controls();
        let decoded_b_bus_controls = BBusControls::new(decoder_4x9(b_bus_controls));
        let b_bus = self.run_b_bus(decoded_b_bus_controls);

        // Create A bus
        let a_bus = Bus32::from(self.h.read(true));

        // Calculate C bus
        let (mut c_bus, n_bit, z_bit) = alu_32(a_bus, b_bus, self.mir.mir_alu_controls());

        // Shifting
        c_bus = sll8(c_bus, self.mir.mir_ssl8());
        c_bus = sra1(c_bus, self.mir.mir_sra1());

        // Write C bus into registers
        let c_bus_controls = self.mir.mir_c_bus_controls();
        self.run_c_bus(&c_bus, c_bus_controls);

        // Initialize reads
        // XXX
        if self.mir.mir_read() {
            self.read_state = ReadInitialized;
            self.mar_to_read = fast_encode(&self.mar.get());
            println!("MAR_TO_READ: {}", self.mar_to_read);
        }
        if self.mir.mir_fetch() {
            self.fetch_state = ReadInitialized;
            self.pc_to_read = fast_encode(&self.pc.get())
        }

        // O operation
        // Select next command
        let mut next_command = self.o();
        next_command[8] |= self.f(z_bit, n_bit);
        self.mpc.update(next_command, true);

        return;
    }

    fn f(&self, z: bool, n: bool) -> bool {
        self.mir.mir_jamz() && z || self.mir.mir_jamn() && n
    }

    fn o(&self) -> [bool; 9] {
        let mut next_command = self.mir.mir_addr();

        let mut mbr_value = self.mbr.get();
        for i in 0..8 {
            mbr_value[i] &= self.mir.mir_jmpc()
        }

        for i in 0..8 {
            next_command[i] |= mbr_value[i];
        }

        next_command
    }

    fn run_b_bus(&self, controls: BBusControls) -> Bus32 {
        let mut bus = Bus32::new();

        bus.connect(self.mdr.read(controls.mdr()));
        bus.connect(self.pc.read(controls.pc()));
        bus.connect(self.mbr.read(controls.mbr()));
        bus.connect(self.sp.read(controls.sp()));
        bus.connect(self.lv.read(controls.lv()));
        bus.connect(self.cpp.read(controls.cpp()));
        bus.connect(self.tos.read(controls.tos()));
        bus.connect(self.opc.read(controls.opc()));

        let mut mbru_value = self.mbr.read(controls.mbru());
        for x in 8..32 {
            mbru_value[x] = mbru_value[7];
        }
        bus.connect(mbru_value);

        bus
    }

    fn run_c_bus(&mut self, bus: &Bus32, controls: CBusControls) {
        self.h.update_from_bus(bus, controls.h());
        self.opc.update_from_bus(bus, controls.opc());
        self.tos.update_from_bus(bus, controls.tos());
        self.cpp.update_from_bus(bus, controls.cpp());
        self.lv.update_from_bus(bus, controls.lv());
        self.sp.update_from_bus(bus, controls.sp());
        self.pc.update_from_bus(bus, controls.pc());
        self.mdr.update_from_bus(bus, controls.mdr());
        self.mar.update_from_bus(bus, controls.mar());
    }

    fn print_current_command(&self) {
        println!("Current command: {:?}", self.get_current_command())
    }

    fn get_current_command(&self) -> MicroAsm {
        let current_mir = self.mir.read(true);

        for comm in MicroAsm::iter() {
            if Mic1::arrays_equals(&comm.command(), &current_mir) {
                return comm;
            }
        }
        panic!("Command not found")
    }

    fn print_reg(reg: &Register32, str: &str) {
        let pc_value = reg.read(true);
        let encoded_value = fast_encode(&pc_value);
        println!("{} {:?}", str, encoded_value)
    }

    fn arrays_equals(first: &[bool; 36], second: &[bool; 36]) -> bool {
        for i in 0..36 {
            if first[i] != second[i] {
                return false;
            }
        }
        return true;
    }
}
