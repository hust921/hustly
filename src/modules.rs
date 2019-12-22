pub mod modules {
    pub struct ModuleCollection {
        names : Vec<Module>,
    }

    impl ModuleCollection {
        pub fn new() -> ModuleCollection {
            ModuleCollection {
                names: vec![]
            } 
        }

        pub fn add(&self, module : Module) {
            self.names.append(module);
        }

        pub fn remove(&self, module : Module) {
            self.names.remove_item(module);
        }

        pub fn get_module(&self, name : String) -> Option<Module> {
            for m in self.names.into_iter() {
                if m.name == name {
                    return Some(m);
                }
            }
            return None;
        }
    }

    pub struct Module {
        pub name : String,
        pub deps : Vec<String>,
    }

    pub trait Operations {
        fn install() -> bool {
            return false;
        }

        fn uninstall() -> bool {
            return false;
        }

        fn check() -> bool {
            return false;
        }

        fn test() -> bool {
            return false;
        }
    }

    pub fn add_module(module : Module) {
    }
}
