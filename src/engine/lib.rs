extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

use opengl_graphics::{Filter, GlGraphics, OpenGL, TextureSettings};
use piston::window::WindowSettings;
use piston_window::PistonWindow as Window;
use piston_window::*;
use sprite::*;
pub use uuid::Uuid;

use std::rc::Rc;

mod gameobject;

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    sprites: Vec<(Uuid, Scene<Texture<gfx_device_gl::Resources>>)>, // Sprites in world
    main_window: Window, // The main game window
    objects: Vec<gameobject::GameObject>,
}

impl Game {
    pub fn run(&mut self) {
        while let Some(e) = self.main_window.next() {
            self.render(&e);
            self.update(&e);
        }
    }
    pub fn new(title: &str) -> Game {
        let opengl = OpenGL::V3_2;

        let main_window: Window = WindowSettings::new(title, [800, 800])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let gl = GlGraphics::new(opengl);

        let sprites: Vec<(Uuid, Scene<Texture<gfx_device_gl::Resources>>)> = Vec::new();

        let objects: Vec<gameobject::GameObject> = Vec::new();

        Game {
            gl,
            sprites,
            main_window,
            objects,
        }
    }

    /// Adds sprite to world
    pub fn add_sprite(
        &mut self,
        file_name: &str,
        (x, y): (f64, f64), // Position
        rotation: f64,
        size_factor: f64,
    ) -> usize {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);

        let mut scene = Scene::new();
        let mut texture_context = TextureContext {
            factory: self.main_window.factory.clone(),
            encoder: self.main_window.factory.create_command_buffer().into(),
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

        let mut sprite = Sprite::from_texture(texture.clone());

        sprite.set_position(x, y);
        sprite.set_rotation(rotation);
        sprite.set_scale(size_factor, size_factor);

        let index = scene.add_child(sprite);

        self.sprites.push((index, scene));
        self.sprites.len()
    }
    fn render(&mut self, event: &Event) {
        use graphics::*;
        let sprites = &mut self.sprites;
        self.main_window.draw_2d(event, |context, gfx, _| {
            clear([0.0, 0.0, 0.0, 0.0], gfx);
            for (_, scene) in sprites.iter_mut() {
                scene.event(event);
                scene.draw(context.transform, gfx);
            }
        });
    }

    fn update(&mut self, event: &Event) {
        if let Some(render_args) = event.render_args() {
            // Update game
        }
    }

    pub fn new_game_object(&mut self, x: f64, y: f64) -> usize {
        let object = gameobject::GameObject::new((x, y));
        self.objects.push(object);
        self.objects.len() - 1
    }

    pub fn get_game_object(&self, index: usize) -> Option<&gameobject::GameObject> {
        self.objects.get(index)
    }

    pub fn get_game_object_mut(&mut self, index: usize) -> Option<&mut gameobject::GameObject> {
        self.objects.get_mut(index)
    }
}
