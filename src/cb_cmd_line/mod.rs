use std::boxed;
use std::io::{stdin, stdout, Write};

pub struct CbCmdMenu {
    menu_level: usize,
    display_text: &'static str,
    child: Box<Option<CbCmdMenu>>,
    options: Vec<CbCmdMenuOption>,
}

pub struct CbCmdMenuOption {
    id: usize,
    display_text: &'static str,
}

impl CbCmdMenu {
    pub fn root(header: &'static str, options_text: Vec<&'static str>) -> Self {
        let mut menu = CbCmdMenu::new(header, options_text, 0);

        return menu;
    }

    fn new(header: &'static str, options_text: Vec<&'static str>, menu_level: usize) -> Self {
        let mut options = vec![];

        let mut option_id = 0;

        options_text.iter().for_each(|value| {
            option_id += 1;
            options.push(CbCmdMenuOption {
                id: option_id,
                display_text: value,
            });
        });

        return Self {
            menu_level: menu_level,
            display_text: header,
            child: Box::new(None),
            options: options,
        };
    }

    pub fn print(&self) {
        let prefix = "";
        println!("{}{}", prefix, self.display_text);
        self.options.iter().for_each(|option| {
            println!(
                "{}# {}: {}",
                prefix,
                option.id,
                option.display_text.to_string()
            )
        });
    }

    pub fn get_menu_choice(&self) -> String {
        let mut s = String::new();
        print!("-> : ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        return s;
    }
}
