// skin.rs
use ggez::graphics::Color;

// Defines a single skin with its name, color, and price.
pub struct Skin {
    pub name: String,
    pub color: Color,
    pub price: u32,
}

// Manages the collection of available skins.
pub struct SkinManager {
    pub skins: Vec<Skin>,
}

impl SkinManager {
    // Creates a new SkinManager and populates it with a default list of skins.
    pub fn new() -> Self {
        let skins = vec![
            Skin { name: "Green".into(), color: Color::GREEN, price: 0 },
            Skin { name: "Blue".into(), color: Color::BLUE, price: 0 },
            Skin { name: "Orange".into(), color: Color::from_rgb(255, 165, 0), price: 5 },
            Skin { name: "Purple".into(), color: Color::from_rgb(128, 0, 128), price: 10 },
            Skin { name: "Yellow".into(), color: Color::YELLOW, price: 15 },
            Skin { name: "Pink".into(), color: Color::from_rgb(255, 192, 203), price: 20 },
        ];
        Self { skins }
    }
}
