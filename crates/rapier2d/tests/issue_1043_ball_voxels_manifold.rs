use rapier2d::prelude::*;

#[test]
fn ball_on_single_voxel_does_not_panic() {
    let mut world = PhysicsWorld::new();
    let ground = world.insert_body(
        RigidBodyBuilder::new(RigidBodyType::Fixed)
            .pose(Pose::new(Vector::new(0.0, 0.0), 0.0))
            .build(),
    );
    world.insert_collider(
        ColliderBuilder::voxels_from_points(Vector::new(0.1, 0.1), &[Vector::new(0.0, 0.0)]),
        Some(ground),
    );
    let ball = world.insert_body(
        RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .pose(Pose::new(Vector::new(0.05, 2.0), 0.4))
            .angvel(2.0)
            .can_sleep(false)
            .build(),
    );
    world.insert_collider(
        ColliderBuilder::new(SharedShape::ball(0.1))
            .density(2700.0)
            .friction(0.8)
            .build(),
        Some(ball),
    );
    for _ in 0..600 {
        world.step();
    }
}
