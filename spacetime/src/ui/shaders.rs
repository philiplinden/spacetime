use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource, Reflect)]
struct ShaderHandles {
    time_dilation: Handle<Shader>,
    gravitational_potential: Handle<Shader>,
    current: Handle<Shader>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    // Load shaders
    let time_dilation_shader = shaders.add(Shader::from_wgsl(
        asset_server.load("shaders/time_dilation.wgsl"),
    ));
    let gravitational_potential_shader = shaders.add(Shader::from_wgsl(
        asset_server.load("shaders/gravitational_potential.wgsl"),
    ));

    commands.insert_resource(ShaderHandles {
        time_dilation: time_dilation_shader.clone(),
        gravitational_potential: gravitational_potential_shader.clone(),
        current: time_dilation_shader.clone(),
    });

    // Add a 2D quad with the shader material applied
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: asset_server.load("shaders/time_dilation.wgsl"), // Start with time dilation shader
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: shader_material,
        ..Default::default()
    });
}


fn ui_system(
    mut egui_ctx: EguiContexts,
    mut shader_handles: ResMut<ShaderHandles>,
    mut query: Query<&mut Handle<ColorMaterial>>,
) {
    egui::Window::new("Shader Toggle").show(egui_ctx.ctx_mut(), |ui| {
        if ui.button("Toggle Shader").clicked() {
            if let current = shader_handles.current.clone() {
                shader_handles.current = if current == shader_handles.time_dilation {
                    shader_handles.gravitational_potential.clone()
                } else {
                    shader_handles.time_dilation.clone()
                };

                // Apply the new shader to the mesh
                if let Some(material_handle) = query.iter_mut().next() {
                    *material_handle = shader_handles.current.clone().unwrap();
                }
            }
        }
    });
}
