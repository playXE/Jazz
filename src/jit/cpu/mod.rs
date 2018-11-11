pub mod x64;

pub const REG_COUNT: usize = 16;


pub struct State {
    pub pc: usize,
    pub sp: usize,
    pub ra: usize,

    pub regs: [usize;REG_COUNT],
}

use std::fmt;

impl fmt::Debug for State {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        println!("State:");
        println!("\t pc = {:?}",self.pc as *const u8);
        println!("\t sp = {:?}",self.sp as *const u8);
        println!("\t ra = {:?}",self.ra as *const u8);
        for (ind,&val) in self.regs.iter().enumerate() {
            println!("R[{:2}] = {:-20?} {:-20}",ind,val as *const u8,val)
        }
        write!(f,"")
    }
}