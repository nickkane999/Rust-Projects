mod lib;
use lib::Draw;

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options {:?}", self.options);
    }
}

pub fn run() {
    let component1 = SelectBox {
        width: 100,
        height: 100,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    };
    let component2 = lib::Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };

    let mut screen = lib::Screen { components: vec![] };

    screen.add_component(component1);
    screen.add_component(component2);
    screen.start();
}