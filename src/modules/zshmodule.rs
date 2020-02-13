use super::{ModuleOperation, Module, ModuleResult};

pub struct ZshModule {
    links : Vec<(String, String)>,
    deps  : Vec<String>,
}

impl ZshModule {
    pub fn new() -> ZshModule {
        ZshModule { links: vec![], deps: vec![] }
    }
}

impl Module for ZshModule {
    fn name(&self) -> String {
        return String::from("ZSH");
    }

    fn operation(&self, op : ModuleOperation) -> ModuleResult {
        return ModuleResult { operation: op, module_name: self.name(), result: Ok(()) };
    }
}
