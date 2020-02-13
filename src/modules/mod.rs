pub use self::zshmodule::ZshModule;
mod zshmodule;

pub trait Module {
    fn operation(&self, _ : ModuleOperation) -> ModuleResult;
    fn name(&self) -> String;
}

#[derive(Copy, Clone)]
pub enum ModuleOperation {
    Install,
    Uninstall,
    Update,
    Check,
    Test,
}

pub struct ModuleResult {
    operation : ModuleOperation,
    module_name : String,
    result : Result<(), String>,
}

pub struct ModuleCollection {
    pub modules : Vec<Box<dyn Module>>,
}

impl ModuleCollection {
    pub fn operation(&self, operation : ModuleOperation) -> Vec<ModuleResult> {
        let mut results = vec![];
        for module in self.modules.iter() {
            results.push(module.operation(operation));
        }
        results
    }
}
