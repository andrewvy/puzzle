use std::path;
use std::rc::Rc;
use std::f32;
use std::time::Instant;

use ggez;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Color, DrawParam, Point2, Rect, TextCached, TextFragment};
use ggez::timer;
use ggez::{Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, Join};

use assets::Assets;
use components;
use entities;
use gui::GuiManager;
use input::{ControllerState, InputBinding};
use resources;
use screen::Screen;
use state::Store;
use systems;
use tilemap::{SpriteLayer, TileMap};
use world::World;

pub struct AppState<'a> {
    assets: Assets,
    gui_manager: GuiManager,
    input_binding: InputBinding,
    screen: Screen,
    store: Rc<Store>,
    world: World,
    sprite_layers: Vec<SpriteLayer>,
    dispatcher: Dispatcher<'a, 'a>,
}

impl<'a> AppState<'a> {
    pub fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<AppState<'a>> {
        let screen = Screen::new(ctx)?;
        let mut assets = Assets::new(resource_dir, ctx, &screen)?;
        let input_binding = InputBinding::new();
        let controller_state = ControllerState::new();
        let mut world = World::new();
        let gui_manager = GuiManager::new();
        let store = Store::new();

        let bg_tilemap = TileMap::new(
            "/images/grass-map.png",
            screen,
            &mut assets.asset_store,
            ctx,
            32,
        );

        let entity_tilemap = TileMap::new(
            "/images/grass-map.png",
            screen,
            &mut assets.asset_store,
            ctx,
            32,
        );

        let background_layer = SpriteLayer::new(bg_tilemap.clone());
        let entity_layer = SpriteLayer::new(entity_tilemap.clone());
        let sprite_layers = vec![background_layer, entity_layer];

        let entity_map = resources::EntityMap::new();
        let mut background_map = resources::BackgroundMap::new();

        background_map.generate();

        world.specs_world.add_resource(entity_map);
        world.specs_world.add_resource(background_map);
        world.specs_world.add_resource(controller_state);

        entities::create_player(&mut world, 3, 3);

        let dispatcher = DispatcherBuilder::new()
            .with(systems::Plantae { ticks: 0 }, "plantae", &[])
            .with(systems::PlayerMovement { }, "PlayerMovement", &[])
            .with(systems::ProcessMovement { }, "ProcessMovement", &["PlayerMovement"])
            .with(systems::ProcessAnimation { }, "ProcessAnimation", &[])
            .build();

        Ok(AppState {
            assets,
            gui_manager,
            input_binding,
            screen,
            store,
            world,
            sprite_layers,
            dispatcher,
        })
    }
}

impl<'a> event::EventHandler for AppState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.dispatcher.dispatch(&self.world.specs_world.res);

            {
                let mut controller_state = self.world.specs_world.write_resource::<ControllerState>();
                controller_state.update();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0))?;

        let now = Instant::now();

        let background_map = self.world
            .specs_world
            .read_resource::<resources::BackgroundMap>();

        let entity_map = self.world
            .specs_world
            .read_resource::<resources::EntityMap>();

        for ((x, y), tile) in background_map.tiles.iter() {
            if let Some(layer) = self.sprite_layers.get_mut(tile.sprite_layer as usize) {
                layer.add(tile, *x, *y, 0.0, 0.0);
            }
        }

        for ((x, y), tile) in entity_map.tiles.iter() {
            if let Some(layer) = self.sprite_layers.get_mut(tile.sprite_layer as usize) {
                layer.add(tile, *x, *y, 0.0, 0.0);
            }
        }

        let positions = self.world.specs_world.read_storage::<components::Position>();
        let sprites = self.world.specs_world.read_storage::<components::Sprite>();
        let animations = self.world.specs_world.read_storage::<components::Animation>();

        for (position, sprite, animation) in (&positions, &sprites, &animations).join() {
            if let Some(layer) = self.sprite_layers.get_mut(sprite.tile.sprite_layer as usize) {
                let position_events = animation.animation_queue.iter().filter_map(|e|
                    match e {
                        components::AnimationEvent::Position(event) => Some(event),
                    }
                );

                let (combined_offset_x, combined_offset_y) = position_events.fold((0.0, 0.0), |(acc_x, acc_y), event|
                    if now >= event.start && now < event.end {
                        let (offset_x, offset_y) = event.offset;
                        let duration_total = event.end - event.start;
                        let duration_completed = event.end - now;
                        let percentage_completed = ggez::timer::duration_to_f64(duration_completed) / ggez::timer::duration_to_f64(duration_total);

                        (
                            acc_x + offset_x as f32 * percentage_completed as f32,
                            acc_y + offset_y as f32 * percentage_completed as f32
                        )
                    } else {
                        (acc_x, acc_y)
                    }
                );

                layer.add(&sprite.tile, position.x, position.y, combined_offset_x, combined_offset_y);
            }
        }

        for layer in self.sprite_layers.iter_mut() {
            let draw_param = DrawParam {
                src: Rect::new(
                    0.0,
                    0.0,
                    self.screen.screen_w as f32,
                    self.screen.screen_h as f32,
                ),
                dest: Point2::new(0.0, 0.0),
                scale: Point2::new(1.0, 1.0),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                ..Default::default()
            };

            graphics::draw_ex(ctx, &layer.batch, draw_param)?;
            layer.clear();
        }

        let fps = timer::get_fps(ctx);
        let fps_display = TextCached::new(TextFragment {
            text: format!("FPS: {}", fps),
            font_id: Some(self.assets.font.clone().into()),
            scale: Some(self.assets.default_scale),
            ..Default::default()
        })?;

        fps_display.queue(
            ctx,
            self.screen.to_screen_coordinates(Point2::new(5.0, 0.0)),
            None,
        );

        let logo = TextCached::new(TextFragment {
            text: format!(""),
            font_id: Some(self.assets.font.clone().into()),
            scale: Some(self.assets.default_scale),
            ..Default::default()
        })?;

        let position = Point2::new(
            (self.screen.logical_w as f32 / 2.0) * self.screen.scale_w - (logo.width(ctx) as f32 / 2.0),
            (self.screen.logical_h as f32 - 25.0) * self.screen.scale_h
        );

        logo.queue(
            ctx,
            position,
            None,
        );

        TextCached::draw_queued(ctx, DrawParam::default())?;

        self.gui_manager.render(ctx)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(button) = self.input_binding.resolve(keycode) {
            let mut controller_state = self.world.specs_world.write_resource::<ControllerState>();
            controller_state.button_down(button);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(button) = self.input_binding.resolve(keycode) {
            let mut controller_state = self.world.specs_world.write_resource::<ControllerState>();
            controller_state.button_up(button);
        }
    }
}
