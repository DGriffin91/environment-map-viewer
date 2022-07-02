mod cubemap_material;

use bevy::{prelude::*, utils::hashbrown::HashSet};
use cubemap_material::CubemapMaterial;

use bevy_egui::{
    egui::{self, FontDefinitions},
    EguiContext, EguiPlugin,
};

#[derive(Component)]
enum Item {
    Dragon,
    Sphere,
    Env,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CubemapMaterial>>,
) {
    let texture = asset_server.load("textures/autumn_park.jpg");

    let mesh = asset_server.load("models/standard_sphere_normals_flipped.glb#Mesh0/Primitive0");
    let material = materials.add(CubemapMaterial {
        normal: Vec4::new(-1.0, -1.0, -1.0, 0.0),
        texture: texture.clone(),
        needs_conversion: true,
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(100.0, 100.0, 100.0)),
            material,
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(Item::Env);

    let mesh = asset_server.load("models/standard_sphere.glb#Mesh0/Primitive0");
    let material = materials.add(CubemapMaterial {
        normal: Vec4::new(1.0, 1.0, 1.0, 1.0),
        texture: texture.clone(),
        needs_conversion: true,
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh,
            material,
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(Item::Sphere);

    let mesh = asset_server.load("models/dragon.glb#Mesh0/Primitive0");
    let material = materials.add(CubemapMaterial {
        normal: Vec4::new(1.0, 1.0, 1.0, 1.0),
        texture,
        needs_conversion: true,
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh,
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.01, 0.01, 0.01)),
            visibility: Visibility { is_visible: true },
            ..Default::default()
        })
        .insert(Item::Dragon);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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
                if image.texture_descriptor.size.height % 6 != 0 {
                    println!("Dimensions incorrect for cube map strip. Proportions should be 1 wide by 6 tall.")
                } else {
                    image.reinterpret_stacked_2d_as_array(6);
                    converted.insert(material.texture.clone());
                }
            }
        }
    }
}

fn ui(
    mut egui_context: ResMut<EguiContext>,
    mut items: Query<(&mut Visibility, &Item)>,
    mut materials: ResMut<Assets<CubemapMaterial>>,
    asset_server: Res<AssetServer>,
    mut env_path: Local<String>,
    mut drop_events: EventReader<FileDragAndDrop>,
    mut drop_hovered: Local<bool>,
    //mut orbit_cam: Query<&mut OrbitCameraController>,
) {
    let mut update_env = false;

    for event in drop_events.iter() {
        match event {
            FileDragAndDrop::DroppedFile { path_buf, .. } => {
                *env_path = path_buf.to_string_lossy().to_string();
                update_env = true;
                *drop_hovered = false;
            }
            FileDragAndDrop::HoveredFile { .. } => *drop_hovered = true,
            FileDragAndDrop::HoveredFileCancelled { .. } => *drop_hovered = false,
        }
    }
    let _panel_hovered = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            if *drop_hovered {
                ui.label("🙌");
            } else {
                ui.label("");
            }
            ui.label("CTRL + mouse drag: Rotate camera");
            ui.label("Right mouse drag: Pan camera");
            ui.label("Mouse wheel: Zoom");
            for (mut vis, item) in items.iter_mut() {
                match item {
                    Item::Dragon => ui.checkbox(&mut vis.is_visible, "Dragon"),
                    Item::Sphere => ui.checkbox(&mut vis.is_visible, "Sphere"),
                    Item::Env => ui.checkbox(&mut vis.is_visible, "Environment"),
                };
            }
            if ui.button("Reset Environment Texture").clicked() {
                *env_path = String::from("textures/autumn_park.jpg");
                update_env = true;
            }
            ui.text_edit_singleline(&mut *env_path);
            if ui.button("Load Environment Texture").clicked() || update_env {
                dbg!(&env_path);
                let env = asset_server.load(&*env_path);
                for (_, mat) in materials.iter_mut() {
                    mat.needs_conversion = true;
                    mat.texture = env.clone();
                }
            }
        })
        .response
        .hovered();
    //if let Some(mut cam) = orbit_cam.iter_mut().next() {
    //    cam.enabled = !(egui_context.ctx_mut().wants_pointer_input() || panel_hovered);
    //}
}

pub fn setup_fonts(mut egui_context: ResMut<EguiContext>) {
    let mut fonts = FontDefinitions::default();

    for (_text_style, data) in fonts.font_data.iter_mut() {
        data.tweak.scale = 1.5;
    }
    egui_context.ctx_mut().set_fonts(fonts);
}

#[derive(Default, Deref, DerefMut)]
pub struct ConvertedTextures(HashSet<Handle<Image>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .add_startup_system(setup)
        .add_startup_system(setup_fonts)
        .add_system(convert_to_cube)
        .add_system(ui)
        .run();
}
