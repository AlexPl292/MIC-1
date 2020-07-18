pub struct BBusControls {
    controls: [bool; 9]
}

impl BBusControls {
    pub fn new(controls: [bool; 9]) -> BBusControls { BBusControls { controls } }
    pub fn mdr(&self) -> bool { self.controls[0] }
    pub fn pc(&self) -> bool { self.controls[1] }
    pub fn mbr1(&self) -> bool { self.controls[2] }
    pub fn mbr2(&self) -> bool { self.controls[3] }
    pub fn sp(&self) -> bool { self.controls[4] }
    pub fn lv(&self) -> bool { self.controls[5] }
    pub fn cpp(&self) -> bool { self.controls[6] }
    pub fn tos(&self) -> bool { self.controls[7] }
    pub fn opc(&self) -> bool { self.controls[8] }
}

pub struct CBusControls {
    controls: [bool; 9]
}

impl CBusControls {
    pub fn new(controls: [bool; 9]) -> CBusControls { CBusControls { controls } }
    pub fn h(&self) -> bool { self.controls[0] }
    pub fn opc(&self) -> bool { self.controls[1] }
    pub fn tos(&self) -> bool { self.controls[2] }
    pub fn cpp(&self) -> bool { self.controls[3] }
    pub fn lv(&self) -> bool { self.controls[4] }
    pub fn sp(&self) -> bool { self.controls[5] }
    pub fn pc(&self) -> bool { self.controls[6] }
    pub fn mdr(&self) -> bool { self.controls[7] }
    pub fn mar(&self) -> bool { self.controls[8] }
}
