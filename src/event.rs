use bevy::prelude::*;

use crate::{coin::Coin, snake::SnakeHead};

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeCollideEvent>()
            .add_event::<CoinCollectedEvent>()
            .add_event::<GameOverEvent>()
            .add_systems(Update, snake_collide_event_writer);
    }
}

#[derive(Event, Debug)]
pub struct SnakeCollideEvent;

#[derive(Event, Debug)]
pub struct CoinCollectedEvent;

#[derive(Event, Debug)]
pub struct GameOverEvent;

fn snake_collide_event_writer(
    mut snake_collide_event: EventWriter<SnakeCollideEvent>,
    mut coin_collected_event: EventWriter<CoinCollectedEvent>,
    mut commands: Commands,
    snake_query: Query<&Transform, With<SnakeHead>>,
    circle_query: Query<(&Transform, Entity), With<Coin>>,
) {
    if let Ok(snake_transform) = snake_query.get_single() {
        for (circle_transform, circle_entity) in circle_query.iter() {
            let snake_position = snake_transform.translation.truncate();
            let circle_position = circle_transform.translation.truncate();
            let distance = snake_position.distance(circle_position);

            let snake_radius = 10.0;
            let circle_radius = 10.0;

            if distance < snake_radius + circle_radius {
                snake_collide_event.send(SnakeCollideEvent);
                coin_collected_event.send(CoinCollectedEvent);
                commands.entity(circle_entity).despawn();
            }
        }
    }
}
