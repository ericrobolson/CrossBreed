struct NetworkInputs {}

pub type GameTick = usize;

pub struct Input {
    pub game_tick: GameTick,
}

enum RunType {
    RunWhenActive,
    AlwaysRun,
}

pub struct Context {
    run_type: RunType,
}

pub struct ContextManager {
    context_stack: Vec<Context>, //TODO: implement a stack of all context types.

                                 // Base context is main menu. Sim context would be the actual game itself.
                                 // Note: if real time game, game sim must always run if it's on the stack.
                                 // Each 'context' signals the following when it returns: Stay in context, Exit context, Push context X onto stack
                                 // Each context takes the inputs, and returns a new game state.
                                 // When network code comes in, must grab all inputs from last confirmed frame, then rerun the simulation until it gets to the current frame.
}

impl ContextManager {
    pub fn new() -> Self {
        let mut mgr = ContextManager {
            context_stack: vec![],
        };

        mgr.init_main_menu();

        return mgr;
    }

    fn init_main_menu(&mut self) {
        let main = Context {
            run_type: RunType::RunWhenActive,
        };
        self.context_stack.push(main);
    }

    pub fn tick(&self, inputs: Vec<Input>) -> Self {
        //TODO: do stuff with inputs and run the different contexts, inputting no input if blank
        return Self::new();
    }

    pub fn has_active_context(&self) -> bool {
        return !self.context_stack.is_empty();
    }
}
