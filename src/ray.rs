use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray(Vec3, Vec3);

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray(origin, direction)
    }

    pub fn origin(&self) -> Vec3 {
        self.0
    }

    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.0 + t * self.1
    }
}

#[cfg(test)]
mod tests {
}
