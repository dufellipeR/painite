use crate::ides::{IDE, Entries};

pub struct Goland {
    entries: Entries,
}

impl IDE for Goland {

    fn get_name(&self) -> &String {
        &self.entries.name
    }

    fn get_comment(&self) -> &String {
        &self.entries.comment
    }

    fn get_version(&self) -> &String {
        &self.entries.version
    }

    fn get_short_name(&self) -> &String {
        &self.entries.short_name
    }

    fn get_exec(&self) -> &String {
        &self.entries.exec
    }

    fn get_icon(&self) -> &String {
        &self.entries.icon
    }

    fn get_entries(&self) -> &Entries {
        &self.entries
    }

    fn set_version(&mut self, version: String) {
        self.entries.version = version
    }

    fn set_icon(&mut self, icon_path: String) {
        self.entries.icon = icon_path;
    }

    fn set_exec(&mut self, exec_path: String) {
        self.entries.exec = exec_path
    }
}

impl Goland {
    pub fn new() -> Self {
        Goland{ entries: Entries {
            name: "GoLand".to_string(),
            comment: "The complete IDE crafted for Gophers".to_string(),
            icon: "".to_string(),
            exec: "".to_string(),
            version: "".to_string(),
            short_name: "goland".to_string(),
        }
        }
    }
}