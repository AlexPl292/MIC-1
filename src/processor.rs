use std::collections::VecDeque;

use strum::IntoEnumIterator;

use crate::alu::{alu_32, AluControl};
use crate::asm::IjvmCommand::NOP;
use crate::bus::{Bus32, Bus9};
use crate::decoders::decoder_4x9;
use crate::main_memory::{fast_decode, fast_encode, MainMemory, ReadState};
use crate::main_memory::ReadState::{NoRead, ReadInitialized, ReadInProgress};
use crate::memory::{Memory512x36, Register32, Register36, Register9};
use crate::microasm::MicroAsm;
use crate::microasm::MicroAsm::{invokevirtual14, invokevirtual15, nop1, wide2, wide_iload1, Main1};
use crate::processor_elements::{BBusControls, CBusControls};
use crate::shifter::{sll8, sra1};
use crate::STACK_START;

pub struct Mic1 {
    mir: Register36,
    mpc: Register9,

    pub mar: Register32,
    pub mdr: Register32,
    pub pc: Register32,
    pub mbr: Register32,
    pub sp: Register32,
    pub lv: Register32,
    pub cpp: Register32,
    pub tos: Register32,
    pub opc: Register32,
    pub h: Register32,

    control_memory: Memory512x36,

    pub main_memory: MainMemory,
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

    pub fn run_until_stop(&mut self, stop_instruction: i32) {
        let mut pc_counter = 0;
        let mut hit_stop = false;
        while !(hit_stop && self.get_current_command() == Main1) {
            self.execute_command();
            pc_counter = fast_encode(&self.pc.get()) as usize;
            hit_stop = hit_stop || self.main_memory.read_number(pc_counter) == stop_instruction;
        }

        // Finish last command
        self.execute_command();
        while self.get_current_command() != Main1 {
            self.execute_command();
        }
    }

    pub fn run_n_times(&mut self, len_of_command: usize) {
        let mut protect_counter = 0;
        while protect_counter < len_of_command {
            self.execute_command();
            fast_encode(&self.pc.get()) as usize;
            protect_counter += 1;
        }
    }

    pub fn execute_command(&mut self) {
        // Debugging info
        self.print_stack();
        Mic1::print_reg(&self.pc, "PC: ");
        Mic1::print_reg(&self.lv, "LV: ");
        Mic1::print_reg(&self.tos, "TOS: ");
        Mic1::print_reg(&self.sp, "SP: ");
        Mic1::print_reg(&self.h, "H: ");
        Mic1::print_reg(&self.mar, "MAR: ");

        // Update registers from the main memory
        let (data, enabled) = self.main_memory.check_first_read();
        self.mdr.update_from_bus(&Bus32::from(data), enabled);
        let (data, enabled) = self.main_memory.check_second_read();
        self.mbr.update_from_bus(&Bus32::from(data), enabled);

        // Read new command
        let mpc_bus = Bus9::from(self.mpc.get());
        let new_command = self.control_memory.get(mpc_bus);

        // Write new command to mir register
        self.mir.update_from_bus(&new_command, true);
        self.print_current_command();

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
        self.main_memory.request_first_read(self.mar.get(), self.mir.mir_read());
        self.main_memory.request_second_read(self.pc.get(), self.mir.mir_fetch());

        // Writing
        self.main_memory.write(self.mdr.get(), self.mar.get(), self.mir.mir_write());

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
        println!("Cannot find command. Return NOP");
        return nop1;
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

    fn arrays_equals(first: &[bool; 36], second: &[bool; 36]) -> bool {
        for i in 0..36 {
            if first[i] != second[i] {
                return false;
            }
        }
        return true;
    }

    fn print_reg(reg: &Register32, str: &str) {
        let pc_value = reg.read(true);
        let encoded_value = fast_encode(&pc_value);
        println!("{} {:?}", str, encoded_value)
    }

    fn print_stack(&self) {
        let stack_ptr = fast_encode(&self.sp.get());
        let stack_size = stack_ptr - STACK_START + 1;
        let mut real_stack = Vec::new();
        for x in 0..stack_size {
            real_stack.push(fast_encode(&self.main_memory.read(fast_decode(x + STACK_START))));
        }

        println!("{:?}", real_stack);
    }
}
