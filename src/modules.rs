pub mod modules {
    pub trait Module {
        fn operation(&self, _ : ModuleOperation) -> ModuleResult;
        fn name(&self) -> String;
    }

    #[allow(dead_code)]
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

    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub enum ModuleOperation {
        Install,
        Uninstall,
        Update,
        Check,
        Test,
    }

    #[allow(dead_code)]
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
}
