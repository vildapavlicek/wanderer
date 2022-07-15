use crate::{components::player::*, resources::Materials};
use bevy::{ecs::query, prelude::*, sprite::collide_aabb};
use bevy_rapier2d::{na::Translation, prelude::*};
use std::f32::consts::PI;

const MOVE_SPEED: f32 = 250_f32;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage_after(
            StartupStage::PostStartup,
            "spawn_player",
            SystemStage::single(spawn_player),
        )
        .add_system(handle_keyboard_input)
       // .add_system(display_player_contact_info)
        .add_system(display_player_intersection_info)
        //.add_system(rotator)
        ;
    }
}

fn spawn_player(mut cmd: Commands, materials: Res<Materials>) {
    cmd.spawn_bundle(SpriteBundle {
        texture: materials.player_material.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(super::SPRITE_SIZE, super::SPRITE_SIZE)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-96., -96., 0.),
        ..Default::default()
    })
    .insert(Player)
    // .insert(StepTimer(Timer::from_seconds(0.1, true)))
    .insert(RigidBody::Dynamic)
    .insert(Velocity {
        linvel: Vec2::new(0., 0.),
        angvel: 0.,
    })
    .insert(Collider::cuboid(16_f32, 16_f32))
    .with_children(|children| {

        /* let max = 8;

        for i in 1..=max {
            children
            .spawn()
            .insert(Collider::cuboid(64_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::ZERO, //Vec3::new(36_f32, 36_f32, 0_f32),
                rotation: Quat::from_rotation_z((PI / max as f32) * i as f32),
                ..Default::default()
            }))
            .insert(Sensor);
        } */


         let max = 20;

        for i in 1..=max {
            let angle = (PI / max as f32) * i as f32;
            children
            .spawn()
            .insert(Collider::cuboid(32_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::new(((32. * 1.5) * angle.cos()), (32_f32 * 1.5) * angle.sin(), 0.), //Vec3::new(36_f32, 36_f32, 0_f32),
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            }))
            .insert(Sensor);
        }

        let angle = PI;
            children
            .spawn()
            .insert(Collider::cuboid(32_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::new(48. ,0., 0.), //Vec3::new(36_f32, 36_f32, 0_f32),
                ..Default::default()
            }))
            .insert(Sensor);

        /* let x = 4.;
        let angle = PI / x;

        children
            .spawn()
            .insert(Collider::cuboid(32_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::new( ((32. * 1.5) * angle.cos()), (32_f32 * 1.5) * angle.sin(), 0.), //Vec3::new(((angle * PI) / 2.) * 32., ((angle * PI) / 2.) * 32., 0.), //Vec3::new(36_f32, 36_f32, 0_f32),
                rotation: Quat::from_rotation_z(angle),
                //translation: Vec3::new(80., 16., 0.),
                //rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            }))
            .insert(Sensor); */
        

        /* children
            .spawn()
            .insert(Collider::cuboid(64_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::ZERO, //Vec3::new(36_f32, 36_f32, 0_f32),
                rotation: Quat::from_rotation_z(0.75),
                ..Default::default()
            }))
            .insert(Sensor);

        children
            .spawn()
            .insert(Collider::cuboid(32_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::ZERO, //Vec3::new(0_f32, 48_f32, 0_f32),
                rotation: Quat::from_rotation_z(1.57),
                ..Default::default()
            }))
            .insert(Sensor);

        children
            .spawn()
            .insert(Collider::cuboid(32_f32, 1_f32))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::ZERO, //Vec3::new(-36_f32, 36_f32, 0_f32),
                rotation: Quat::from_rotation_z(-0.75),
                ..Default::default()
            }))
            .insert(Sensor); */
    })
    .insert(GravityScale(0.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Restitution::coefficient(0.7)) // creates a bounce effect
    ;

    cmd.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PlayerCamera);
}

#[derive(Component)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component)]
pub struct Move {
    direction: Direction,
}

impl Move {}

