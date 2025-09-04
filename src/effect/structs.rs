use bevy::prelude::*;
use std::ops::Range;

use crate::util::make_rect_mesh;

// 파티클은 부모는 위치만, 자식하나가 파티클을 가진다
// 자식하나의 자식들로 모든 그래픽을 처리 할 수 있다. 다만, 파티클인데 단일그래픽으로 해야하지 않을까?

#[derive(Component, Debug)]
pub struct Effect {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
    pub time: f32,
    pub time_max: f32,
    pub scale_start: Vec3,
    pub scale_end: Vec3,
    pub direct_start: Vec3,
    pub direct_end: Vec3,
    pub color_start: Vec4,
    pub color_end: Vec4,
    pub rotate: Quat,
}

impl Effect {
    pub fn next(
        &mut self,
        time: &Res<Time>,
        trans: &mut Transform,
        color: &Handle<ColorMaterial>,
        commands: &mut Commands,
        entity: Entity,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        c_trans: &mut Transform,
    ) {
        if self.time >= self.time_max {
            commands.get_entity(entity).unwrap().despawn();
            materials.remove(color);
            return;
        }

        let color = materials.get_mut(color).unwrap();
        let delta_sec = time.delta_secs();
        let pos = &mut trans.translation;
        let color = &mut color.color;

        let t_normal = self.time / self.time_max;

        self.time += delta_sec;

        let direct = ((1. - t_normal) * self.direct_start) + (t_normal * self.direct_end);
        let color_vec = ((1. - t_normal) * self.color_start) + (t_normal * self.color_end);
        let scale = ((1. - t_normal) * self.scale_start) + (t_normal * self.scale_end);

        if pos.length_squared() > 0.0 {
            trans.rotation = Quat::from_rotation_arc(Vec3::Y, direct.normalize());
        }

        *pos += direct * delta_sec;
        *color = Color::srgba(color_vec.x, color_vec.y, color_vec.z, color_vec.w);
        trans.scale = scale;
        c_trans.rotate(self.rotate * delta_sec);
    }
}

#[derive(Component)]
pub struct EffectMesh;

#[derive(Resource)]
pub struct EffectModel {
    pub dot: Handle<Mesh>,
}

impl EffectModel {
    pub fn setup(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let dot = meshes.add(make_rect_mesh(
            &vec![(1. as f32, 1. as f32, 1. as f32, 1. as f32)],
            Color::WHITE,
        ));

        EffectModel { dot }
    }
}

pub fn effect_model_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(EffectModel::setup(&mut meshes));
}

#[derive(Component, Debug)]
pub struct EffectMaker {
    pub count: Range<usize>,       // 파티클 개수
    pub start_dir: Range<Vec3>,    // 방향 범위
    pub end_dir: Range<Vec3>,      // 방향 범위
    pub start_color: Range<Vec4>,  // 색상 범위
    pub end_color: Range<Vec4>,    // 색상 범위
    pub start_scale: Range<f32>,   // 크기 범위
    pub end_scale: Range<f32>,     // 크기 범위
    pub max_time: Range<f32>,      // 시간 범위
    pub rotate: Range<Vec3>,       // 회전 범위
    pub meshes: Vec<Handle<Mesh>>, // 이펙트 메쉬
    pub make_flag: bool,           // 메이커 실행 플래그
    pub offset_pos: Vec3,          // 이펙트 생성 지점
}

fn rand_between(a: f32, b: f32) -> f32 {
    fastrand::f32() * (b - a) + a
}
impl EffectMaker {
    pub fn spawn_effect(
        &self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        trans: &Transform,
    ) {
        let count = fastrand::usize(self.count.clone()).max(1);

        let mut effects = Vec::with_capacity(count);

        let mut trans = trans.clone();

        for _ in 0..count {
            let start_dir = trans.rotation
                * Vec3::new(
                    rand_between(self.start_dir.start.x, self.start_dir.end.x),
                    rand_between(self.start_dir.start.y, self.start_dir.end.y),
                    rand_between(self.start_dir.start.z, self.start_dir.end.z),
                );
            let end_dir = trans.rotation
                * Vec3::new(
                    rand_between(self.end_dir.start.x, self.end_dir.end.x),
                    rand_between(self.end_dir.start.y, self.end_dir.end.y),
                    rand_between(self.end_dir.start.z, self.end_dir.end.z),
                );

            let start_scale = rand_between(self.start_scale.start, self.start_scale.end);
            let end_scale = rand_between(self.end_scale.start, self.end_scale.end);

            let start_color = Vec4::new(
                rand_between(self.start_color.start.x, self.start_color.end.x),
                rand_between(self.start_color.start.y, self.start_color.end.y),
                rand_between(self.start_color.start.z, self.start_color.end.z),
                rand_between(self.start_color.start.w, self.start_color.end.w),
            );
            let end_color = Vec4::new(
                rand_between(self.end_color.start.x, self.end_color.end.x),
                rand_between(self.end_color.start.y, self.end_color.end.y),
                rand_between(self.end_color.start.z, self.end_color.end.z),
                rand_between(self.end_color.start.w, self.end_color.end.w),
            );

            let rotate = Quat::from_euler(
                EulerRot::XYZ,
                rand_between(self.rotate.start.x, self.rotate.end.x),
                rand_between(self.rotate.start.y, self.rotate.end.y),
                rand_between(self.rotate.start.z, self.rotate.end.z),
            );

            let mesh = fastrand::usize(0..(self.meshes.len()));
            let mesh = self.meshes[mesh].clone();

            effects.push(Effect {
                mesh: mesh.clone(),
                color: materials.add(Color::srgba(
                    start_color.x,
                    start_color.y,
                    start_color.z,
                    start_color.w,
                )),
                time: 0.001,
                time_max: (fastrand::f32() * (self.max_time.end - self.max_time.start))
                    + self.max_time.start,
                scale_start: Vec3::splat(start_scale),
                scale_end: Vec3::splat(end_scale),
                direct_start: start_dir,
                direct_end: end_dir,
                color_start: start_color,
                color_end: end_color,
                rotate,
            });
        }

        trans.translation += self.offset_pos;
        for effect in effects {
            let child = (
                Transform::default(),
                Mesh2d(effect.mesh.clone()),
                MeshMaterial2d(effect.color.clone()),
                EffectMesh,
            );
            commands
                .spawn((effect, trans.clone(), Visibility::default()))
                .with_child(child);
        }
    }
}
