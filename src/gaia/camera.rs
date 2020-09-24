#![allow(dead_code)]

use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}
use self::CameraMovement::*;

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Debug,Clone,Copy)]
pub struct Camera {
    // Camera Attributes
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub worldup: Vector3,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(),    // initialized later
            right: Vector3::zero(), // initialized later
            worldup: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensivity: SENSITIVTY,
            zoom: ZOOM,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        if direction == FORWARD {
            self.position += self.front * velocity;
        }
        if direction == BACKWARD {
            self.position += -(self.front * velocity);
        }
        if direction == LEFT {
            self.position += -(self.right * velocity);
        }
        if direction == RIGHT {
            self.position += self.right * velocity;
        }
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn process_mouse_movement(
        &mut self,
        mut xoffset: f32,
        mut yoffset: f32,
        constrainpitch: bool,
    ) {
        xoffset *= self.mouse_sensivity;
        yoffset *= self.mouse_sensivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrainpitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // update front, right and up Vectors using the updated Eular angles
        self.update_camera_vectors();
    }

    // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        if self.zoom <= 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }

    /// Calculates the front vector from the Camera's (updated) Eular Angles
    fn update_camera_vectors(&mut self) {
        // Calculate the new front vector
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalize();
        // Also re-calculate the right and up vector
        self.right = self.front.cross(self.worldup).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn enable_mouse_movement(&mut self, enable: bool) {
        self.mouse_sensivity = if enable { SENSITIVTY } else { 0.0 };
    }
}
