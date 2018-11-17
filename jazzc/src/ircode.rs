use jazz::opcodes::Instruction;
use std::collections::HashMap;


pub const MAX_REGISTERS: usize = 256;


#[derive(Clone)]
pub struct FunctionBuilder {
    pub list: Vec<Instruction>,
    pub label_true: Vec<usize>,
    pub label_false: Vec<usize>,
    pub label_counter: usize,
    pub maxtemps: usize,
    pub ntemps: usize,
    pub nlocals: usize,
    pub locals: HashMap<String,usize>,
    pub state: [bool;MAX_REGISTERS],
    pub skipclear: [bool;MAX_REGISTERS],
    pub registers: Vec<usize>,
    pub context: Vec<Vec<bool>>,
}

impl FunctionBuilder {
    pub fn new(nlocals: usize) -> FunctionBuilder {
        let mut state = [false;MAX_REGISTERS];
        state[0] = true;
        let mut i = 0;
        while i < nlocals {
            i += 1;
            state[i] = true;
        }



        FunctionBuilder {
            label_counter: 0,
            nlocals,
            ntemps: 0,
            locals: HashMap::new(),
            maxtemps: 0,
            list: Vec::new(),
            label_true: Vec::new(),
            label_false: vec![],
            registers: Vec::with_capacity(MAX_REGISTERS),
            context: vec![], 
            state,
            skipclear: [false;MAX_REGISTERS],
        }
    }

   

    pub fn new_local(&mut self,n: String,reg: usize) {
        
        self.state[reg] = true;
        self.nlocals += 1;
        self.locals.insert(n, reg);
    }

    pub fn get_local(&mut self,n: &str) -> usize {
        if self.locals.contains_key(n) {
            let r = self.locals.get(n).unwrap().clone();
            r
        } else {
            panic!("Local doesn't exists");
        }
    }

    pub fn new_label(&mut self) -> usize {
        self.label_counter += 1;
        self.label_counter.clone()
    }

    pub fn set_label_true(&mut self,nlabel: usize) {
        self.label_true.push(nlabel);
    }

    pub fn set_label_false(&mut self,nlabel: usize) {
        self.label_false.push(nlabel)
    }

    pub fn unset_label_true(&mut self) {
        self.label_true.pop().unwrap_or_default();
    }

    pub fn push_op(&mut self,ins: Instruction) {
        self.list.push(ins);
    }

    pub fn unset_label_false(&mut self) {
        self.label_false.pop().unwrap_or_default();
    }

    pub fn get_label_true(&self) -> usize {
        let n = self.label_true.len();
        let v = self.label_true[n - 1];
        return v;
    }

    pub fn get_label_false(&self) -> usize {
        let n = self.label_false.len();
        let v = self.label_false[n - 1];
        return v;
    }

    pub fn register_new(&mut self) -> usize {
        for i in 0..MAX_REGISTERS {
            if self.state[i] == false {
                self.state[i] = true;
                return i;
            }
        }
        println!("No registers available");
        return 0;
    }

    pub fn register_push(&mut self,nreg: usize) -> usize {
        self.registers.push(nreg);
        if self.register_is_temp(nreg) {
            self.ntemps += 1;
        }
        return nreg;
    }

    pub fn register_first_temp_available(&mut self) -> usize {
        for i in 0..MAX_REGISTERS {
            if self.state[i] == false {
                return i;
            }
        }
        return 0;
    }



    pub fn register_push_temp(&mut self) -> usize {
        let value = self.register_new();
        self.registers.push(value);
        if value > self.maxtemps {
            self.maxtemps = value;
            self.ntemps += 1;
        }

        return value;
    }


    pub fn get_insts(&mut self) -> Vec<Instruction> {
        self.list.clone()
    }

    pub fn register_pop_context_protect(&mut self,protect: bool) -> usize {
        if self.registers.len() == 0 {
            panic!("REGISTER ERROR");
        }

        let value = self.registers.pop().unwrap_or_default();

        if protect {
            self.state[value] = true;
        } else if value > self.nlocals {
            self.state[value] = false;
        }

        if protect && value >= self.nlocals {
            let ctx = self.context.last_mut().unwrap();
            ctx[value] = true;
        }

        return value;
    }

    pub fn int_const(&mut self,int: i32) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadInt(register,int));
        return register;
    }

    pub fn float_const(&mut self,float: f32) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadFloat(register,float));
        return register;
    }

    pub fn register_pop(&mut self) -> usize {
        self.register_pop_context_protect(false)
    }
    pub fn register_is_temp(&self,nreg: usize) -> bool {
        return nreg >= self.nlocals;
    }
}
