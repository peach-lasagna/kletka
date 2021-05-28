use macroquad::{
    experimental::{
        // animation::{AnimatedSprite, Animation},
        collections::storage,
        scene::{self, RefMut},
        // state_machine::{State, StateMachine},
    },
    color,
    prelude::*,
};
use crate::map::{get_cell_content};
use crate::interaction::PlayerInteraction;

use crate::resources::Resources;
use crate::structs::{  Items, Gold };


pub struct Player{
    texture:Texture2D,
    pos: Vec2,
    rotation : f32,
    // pub stats: Stats,
    pub items: Items
}


impl Player{
    pub fn new(pos: Vec2) -> Self{
        let resources = storage::get::<Resources>();
        let texture = resources.hero;
        let items = Items{ gold: Gold::new() };
        Self {texture, pos, rotation: 0., items  }
    }

    fn add_rotation(&mut self){
        if self.rotation >= 360.{
            self.rotation = 0.;
        } else {
            self.rotation += 0.1;
        }
    }
    pub fn absorb(&mut self, cell: impl PlayerInteraction){
        // let cell_obj = get_cell_content(cell);
        cell.update_player(self);
    }
}

impl scene::Node for Player { //  s/&mut  //g  
    fn draw(mut node: RefMut<Self>) {
        draw_texture_ex(
            node.texture,
            node.pos.x - node.texture.width()  / 2.,
            node.pos.y - node.texture.height() / 2.,
            color::BLACK,
            DrawTextureParams {
                rotation: node.rotation,
                ..Default::default()
            },
        )
    }
    fn update(mut node: RefMut<Self>) {
        // node.update();
        node.add_rotation();
    }
}
