pub trait Command {
    fn execute(&self);
}

pub struct MacroCommand {
    stack: Vec<Box<Command>>,
}

impl MacroCommand {
    pub fn new() -> Self {
        return Self { stack: vec![] };
    }

    pub fn append(&mut self, cmd: Box<Command>) {
        self.stack.push(cmd);
    }

    pub fn undo(&mut self) {
        self.stack.pop();
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

impl Command for MacroCommand {
    fn execute(&self) {
        for command in &self.stack {
            command.execute();
        }
    }
}

/*
Example:


fn main() {
    let mut history = MacroCommand::new();
    let canvas = Box::new(DrawCanvas::new());
    // TODO
    let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
    let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));
    history.append(cmd1);
    history.append(cmd2);
    println!("----------");
    history.execute();
    println!();
    println!("---undo---");
    history.undo();
    history.execute();
    println!();
    println!("---clear---");
    history.clear();
    history.execute();
}

*/
