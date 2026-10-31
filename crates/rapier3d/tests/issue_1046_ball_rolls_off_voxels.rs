use rapier3d::prelude::*;

#[test]
fn ball_rolling_off_voxel_edge_does_not_panic() {
    let mut world = PhysicsWorld::new();
    let mut cells = Vec::new();
    for x in 0..32 {
        for z in 0..8 {
            cells.push(Vector::new(x as Real + 0.5, 0.5, z as Real + 0.5));
        }
    }
    let ground = world.insert_body(RigidBodyBuilder::fixed().build());
    world.insert_collider(
        ColliderBuilder::voxels_from_points(Vector::new(1.0, 1.0, 1.0), &cells),
        Some(ground),
    );
    let ball = world.insert_body(
        RigidBodyBuilder::dynamic()
            .translation(Vector::new(26.0, 1.4, 4.3))
            .linvel(Vector::new(4.0, 0.0, 0.0))
            .build(),
    );
    world.insert_collider(ColliderBuilder::ball(0.4).build(), Some(ball));
    for _ in 0..300 {
        world.step();
    }
    // the ball left the floor and fell past it
    assert!(world.bodies[ball].translation().y < 0.0);
}
