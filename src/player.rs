use macroquad::{
    experimental::{
        animation::{AnimatedSprite, Animation},
        collections::storage,
        scene::{self, RefMut},
        state_machine::{State, StateMachine},
    },
    color,
    prelude::*,
};

use crate::resources::Resources;

pub struct Player{
    texture: Texture2D,
    pos: Vec2,
    rotation : f32
}

impl Player{
    pub fn new(pos: Vec2) -> Self{
        let resources = storage::get::<Resources>();
        let texture = resources.hero;
        Self {texture, pos, rotation: 0.}
    }

    fn add_rotation(&mut self){
        if self.rotation >= 360.{
            self.rotation = 0.;
        } else {
            self.rotation += 0.1;
        }
    }
}

impl scene::Node for Player {
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
