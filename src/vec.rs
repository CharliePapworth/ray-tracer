use crate::nalgebra::{Point3, Vector3};

type Vec3 = Vector3<f64>;

pub trait VecExtensionMethods {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64>;
    fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64>;
    fn offset_origin(
        origin: &Point3<f64>,
        dir: &Vector3<f64>,
        p_err: &Vector3<f64>,
        norm: &Vector3<f64>,
    ) -> Point3<f64>;
    fn near_zero(&self) -> bool;
    fn swap(&mut self, i: usize, j: usize);
}

impl VecExtensionMethods for Vector3<f64> {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self - 2.0 * self.dot(&normal) * normal
    }

    fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
        let cos_theta = -uv.dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    fn offset_origin(
        origin: &Point3<f64>,
        dir: &Vector3<f64>,
        p_err: &Vector3<f64>,
        norm: &Vector3<f64>,
    ) -> Point3<f64> {
        let d = norm.abs().dot(&p_err);
        let mut offset = d * p_err;
        if dir.dot(&norm) < 0.0 {
            offset = -offset;
        }
        origin + offset
    }

    fn near_zero(&self) -> bool {
        let min = 1e-8;
        self[0].abs() < min && self[1].abs() < min && self[2].abs() < min
    }

    fn swap(&mut self, i: usize, j: usize) {
        let temp = self[j];
        self[j] = self[i];
        self[i] = temp;
    }
}