fn handle_keyboard_input(
    mut cmd: Commands,
    input: Res<Input<KeyCode>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let x_axis =
        (-(input.pressed(KeyCode::Left) as i8) + (input.pressed(KeyCode::Right) as i8)) as f32;

    let y_axis =
        (-(input.pressed(KeyCode::Down) as i8) + (input.pressed(KeyCode::Up) as i8)) as f32;

    player_velocity.single_mut().linvel = Vec2::new(x_axis * MOVE_SPEED, y_axis * MOVE_SPEED);
}

fn display_player_contact_info(
    rapier_context: Res<RapierContext>,
    player: Query<Entity, (With<Player>, With<Collider>)>,
) {
    let player_entity = player.single();
    rapier_context
        .contacts_with(player_entity)
        .for_each(|contact_pair| {
            for manifold in contact_pair.manifolds() {
                println!("Local-space contact normal: {}", manifold.local_n1());
                println!("Local-space contact normal: {}", manifold.local_n2());
                println!("World-space contact normal: {}", manifold.normal());

                // Read the geometric contacts.
                for contact_point in manifold.points() {
                    // Keep in mind that all the geometric contact data are expressed in the local-space of the colliders.
                    println!(
                        "Found local contact point 1: {:?}",
                        contact_point.local_p1()
                    );
                    println!("Found contact distance: {:?}", contact_point.dist()); // Negative if there is a penetration.
                    println!("Found contact impulse: {}", contact_point.impulse());
                    println!(
                        "Found friction impulse: {}",
                        contact_point.tangent_impulse()
                    );
                }

                // Read the solver contacts.
                for solver_contact in manifold.solver_contacts() {
                    // Keep in mind that all the solver contact data are expressed in world-space.
                    println!("Found solver contact point: {:?}", solver_contact.point());
                    println!("Found solver contact distance: {:?}", solver_contact.dist());
                    // Negative if there is a penetration.
                }
            }
        })
}

fn display_player_intersection_info(
    rapier_context: Res<RapierContext>,
    sensors: Query<(Entity, &Sensor, &Collider)>,
    player_collider: Query<(Entity, &Transform), (With<Player>, With<Collider>)>,
    colliders: Query<&Transform, With<Collider>>
) {
    /* for (sensor_entity, sensor, collider) in sensors.iter() {
        for (collider1, collider2, intersection) in rapier_context.intersections_with(sensor_entity)
        {
            info!(
                "collider1: '{collider1:?}' | collider2 '{collider2:?}' | intersection '{intersection}' "
            );

            dbg!(rapier_context
                .contact_pair(collider1, collider2)
                .map(|cp| cp.has_any_active_contacts()));

            /* if let Some(contact_pair) = rapier_context.contact_pair(collider1, collider2) {
                debug!("contact pair found");
                contact_pair
                    .manifolds()
                    .enumerate()
                    .for_each(|(index, manifold)| {
                        manifold.points().for_each(|point| {
                            info!(%index, distance = ?point.dist());
                        })
                    });
            } */
        }
    } */

    let (p_entity, p_transform) = player_collider.single();

    sensors.iter().for_each(|(entity, _, _)|
        {
            rapier_context.intersections_with(entity).for_each(|(c1, c2, b)| {
                info!("collision found ({b}) for entity '{entity:?}', collider 1 '{c1:?}', collider 2 '{c2:?}'");

                if b {
                    let parent = rapier_context.collider_parent(c1);
                    // info!("intersected sensor's parent is  {:?}, player's collider entity is {p_entity:?}", parent);
                    if let Some(parent) = parent {
                        if let Ok(transform) = colliders.get(parent) {
                            info!("player translation: {:?} | collider translation {:?}", p_transform.translation, transform.translation);
                            info!("distance is {:?}", p_transform.translation.distance(transform.translation));
                        }
                    }

                }
                

            });

            
        }

    );
}


fn rotator(mut collider: Query<&mut Transform, (With<Player>, With<Collider>, Without<Sensor>)>) {
    let y = rand::random::<f32>() * 10.;
    let mut transform = collider.single_mut();
    transform.rotation = Quat::from_rotation_z(transform.rotation.z + PI / y);
}