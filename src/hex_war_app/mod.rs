pub mod cursor;
pub mod main_menu;
mod ortho_camera;
pub mod rendering;
pub mod state;
pub mod update_timer;

use crate::app::{App, ELWT};
use crate::graphics::backend::vulkan::VkGraphics;
use crate::graphics::camera::Camera;
use crate::graphics::error::LoadError;
use crate::graphics::geometry::Geometry;
use crate::graphics::proxy::geometry_manager::GeometryManager;
use crate::graphics::proxy::texture_manager::TextureManager;
use crate::graphics::scene::Scene;
use crate::graphics::texture::Texture;
use crate::graphics::Graphics;
use crate::hex_war_app::cursor::Cursor;
use crate::hex_war_app::ortho_camera::OrthographicCamera;
use crate::hex_war_app::update_timer::UpdateTimer;
use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use main_menu::MainMenu;
use slog::Logger;
use state::State;
use winit::event::{ElementState, Event, MouseButton, StartCause, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowId};

pub struct ResourceManagers {
    pub tex_manager: TextureManager,
    pub geom_manager: GeometryManager,
}

#[derive(Debug, Clone)]
struct Cameras {
    pub ui: OrthographicCamera,
}

struct Scenes {
    pub ui: Scene,
    pub game: Scene,
}

pub struct HexWarApp {
    window: Window,
    logger: Logger,
    state: State,
    cursor: Cursor,
    update_timer: UpdateTimer,
    cameras: Cameras,
    graphics: Graphics,
    scenes: Scenes,
}

impl HexWarApp {
    pub fn new(window: Window, logger: Logger) -> Self {
        let graphics_backend = VkGraphics::new();
        let graphics = Graphics::new(Box::new(graphics_backend));
        let resource_managers = ResourceManagers {
            tex_manager: graphics.get_texture_manager(),
            geom_manager: graphics.get_geometry_manager(),
        };

        let unit_quad = Geometry::new(
            "geometries/unit_quad.dae".into(),
            graphics.get_geometry_manager(),
        )
        .expect("Can't load unit quad geometry.");

        let cursor = Cursor::new(resource_managers);

        let update_timer = UpdateTimer::new(60);

        let cameras = Cameras {
            ui: OrthographicCamera::new(WorldCoords::zero(), WorldCoords::zero()), // TODO: Think about coordinates
        };

        let scenes = Scenes {
            ui: Scene::new(graphics.get_scene_manager(), graphics.get_texture_manager()),
            game: Scene::new(graphics.get_scene_manager(), graphics.get_texture_manager()),
        };

        trace!(logger, "HexWarApp initialized");
        Self {
            window,
            logger: logger.clone(),
            state: State::Menu(MainMenu::new(logger)),
            cursor,
            update_timer,
            cameras,
            graphics,
            scenes,
        }
    }

    fn load_cursor_resources(
        tex_manager: TextureManager,
        unit_quad: Geometry,
    ) -> Result<cursor::Resources, LoadError> {
        let released = Texture::new("textures/cursor_released.ktx".into(), tex_manager.clone())?;
        let pressed = Texture::new("textures/cursor_pressed.ktx".into(), tex_manager)?;
        Ok(cursor::Resources {
            pressed,
            released,
            unit_quad,
        })
    }

    pub fn close_requested(&mut self) {
        trace!(self.logger, "HexWarApp close requested.");
        self.state = State::Finished
    }

    pub fn cursor_moved(&mut self, new_pos: ScreenCoords) {
        trace!(self.logger, "HexWarApp cursor moved: {:?}.", new_pos);
        let world_pos = self
            .cameras
            .ui
            .to_world(new_pos, 0f32, self.get_screen_size());
        self.cursor.move_to(world_pos);
    }

    fn get_screen_size(&self) -> ScreenCoords {
        ScreenCoords::new(
            self.window.inner_size().width as i64,
            self.window.inner_size().height as i64,
        )
    }

    pub fn mouse_button_used(&mut self, button: MouseButton, state: ElementState) {
        trace!(
            self.logger,
            "HexWarApp mouse button pressed: ({:?},{:?}).",
            button,
            state
        );
    }

    pub fn is_time_to_update(&self) -> bool {
        self.update_timer.is_time_to_update()
    }

    pub fn update(&mut self) {
        trace!(self.logger, "HexWarApp updating.");
        self.update_timer.update();
        self.window.request_redraw();
    }

    pub fn is_finished(&self) -> bool {
        if let State::Finished = self.state {
            trace!(self.logger, "HexWarApp finished.");
            return true;
        }
        false
    }

    pub fn window_resized(&mut self, new_size: ScreenCoords) {
        trace!(self.logger, "HexWarApp window resized.");
    }

    pub fn window_destroyed(&mut self) {
        trace!(self.logger, "HexWarApp window destroyed.");
    }

    pub fn draw(&mut self) {
        trace!(self.logger, "HexWarApp draw.");
        self.fill_scenes();
        // let ui_image = self.render_ui();
    }

    fn fill_scenes(&mut self) {
        // self.cursor.add_to_scene(&mut self.scenes.ui)
    }

    // fn render_ui(&mut self) -> Texture {
    //     self.scenes.ui.render(&self.cameras.ui, &mut self.graphics)
    // }
}

impl App for HexWarApp {
    type UserEvent = ();

    fn process_event(
        &mut self,
        event: Event<Self::UserEvent>,
        _wt: &ELWT<Self::UserEvent>,
    ) -> ControlFlow {
        let mut control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(cause) => {
                if let StartCause::Poll = cause {
                    if self.is_time_to_update() {
                        self.update();
                    }
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    let new_size = (new_size.width as i64, new_size.height as i64);
                    self.window_resized(new_size.into());
                }
                WindowEvent::Moved(_) => {}
                WindowEvent::CloseRequested => self.close_requested(),
                WindowEvent::Destroyed => self.window_destroyed(),
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::ReceivedCharacter(_) => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::KeyboardInput { .. } => {}
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::CursorMoved { position, .. } => {
                    let new_pos = (position.x as i64, position.y as i64).into();
                    self.cursor_moved(new_pos)
                }
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::MouseInput { button, state, .. } => {
                    self.mouse_button_used(button, state)
                }
                WindowEvent::TouchpadPressure { .. } => {}
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::ScaleFactorChanged { .. } => {}
                WindowEvent::ThemeChanged(_) => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => self.draw(),
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        };

        if self.is_finished() {
            control_flow = ControlFlow::Exit;
        }

        control_flow
    }

    fn draw(&mut self, window_id: WindowId) {
        self.draw()
    }
}
