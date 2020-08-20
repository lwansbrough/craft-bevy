use bevy::prelude::*;

pub trait EulerUtilities {
    fn ypr(&self) -> (f32, f32, f32);
}

impl EulerUtilities for Quat {
    fn ypr(&self) -> (f32, f32, f32) {
        let matrix = Mat3::from_quat(*self);
        // Implementation informed by "Computing Euler angles from a rotation matrix", by Gregory G. Slabaugh
        //  https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.371.6578
        if matrix.z_axis().x().abs() < 1.0 {
            let yaw = -matrix.z_axis().x().asin();
            let pitch = (matrix.y_axis().x() / yaw.cos()).atan2(matrix.x_axis().x() / yaw.cos());
            let roll = (matrix.z_axis().y() / yaw.cos()).atan2(matrix.z_axis().z() / yaw.cos());
            (roll, yaw, pitch)
        } else if matrix.z_axis().x() <= -1.0 {
            (std::f32::consts::FRAC_PI_2, 0.0, matrix.x_axis().y().atan2(matrix.x_axis().z()))
        } else {
            (
                -std::f32::consts::FRAC_PI_2,
                0.0,
                -matrix.x_axis().y().atan2(-matrix.x_axis().z())
            )
        }
    }
}
