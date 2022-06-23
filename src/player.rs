use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::convert::From;

use crate::{
	LEVEL_LEN,
	WIN_W,
	WIN_H,
	TILE_SIZE,
	ANIM_TIME,
	ACCEL_RATE,
	PLAYER_SPEED,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
	level::Background,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Deref, DerefMut)]
pub struct PlayerSheet(Handle<TextureAtlas>);

impl Velocity {
	fn new() -> Self {
		Self(Vec2::splat(0.))
	}
}

impl From<Vec2> for Velocity {
	fn from(velocity: Vec2) -> Self {
		Self(velocity)
	}
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_player_sheet)
			.add_enter_system(GameState::Playing, spawn_player)
			.add_system(move_player.run_in_state(GameState::Playing).label("move_player"))
			.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					.after("move_player")
					.with_system(animate_player)
					.with_system(move_camera)
					.into()
			);
	}
}

fn load_player_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let player_handle = asset_server.load("walking.png");
	loading_assets.insert(
		player_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(player_handle.clone_untyped(), &asset_server),
	);

	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(TILE_SIZE), 4, 1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	
	commands.insert_resource(PlayerSheet(player_atlas_handle));
}

fn spawn_player(
	mut commands: Commands,
	player_sheet: Res<PlayerSheet>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(0., -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		.insert(Velocity::new())
		.insert(Player);
}

fn move_player(
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
){
	let (mut transform, mut velocity) = player.single_mut();

	let mut deltav = Vec2::splat(0.);

	if input.pressed(KeyCode::A) {
		deltav.x -= 1.;
	}

	if input.pressed(KeyCode::D) {
		deltav.x += 1.;
	}

	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;

	// ** needed to dereference the borrow (type &mut Velocity), 
	// and then access the contained valued (via derived Deref)
	**velocity = if deltav.length() > 0. {
		(**velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
	}
	else if velocity.length() > acc {
		**velocity + (velocity.normalize_or_zero() * -acc)
	}
	else {
		Vec2::splat(0.)
	};
	let change = **velocity * deltat;

	let new_pos = transform.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);
	if new_pos.x >= -(WIN_W/2.) + TILE_SIZE/2.
		&& new_pos.x <= LEVEL_LEN - (WIN_W/2. + TILE_SIZE/2.)
	{
		transform.translation = new_pos;
	}

	let new_pos = transform.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
	if new_pos.y >= -(WIN_H/2.) + (TILE_SIZE * 1.5)
		&& new_pos.y <= WIN_H/2. - TILE_SIZE/2.
	{
		transform.translation = new_pos;
	}
}

fn animate_player(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut player: Query<
		(
			&Velocity,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
		),
		With<Player>
	>,
){
	let (velocity, mut sprite, texture_atlas_handle, mut timer) = player.single_mut();
	if velocity.cmpne(Vec2::ZERO).any() {
		timer.tick(time.delta());

		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

fn move_camera(
	player: Query<&Transform, With<Player>>,
	mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
){
	let pt = player.single();
	let mut ct = camera.single_mut();

	ct.translation.x = pt.translation.x.clamp(0., LEVEL_LEN - WIN_W);
}
