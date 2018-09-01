use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};

pub struct World;
