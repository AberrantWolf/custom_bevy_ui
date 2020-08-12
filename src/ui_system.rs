use bevy::prelude::*;

use bevy::render::draw::{Draw, DrawContext, DrawError, Drawable};
use bevy::render::mesh;
use bevy::render::renderer::{
    AssetRenderResourceBindings, BindGroup, BufferUsage, RenderResourceBindings, RenderResourceId,
};
use bevy::render::{
    camera::{Camera, OrthographicProjection, VisibleEntities, WindowOrigin},
    mesh::Mesh,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
};
use bevy::sprite;

use bevy::sprite::{ColorMaterial, QUAD_HANDLE};

use bevy::ui::widget::{Button, Image, Text};
use bevy::ui::UI_PIPELINE_HANDLE;
use bevy::ui::{CalculatedSize, FocusPolicy, Interaction, Style};

#[derive(Bundle)]
pub struct UiWidgetComponents {
    pub node: Node,
    pub widget: UiWidget,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    // pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    // pub material: Handle<ColorMaterial>,
    pub draw: Draw,
    // pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub local_transform: LocalTransform,
}

impl Default for UiWidgetComponents {
    fn default() -> Self {
        UiWidgetComponents {
            widget: UiWidget,
            // mesh: QUAD_HANDLE,
            // render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
            //     UI_PIPELINE_HANDLE,
            //     PipelineSpecialization {
            //         dynamic_bindings: vec![
            //             // Transform
            //             DynamicBinding {
            //                 bind_group: 1,
            //                 binding: 0,
            //             },
            //             // Node_size
            //             DynamicBinding {
            //                 bind_group: 1,
            //                 binding: 1,
            //             },
            //         ],
            //         ..Default::default()
            //     },
            // )]),
            interaction: Default::default(),
            focus_policy: Default::default(),
            node: Default::default(),
            style: Default::default(),
            // material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            local_transform: Default::default(),
        }
    }
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

#[derive(Default)]
pub struct UiWidget;

pub fn draw_widget_system(
    mut draw_context: DrawContext,
    fonts: Res<Assets<Font>>,
    msaa: Res<Msaa>,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    mut asset_render_resource_bindings: ResMut<AssetRenderResourceBindings>,
    mut query: Query<(&mut Draw, &UiWidget, &Node, &Transform)>,
) {
    for (mut draw, widg, node, transform) in &mut query.iter() {
        let position =
            Vec3::from(transform.value.w_axis().truncate()) - (node.size / 2.0).extend(0.0);

        draw_context
            .set_pipeline(
                &mut draw,
                sprite::SPRITE_SHEET_PIPELINE_HANDLE,
                &PipelineSpecialization {
                    sample_count: msaa.samples,
                    ..Default::default()
                },
            )
            .unwrap();

        let render_resource_context = &**draw_context.render_resource_context;
        if let Some(RenderResourceId::Buffer(quad_vertex_buffer)) = render_resource_context
            .get_asset_resource(sprite::QUAD_HANDLE, mesh::VERTEX_BUFFER_ASSET_INDEX)
        {
            draw.set_vertex_buffer(0, quad_vertex_buffer, 0);
        }
        let mut indices = 0..0;
        if let Some(RenderResourceId::Buffer(quad_index_buffer)) = render_resource_context
            .get_asset_resource(sprite::QUAD_HANDLE, mesh::INDEX_BUFFER_ASSET_INDEX)
        {
            draw.set_index_buffer(quad_index_buffer, 0);
            if let Some(buffer_info) = render_resource_context.get_buffer_info(quad_index_buffer) {
                indices = 0..(buffer_info.size / 2) as u32;
            } else {
                panic!("expected buffer type");
            }
        }

        draw_context
            .set_bind_groups_from_bindings(&mut draw, &mut [&mut render_resource_bindings])
            .unwrap();
        draw.draw_indexed(indices.clone(), 0, 0..1);
    }
}

// pub fn button_system(
//     button_materials: Res<ButtonMaterials>,
//     mut interaction_query: Query<(
//         &Button,
//         Mutated<Interaction>,
//         &mut Handle<ColorMaterial>,
//         &Children,
//     )>,
//     text_query: Query<&mut Text>,
// ) {
//     for (_button, interaction, mut material, children) in &mut interaction_query.iter() {
//         let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 text.value = "Press".to_string();
//                 *material = button_materials.pressed;
//             }
//             Interaction::Hovered => {
//                 text.value = "Hover".to_string();
//                 *material = button_materials.hovered;
//             }
//             Interaction::None => {
//                 text.value = "Button".to_string();
//                 *material = button_materials.normal;
//             }
//         }
//     }
// }
