extern crate utilities;

use utilities::CustomMaterial;

use bevy::prelude::*;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<CustomMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2 { x: 1.0, y: 1.0 }, ..default() })),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		material: materials.add(CustomMaterial {
			color: Color::default(),
			color_texture: Some(asset_server.load("branding/icon.png")),
			color_uv: Vec2 { x: 2.0, y: 2.0 },
			alpha_mode: AlphaMode::Blend,
		}),
		..default()
	});

	//3D camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 0.0, 5.0),
		..default()
	});
}

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
		.add_systems(Startup, setup)
		.run();
}