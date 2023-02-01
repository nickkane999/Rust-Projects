use std::any::Any;
use core::any::TypeId;

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
pub struct Screen {
    pub components: Vec<Box<dyn Any>>,
}

impl Screen {
    pub fn add_component<T>(&mut self, component: T)
    where
        T: 'static + Any,
    {
        self.components.push(Box::new(component));
    }

    pub fn start(&self) {
        for component in self.components.iter() {
            /*
            let drawable = component.downcast_ref::<&dyn Draw>();
            println!("Component has a draw method {}", drawable.is_some());
            /*
            if let Some(drawable) = component.downcast_ref::<&dyn Draw>() {
                drawable.draw();
            } else {
                println!("Component does not have a draw method {}", component.downcast_ref::<&dyn Draw>());
            }
            */
            if let Some(drawable) = drawable {
                drawable.draw();
            } else {
                println!("Component is not a type with the draw method and is of type: {:?}", component);
            }
            println!("Component has a draw method: {}", drawable.is_some());
            println!("Debug representation of component: {:?}", component);
            println!("Memory address of component: {:p}", component);
            println!("Size of component: {}", std::mem::size_of_val(component));

            println!("My component ID: {:?}", component.type_id());
            let temp = component.downcast_ref::<&Box<dyn Any>>();
            println!("Is it right: {:?}", temp);
            match component.type_id() {
                // match against a specific type and access its attributes/values
                type_id if type_id == TypeId::of::<&Box<SelectBox>>() => {
                    if let Some(select_box) = component.downcast_ref::<&SelectBox>() {
                        println!("SelectBox: width: {}, height: {}, options: {:?}", select_box.width, select_box.height, select_box.options);
                    }
                },
                type_id if type_id == TypeId::of::<&Box<Button>>() => {
                    if let Some(button) = component.downcast_ref::<&Button>() {
                        println!("Button: width: {}, height: {}, label: {}", button.width, button.height, button.label);
                    }
                },
                // handle other types
                _ => println!("Component is of unknown type"),
            }
            */
            println!("My component ID: {:?}", component.type_id());
            let component1 = Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            });
            println!("Id Box<SelectBox> {:?}", component1.type_id());
            //let temp = component.downcast_ref::<Box<SelectBox>>();
            //println!("Is it right: {:?}", temp);
            //component; // This is a Box<dyn Any>
            match component.downcast_ref::<&Box<dyn Any>>() {
                Some(selectbox) => {
                    println!("Found a generic object component with options {:?}", selectbox);
                }
                None => (),
            }
            match component.downcast_ref::<&SelectBox>() {
                Some(selectbox) => {
                    println!("Found a SelectBox component with options {:?}", selectbox.options);
                }
                None => (),
            }
        
            match component.downcast_ref::<&Button>() {
                Some(button) => {
                    println!("Found a Button component with label {}", button.label);
                }
                None => (),
            }

        }
        println!("Id Box<SelectBox> {:?}", TypeId::of::<&Box<SelectBox>>());
        println!("Id Box<Button> {:?}", TypeId::of::<&Box<Button>>());
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label {}", self.label);
    }
}
