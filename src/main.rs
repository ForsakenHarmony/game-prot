mod world;

use bevy::prelude::*;

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
	commands
		.spawn((Person, Name("Elaina Proctor".to_string())))
		.spawn((Person, Name("Renzo Hume".to_string())))
		.spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
	timer.0.tick(time.delta_seconds);

	if timer.0.finished {
		for (_person, name) in &mut query.iter() {
			println!("hello {}!", name.0);
		}
		timer.0.reset();
	}
}

fn main() {
	App::build()
		.add_default_plugins()
		.add_startup_system(setup.system())
		.run();
}

struct Background {
	parallax: Option<f32>,
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<Assets<Texture>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	setup_background(commands, asset_server, textures, materials);

}

fn create_ship() {

}

fn setup_background(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<Assets<Texture>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let background_tile = asset_server
		.load_sync(
			&mut textures,
			"assets/backgrounds/darkPurple.png"
		)
		.unwrap();
	let background_meteors2 = asset_server
		.load_sync(
			&mut textures,
			"assets/backgrounds/meteors_2.png"
		)
		.unwrap();
	let background_meteors = asset_server
		.load_sync(
			&mut textures,
			"assets/backgrounds/meteors.png"
		)
		.unwrap();

	commands
		.spawn(Camera2dComponents::default())
		.spawn(UiCameraComponents::default())
		.spawn(SpriteComponents {
			material: materials.add(background_tile.into()),
			..Default::default()
		})
		.with(Background {
			parallax: Some(1.5),
		})
		.spawn(SpriteComponents {
			material: materials.add(background_meteors2.into()),
			..Default::default()
		})
		.with(Background {
			parallax: Some(1.15),
		})
		.spawn(SpriteComponents {
			material: materials.add(background_meteors.into()),
			..Default::default()
		})
		.with(Background {
			parallax: None,
		});
}
