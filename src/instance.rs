use cgmath::{Quaternion, Rotation3, Vector3};

use crate::InstanceRaw;

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
    pub fn update_rotation(&mut self, delta_time_secs: f32, rotation_speed_deg_per_sec: f32) {
        let y_delta_rotation = Quaternion::from_axis_angle(
            Vector3::unit_y(),
            cgmath::Deg(rotation_speed_deg_per_sec * delta_time_secs / 1.7),
        );
        let z_delta_rotation = Quaternion::from_axis_angle(
            Vector3::unit_z(),
            cgmath::Deg(rotation_speed_deg_per_sec * delta_time_secs / 1.9),
        );
        let x_delta_rotation = Quaternion::from_axis_angle(
            Vector3::unit_x(),
            cgmath::Deg(rotation_speed_deg_per_sec * delta_time_secs),
        );
        self.rotation = self.rotation * z_delta_rotation * x_delta_rotation;
    }
}
