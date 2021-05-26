use macroquad::prelude::*;
use crate::pub_struct;


pub_struct!( Resources {
    gold: Texture2D,
    hero: Texture2D,
});
// async fn load_t(path: &str) -> Result<Texture2D, macroquad::prelude::FileError> {
//     let texture = load_texture(path).await?;
//     texture.set_filter(FilterMode::Nearest);
//     texture
// }

impl Resources{
    pub async  fn new() -> Result<Self, macroquad::prelude::FileError> {
        let gold = load_texture("assets/gold.png").await?;
        let hero = load_texture("assets/hero.png").await?;
        
        Ok(Self{
            gold, hero, 
        })
    }
}

