use bevy::{prelude::*, render::render_resource::{SamplerDescriptor, AddressMode}};

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0, -1.),
			texture: asset_server.load("branding/icon.png"),
			sprite: Sprite {
				rect: Some(Rect {
					min: Vec2 { x: 0.0, y: 0.0 },
					max: Vec2 { x: 512.0, y: 512.0 },
				}),
				..default()
			},
			..default()
		},
	));

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(0.0, 0.0, 5.0),
		..default()
	});
}

fn main() {
	App::new()
		.add_plugins(
			DefaultPlugins
				.set(ImagePlugin {
					default_sampler: SamplerDescriptor {
						address_mode_u: AddressMode::Repeat,
						address_mode_v: AddressMode::Repeat,
						..default()
					}
				})
		)
		.add_systems(Startup, setup)
		.run();
}