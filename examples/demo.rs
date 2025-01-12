use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use bevy_frustum_culling::*;
use bevy_mod_bounding::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugins(DefaultPlugins)
        .add_plugin(BoundingVolumePlugin::<obb::OrientedBB>::default())
        .add_plugin(FrustumCullingPlugin::<obb::OrientedBB>::default())
        .add_startup_system(setup.system())
        .add_system(camera_rotation_system.system())
        .add_system(mesh_rotation_system.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_path = "models/waterbottle/WaterBottle.gltf#Mesh0/Primitive0";
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("models").unwrap();
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    let mesh_handle = asset_server.get_handle(mesh_path);

    commands
        /*
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(10.0, 10.0, 10.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        */
        .spawn(PerspectiveCameraBundle {
            /*
            camera: Camera {
                name: Some("Secondary".to_string()),
                ..Default::default()
            },
            */
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(FrustumCulling)
        .with(CameraRotator)
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                ..Default::default()
            });
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    for x in -10..10 {
        for y in -10..10 {
            for z in -10..10 {
                if !(x == 0 && y == 0 && z == 0) {
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle.clone(),
                            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                            transform: Transform::from_translation(Vec3::new(
                                x as f32 * 2.0,
                                y as f32 * 2.0,
                                z as f32 * 2.0,
                            )),
                            ..Default::default()
                        })
                        .with(MeshRotator)
                        // Manually set the bounding volume of the mesh. We can precompute the
                        // bounds and specify them. Computing for every mesh makes startup slow.
                        .with(obb::OrientedBB::from_aabb_orientation(
                            aabb::AxisAlignedBB::from_extents(
                                Vec3::new(-0.05437539, -0.13022034, -0.0543754),
                                Vec3::new(0.05437539, 0.13022034, 0.054375406),
                            ),
                            Quat::from_xyzw(0.0, 0.1305262, 0.0, 0.9914449),
                        ))
                        .with(debug::DebugBounds);
                }
            }
        }
    }
}

struct CameraRotator;

fn camera_rotation_system(time: Res<Time>, mut query: Query<&mut Transform, With<CameraRotator>>) {
    for mut transform in query.iter_mut() {
        let rot_y = Quat::from_rotation_y((0.2 * time.seconds_since_startup() as f32).sin() * 4.0);
        *transform = Transform::from_rotation(rot_y);
    }
}

struct MeshRotator;

fn mesh_rotation_system(time: Res<Time>, mut query: Query<&mut Transform, With<MeshRotator>>) {
    for mut transform in query.iter_mut() {
        let scale = Vec3::ONE * ((time.seconds_since_startup() as f32).sin() + 1.5);
        let rot_x =
            Quat::from_rotation_x((time.seconds_since_startup() as f32 / 5.0).sin() / 100.0);
        let rot_y =
            Quat::from_rotation_y((time.seconds_since_startup() as f32 / 3.0).sin() / 100.0);
        let rot_z =
            Quat::from_rotation_z((time.seconds_since_startup() as f32 / 4.0).sin() / 100.0);
        transform.scale = scale;
        transform.rotate(rot_x * rot_y * rot_z);
    }
}
