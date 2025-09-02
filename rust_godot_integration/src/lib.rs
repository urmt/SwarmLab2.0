use gdnative::prelude::*;
use weave_lang::interpreter::Interpreter;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct WeaveLangNode {
    interpreter: Interpreter,
}

#[methods]
impl WeaveLangNode {
    fn new(_owner: &Node) -> Self {
        WeaveLangNode {
            interpreter: Interpreter::new(),
        }
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f64) {
        // Example: simulate sensing light intensity (placeholder)
        let sensed_light = 0.7; // This would come from Godot environment

        // Run interpreter cycle
        self.interpreter.tension_cycle.detect_tension(sensed_light);
        self.interpreter.tension_cycle.drift();
        self.interpreter.tension_cycle.resolve(sensed_light);
        self.interpreter.metaweave.propose_new_primitives();

        // Additional logic to update Godot nodes based on interpreter output
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<WeaveLangNode>();
}

godot_init!(init);
