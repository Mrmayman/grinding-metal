use crate::{interval::Interval, ray::Ray, vector::Vec3};

#[derive(Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            x: Interval::new(0.0, 0.0),
            y: Interval::new(0.0, 0.0),
            z: Interval::new(0.0, 0.0),
        }
    }

    pub fn new_point(a: &Vec3, b: &Vec3) -> AABB {
        AABB {
            x: Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x())),
            y: Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y())),
            z: Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z())),
        }
    }

    pub fn new_interval(ix: Interval, iy: Interval, iz: Interval) -> AABB {
        AABB {
            x: ix,
            y: iy,
            z: iz,
        }
    }

    pub fn new_aabb(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::new_interval(&box0.x, &box1.x),
            y: Interval::new_interval(&box0.y, &box1.y),
            z: Interval::new_interval(&box0.z, &box1.z),
        }
    }

    pub fn axis(&self, axis: i32) -> Interval {
        if axis == 2 {
            return self.z;
        }
        if axis == 1 {
            return self.y;
        }
        return self.x;
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        // Unoptimized hit method by Peter Shirley
        /*for a in 0..3 {
            let t0: f64 = f64::min(
                (self.axis(a).min - ray.origin()[a as usize]) / ray.direction()[a as usize],
                (self.axis(a).max - ray.origin()[a as usize]) / ray.direction()[a as usize],
            );
            let t1: f64 = f64::max(
                (self.axis(a).min - ray.origin()[a as usize]) / ray.direction()[a as usize],
                (self.axis(a).max - ray.origin()[a as usize]) / ray.direction()[a as usize],
            );
            ray_t.min = f64::max(t0, ray_t.min);
            ray_t.max = f64::min(t1, ray_t.max);
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;*/

        // Optimized hit method by Andrew Kensler at Pixar.
        for a in 0..3 {
            let invD: f64 = 1.0 / ray.direction()[a];
            let orig: f64 = ray.origin()[a];

            let mut t0 = (self.axis(a as i32).min - orig) * invD;
            let mut t1 = (self.axis(a as i32).max - orig) * invD;

            if invD < 0.0 {
                let t2 = t1;
                t1 = t0;
                t0 = t2;
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }

    pub fn pad(&self) -> AABB {
        // Return an AABB that has no side narrower than some delta, padding if necessary.
        let delta: f64 = 0.0001;
        let new_x: Interval;
        if self.x.size() >= delta {
            new_x = self.x;
        } else {
            new_x = self.x.expand(delta);
        }
        let new_y: Interval;
        if self.y.size() >= delta {
            new_y = self.y;
        } else {
            new_y = self.y.expand(delta);
        }
        let new_z: Interval;
        if self.z.size() >= delta {
            new_z = self.z;
        } else {
            new_z = self.z.expand(delta);
        }

        AABB::new_interval(new_x, new_y, new_z)
    }
}
