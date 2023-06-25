
use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use rand::prelude::*;
use bevy::app::AppExit;

pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const SOUND_ENABLED: bool = false;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const NUMBER_OF_STARS: usize = 10;
pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_player) 
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_hit_player)
        .add_system(enemy_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(player_collide_with_stars)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}

pub fn spawn_camera(mut commands: Commands, 
                    window_query: Query<&Window, With<PrimaryWindow>>) 
{
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
 
pub struct GameOver {
    pub score: u32
}

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}
 
#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)}
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(mut commands: Commands, 
                    window_query: Query<&Window, With<PrimaryWindow>>, 
                    asset_server: Res<AssetServer>) 
{
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0), 
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(mut commands: Commands,
                     window_query: Query<&Window, With<PrimaryWindow>>, 
                     asset_server: Res<AssetServer>) 
{
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"), 
                ..default()
            },
            Enemy { 
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(mut commands: Commands, 
                   window_query: Query<&Window, With<PrimaryWindow>>,
                   asset_server: Res<AssetServer>)
{
    let window = window_query.get_single().unwrap();
    
    for _ in 0..NUMBER_OF_STARS {
        spawn_star(&mut commands, window, &asset_server);
    }
}

pub fn spawn_star(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {

    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0), 
            texture: asset_server.load("sprites/star.png"),
            ..default()
        }, 
        Star {}
    ));
}

pub fn player_movement(keyboard_input: Res<Input<KeyCode>>, 
                       mut player_query: Query<&mut Transform, With<Player>>,
                       time: Res<Time>) 
{
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn clamp(val: f32, lower: f32, upper: f32) -> f32 {
    let min = if val < lower { lower } else { val };
    if min > upper { upper } else { min }
}

pub fn confine_enemy_movement(mut enemy_query: Query<&mut Transform, With<Enemy>>, 
                              window_query: Query<&Window, With<PrimaryWindow>>) 
{
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        translation.y = clamp(translation.y, y_min, y_max);
        translation.x = clamp(translation.x, x_min, x_max);
        transform.translation = translation;
    }
}

pub fn confine_player_movement(mut player_query: Query<&mut Transform, With<Player>>, 
                               window_query: Query<&Window, With<PrimaryWindow>>)
{
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;
        translation.y = clamp(translation.y, y_min, y_max);
        translation.x = clamp(translation.x, x_min, x_max);

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, 
                      time: Res<Time>) 
{
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(mut enemy_query: Query<(&Transform, &mut Enemy)>, 
                              window_query: Query<&Window, With<PrimaryWindow>>, 
                              audio: Res<Audio>, 
                              asset_server: Res<AssetServer>)
{
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            if SOUND_ENABLED {
                audio.play(sound_effect);
            }
        }
    }
}

pub fn enemy_hit_player(mut commands: Commands, 
                        mut player_query: Query<(Entity, &Transform), With<Player>>,  
                        mut game_over_event_writer: EventWriter<GameOver>,
                        enemy_query: Query<&Transform, With<Enemy>>, 
                        asset_server: Res<AssetServer>, 
                        audio: Res<Audio>,
                        score: Res<Score>)
{
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < (player_radius + enemy_radius) {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_collide_with_stars(mut commands: Commands, 
                                 mut player_query: Query<&Transform, With<Player>>, 
                                 star_query: Query<(&Transform, Entity), With<Star>>,
                                 mut score: ResMut<Score>)
{
    if let Ok(transform) = player_query.get_single_mut() {

        for (star_transform, star_entity) in star_query.iter() {
            
            let distance = transform.translation.distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < (player_radius + star_radius) {
                score.value += 1;
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Player Score: {}", score.value);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(mut commands: Commands, 
                             window_query: Query<&Window, With<PrimaryWindow>>, 
                             asset_server: Res<AssetServer>, 
                             star_spawn_timer: Res<StarSpawnTimer>)
{
    let window = window_query.get_single().unwrap();

    if star_spawn_timer.timer.finished() {
        spawn_star(&mut commands, window, &asset_server);
    }
}

pub fn exit_game(keyboard_input: Res<Input<KeyCode>>,
                 mut app_exit_event_wirter: EventWriter<AppExit>) 
{ 
    if keyboard_input.just_pressed(KeyCode::Escape) {  
        app_exit_event_wirter.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) { 
    for event in game_over_event_reader.iter() { 
        println!("Game Over: Final Score: {}", event.score);
    }
} 
