extern crate utilities;

use utilities::CustomMaterial;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<CustomMaterial>>,
	mut color_materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		MaterialMesh2dBundle {
			mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2 { x: 400.0, y: 400.0 }, ..default() })).into(),
			transform: Transform::from_xyz(0.0, 200.0, 0.0),

			material: color_materials.add(ColorMaterial::from(Color::WHITE)),

			..default()
		}
	);

	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2 { x: 400.0, y: 400.0 }, ..default() })),
		transform: Transform::from_xyz(0.0, -200.0, 0.0),

		material: materials.add(CustomMaterial {
			color: Color::default(),
			color_texture: Some(asset_server.load("branding/icon.png")),
			color_uv: Vec2 { x: 2.0, y: 2.0 },
			alpha_mode: AlphaMode::Blend,
		}),

		..default()
	});

	//2D camera
	commands.spawn(Camera2dBundle {
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