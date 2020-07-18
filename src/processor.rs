use crate::alu::{alu_36, AluControl};
use crate::bus::{Bus36, Bus9};
use crate::decoders::decoder_4x9;
use crate::memory::{Memory512x36, Register36, Register9};
use crate::processor_elements::{BBusControls, CBusControls};

impl Register36 {
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

    fn mir_alu_controls(self) -> AluControl {
        let mut code = [false; 6];
        code.copy_from_slice(&self.get()[14..20]);
        AluControl::from(code)
    }

    fn mir_c_bus_controls(self) -> CBusControls {
        let mut code = [false; 9];
        code.copy_from_slice(&self.get()[21..30]);
        CBusControls::new(code)
    }
}

pub struct Mic1 {
    mir: Register36,
    mpc: Register9,

    mar: Register36,
    mdr: Register36,
    pc: Register36,
    mbr: Register36,
    sp: Register36,
    lv: Register36,
    cpp: Register36,
    tos: Register36,
    opc: Register36,
    h: Register36,

    control_memory: Memory512x36,
}

impl Mic1 {
    pub fn execute_command(&mut self) {
        // Read new command
        let mpc_bus = Bus9::from(self.mpc.get());
        let new_command = self.control_memory.get(mpc_bus);

        // Write new command to mir register
        self.mir.update_from_bus(&new_command, true);

        // Create B bus
        let b_bus_controls = self.mir.mir_b_bus_controls();
        let decoded_b_bus_controls = BBusControls::new(decoder_4x9(b_bus_controls));
        let b_bus = self.run_b_bus(decoded_b_bus_controls);

        // Create A bus
        let a_bus = Bus36::from(self.h.read(true));

        let (c_bus, carry) = alu_36(a_bus, b_bus, self.mir.mir_alu_controls());

        //----- Shifting missed

        self.run_c_bus(&c_bus, self.mir.mir_c_bus_controls());

        return;
    }

    fn run_b_bus(&self, controls: BBusControls) -> Bus36 {
        let mut bus = Bus36::new();

        bus.connect(self.mdr.read(controls.mdr()));
        bus.connect(self.pc.read(controls.pc()));
        bus.connect(self.mbr.read(controls.mbr1()));
        bus.connect(self.mbr.read(controls.mbr2()));
        bus.connect(self.sp.read(controls.sp()));
        bus.connect(self.lv.read(controls.lv()));
        bus.connect(self.cpp.read(controls.cpp()));
        bus.connect(self.tos.read(controls.tos()));
        bus.connect(self.opc.read(controls.opc()));

        bus
    }

    fn run_c_bus(&mut self, bus: &Bus36, controls: CBusControls) {
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
}
