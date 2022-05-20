mod cubemap_material;

use std::env;

use bevy::{prelude::*, utils::hashbrown::HashSet};
use cubemap_material::CubemapMaterial;
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    mut materials: ResMut<Assets<CubemapMaterial>>,
) {
    let texture = asset_server.load(&settings.texture_path);

    if !settings.no_env {
        dbg!("env");
        let mesh = asset_server.load("models/standard_sphere_normals_flipped.glb#Mesh0/Primitive0");
        let material = materials.add(CubemapMaterial {
            normal: Vec4::new(-1.0, -1.0, -1.0, 0.0),
            texture: texture.clone(),
            needs_conversion: true,
        });

        commands.spawn().insert_bundle(MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(100.0, 100.0, 100.0)),
            material: material.clone(),
            ..Default::default()
        });
    }

    if settings.sphere {
        dbg!("sphere");
        let mesh = asset_server.load("models/standard_sphere.glb#Mesh0/Primitive0");
        let material = materials.add(CubemapMaterial {
            normal: Vec4::new(1.0, 1.0, 1.0, 1.0),
            texture: texture.clone(),
            needs_conversion: true,
        });

        commands.spawn().insert_bundle(MaterialMeshBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        });
    }

    if settings.dragon {
        dbg!("dragon");
        let mesh = asset_server.load("models/dragon.glb#Mesh0/Primitive0");
        let material = materials.add(CubemapMaterial {
            normal: Vec4::new(1.0, 1.0, 1.0, 1.0),
            texture: texture.clone(),
            needs_conversion: true,
        });

        commands.spawn().insert_bundle(MaterialMeshBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.0).with_scale(Vec3::new(0.01, 0.01, 0.01)),
            ..Default::default()
        });
    }

    commands.spawn_bundle(UnrealCameraBundle::new(
        UnrealCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ));
}

fn convert_to_cube(
    mut converted: Local<ConvertedTextures>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CubemapMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        // TODO find better way to track and update if texture needs conversion
        if converted.contains(&material.texture) {
            material.needs_conversion = false;
        } else {
            if let Some(image) = images.get_mut(&material.texture) {
                image.reinterpret_stacked_2d_as_array(6);
                converted.insert(material.texture.clone());
            }
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct ConvertedTextures(HashSet<Handle<Image>>);

#[derive(Default)]
pub struct Settings {
    pub texture_path: String,
    pub no_env: bool,
    pub sphere: bool,
    pub dragon: bool,
}

fn main() {
    // TODO just use egui instead of args
    println!("Options: textures/autumn_park.jpg no_env sphere dragon");
    let mut settings = Settings::default();
    settings.texture_path = String::from("textures/autumn_park.jpg");

    let mut args = env::args();
    args.next();

    for arg in args {
        // TODO actually see if this is a file path
        if arg.contains('.') {
            settings.texture_path = arg;
            continue;
        }
        match arg.as_str() {
            "no_env" => settings.no_env = true,
            "sphere" => settings.sphere = true,
            "dragon" => settings.dragon = true,
            _ => continue,
        }
    }

    App::new()
        .insert_resource(settings)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_startup_system(setup)
        .add_system(convert_to_cube)
        .run();
}
