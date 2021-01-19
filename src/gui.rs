use widget::Widget;
use sfml::window::Event;
use std::collections::HashMap;


pub struct Gui {
    //List of widgets to be prompt into screen
    pub components: HashMap<&str, Widget>,
    //List of possible texture names loaded into the GUI handler; should throw an error if using an unregistred texture
    pub textures: Vec<String>
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            components: Vec::new(),
            textures: Vec::new()
        }
    }
    pub fn register_texture(&mut self, texture_name: String) {
        self.textures.push(texture_name);
    }
    pub fn handle_events(&mut self, event: Event) {
        for (_, component) in &self.components {
            //REVISE
            match component.events.get(event) {
                Some(func) => func(),
                None => {}
            }
        }
    }
}
