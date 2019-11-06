extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

use opengl_graphics::{Filter, TextureSettings};
use piston_window::PistonWindow as Window;
use piston_window::*;
use sprite::*;
use std::rc::Rc;
pub use uuid::Uuid;
use std::ptr::NonNull;
use std::pin::Pin;
use std::marker::PhantomPinned;

pub struct TransformComponent {
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
}

struct SpriteComponent {
    scene: sprite::Scene<piston_window::Texture<gfx_device_gl::Resources>>,
    window: Window,
    parent_transform: NonNull<TransformComponent>,
    _pin: PhantomPinned,
}

impl SpriteComponent {
    fn transform(&self) -> &TransformComponent {
        unsafe { self.parent_transform.as_ref() }
    }

    fn render(&mut self, window: &mut Window, event: &piston_window::Event) {
        window.draw_2d(event, |context, gfx, _| {
            self.scene.event(event);
            self.scene.draw(context.transform, gfx);
        });
    }

    fn update(&mut self, args: &RenderArgs) {
        if let Some(sprite) = self.scene.children().first() {
            let sprite = self.scene.child_mut(sprite.id()).unwrap();

            sprite.set_position(unsafe { self.parent_transform.as_ref().x }, unsafe { self.parent_transform.as_ref().x });
            sprite.set_rotation(unsafe { self.parent_transform.as_ref().rotation });

            let size = unsafe { self.parent_transform.as_ref().size };
            sprite.set_scale(size, size);
        }
    }
}

pub struct GameObject {
    pub transform: TransformComponent,
    sprite: Option<SpriteComponent>,
}

impl GameObject {
    pub fn new((x, y): (f64, f64)) -> Pin<Box<GameObject>> {
        Box::pin(GameObject {
            transform: TransformComponent {
                x,
                y,
                size: 1.0,
                rotation: 0.0,
            },
            sprite: None,
        })
    }

    pub fn add_sprite(pin: &mut Pin<Box<Self>>, file_name: &str, window: Window) {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);

        let mut factory = window.factory.clone();
        let encoder = factory.create_command_buffer().into();

        let transform = NonNull::from(&pin.transform);
        unsafe {
            let mut_ref = Pin::as_mut(pin);
            Pin::get_unchecked_mut(mut_ref).sprite = Option::from(SpriteComponent {
                scene: Scene::new(),
                window,
                parent_transform: transform,
                _pin: PhantomPinned,
            });
        }

        let mut texture_context = TextureContext {
            factory: factory,
            encoder: encoder,
        };

        let texture = Rc::new(
            Texture::from_path(
                &mut texture_context,
                find_folder::Search::ParentsThenKids(3, 3)
                    .for_folder("assets")
                    .unwrap()
                    .join(file_name),
                Flip::None,
                &texture_settings,
            )
            .unwrap(),
        );

        let sprite = Sprite::from_texture(texture);

        let mut_ref = pin.as_mut();
        let spr = unsafe { &mut Pin::get_unchecked_mut(mut_ref).sprite };
        spr.as_mut().unwrap().scene.add_child(sprite);
    }
}
