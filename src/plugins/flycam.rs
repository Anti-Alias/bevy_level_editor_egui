use std::f32::consts::FRAC_PI_2;

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

const LOOK_SCALE: f32 = 0.007;
const LINE_SCALE: f32 = 1.0;

#[derive(Default)]
pub struct FlycamPlugin;
impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_flycam);
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bindings {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    near: KeyCode,
    far: KeyCode,
    rotate: MouseButton
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            left: KeyCode::A,
            right: KeyCode::D,
            up: KeyCode::Space,
            down: KeyCode::ShiftLeft,
            near: KeyCode::S,
            far: KeyCode::W,
            rotate: MouseButton::Middle
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Flycam {
    speed: f32,
    look_sensitivity: f32,
    scroll_sensitivity: f32,
    pitch: f32,
    yaw: f32,
    bindings: Bindings
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            speed: 7.0,
            look_sensitivity: 0.5,
            scroll_sensitivity: 0.5,
            pitch: -0.3,
            yaw: 0.0,
            bindings: default()
        }
    }
}

impl Flycam {
    pub fn direction(&self) -> Vec3 {
        let quat = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        quat * Vec3::NEG_Z
    }
    fn direction_xz(&self) -> Vec3 {
        let quat = Quat::from_euler(EulerRot::XYZ, 0.0, self.yaw, 0.0);
        quat * Vec3::NEG_Z
    }
}

fn control_flycam(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut flycams: Query<(&mut Transform, &mut Flycam)>
) {
    const EPS: f32 = 0.01;
    let delta = time.delta_seconds();

    for (mut trans, mut cam) in &mut flycams {

        // Orients camera
        if mouse_input.pressed(cam.bindings.rotate) {
            for ev in mouse_motion_events.iter() {
                let theta = -LOOK_SCALE * ev.delta.x * cam.look_sensitivity;
                cam.yaw += theta;
                let theta = -LOOK_SCALE * ev.delta.y * cam.look_sensitivity;
                cam.pitch += theta;
                if cam.pitch > FRAC_PI_2-EPS {
                    cam.pitch = FRAC_PI_2-EPS;
                }
                else if cam.pitch < -FRAC_PI_2+EPS {
                    cam.pitch = -FRAC_PI_2+EPS;
                }
            };
        }

        // Zooms camera
        let direction = cam.direction();
        for ev in mouse_wheel_events.iter() {
            let zoom = direction * match ev.unit {
                MouseScrollUnit::Line => ev.y * LINE_SCALE * cam.scroll_sensitivity,
                MouseScrollUnit::Pixel => 0.0   // TODO
            };
            trans.translation += zoom;
        }

        // Moves camera
        let far = cam.direction_xz();
        let up = Vec3::Y;
        let left = up.cross(far);
        let right = -left;
        let down = -up;
        let near = -far;
        if key_input.pressed(cam.bindings.left) {
            trans.translation += left * cam.speed * delta;
        }
        if key_input.pressed(cam.bindings.right) {
            trans.translation += right * cam.speed * delta;
        }
        if key_input.pressed(cam.bindings.up) {
            trans.translation += up * cam.speed * delta;
        }
        if key_input.pressed(cam.bindings.down) {
            trans.translation += down * cam.speed * delta;
        }
        if key_input.pressed(cam.bindings.near) {
            trans.translation += near * cam.speed * delta;
        }
        if key_input.pressed(cam.bindings.far) {
            trans.translation += far * cam.speed * delta;
        }

        // Syncs camera's direction with pitch + yaw
        let target = trans.translation + direction;
        trans.look_at(target, Vec3::Y);
    }
}