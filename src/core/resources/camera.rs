
use na::*;
use nc;
use nc::bounding_volume::HasBoundingVolume;
use std::f64::consts;
use std::sync::{Arc, Mutex};

use core::engine_settings;
use input::KeyMap;

use std::time::{Duration, Instant};

///Camera trait, use this to implement any type of camera
pub trait Camera {
    ///Creates a default camera
    fn new(settings: Arc<Mutex<engine_settings::EngineSettings>>, key_map: Arc<Mutex<KeyMap>>) -> Self;
    ///Calculates / Update the view
    fn update_view(&mut self);
    ///Returns the view matrix if needed
    fn get_view_matrix(&self) -> Matrix4<f32>;
    ///Returns the current direction of the camera
    fn get_direction(&self) -> Vector3<f32>;
    ///Set current direction
    fn set_direction(&mut self, new_direction: Vector3<f32>);
    ///Returns Position
    fn get_position(&self) -> Vector3<f32>;
    ///Set current position
    fn set_position(&mut self, new_pos: Vector3<f32>);
    ///Sets Fov on this camera
    fn set_fov(&mut self, new_fov: f32);
    ///Sets the far, and near planes of the frustum
    fn set_frustum_planes(&mut self, near: f32, far: f32);
    ///Returns the perspective matrix based on the window settings
    fn get_perspective(&self) -> Matrix4<f32>;
    ///Returns the bound of the view frustum
    fn get_frustum_bound(&self) -> nc::bounding_volume::AABB<Point3<f32>>;
}

///An example implementation
#[derive(Clone)]
pub struct DefaultCamera {
    //camera General
    pub cameraPos: Vector3<f32>,
    pub cameraFront: Vector3<f32>,
    pub cameraUp: Vector3<f32>,
    //Camera Rotation
    yaw: f32,
    pitch: f32,

    //Setting
    fov: f32,
    near_plane: f32,
    far_plane: f32,

    settings: Arc<Mutex<engine_settings::EngineSettings>>,
    key_map: Arc<Mutex<KeyMap>>,

    last_time: Instant,
}


impl Camera for DefaultCamera{
    fn new(
        settings: Arc<Mutex<engine_settings::EngineSettings>>,
        key_map: Arc<Mutex<KeyMap>>
    ) -> Self {
        //camera General
        let cameraPos = Vector3::new(0.0, 0.0, 0.0);
        let cameraFront = Vector3::new(0.0, 0.0, 1.0);
        let cameraUp = Vector3::new(0.0, 0.0, -1.0);
        //Camera Rotation
        let yaw: f32 = 0.0;
        let pitch: f32 = 0.0;

        let fov = 45.0;
        let near_plane = 0.1;
        let far_plane = 100.0;

        DefaultCamera {
            cameraPos: cameraPos,
            cameraFront: cameraFront,
            cameraUp: cameraUp,
            yaw: yaw,
            pitch: pitch,
            fov: fov,
            near_plane: near_plane,
            far_plane: far_plane,

            settings: settings,

            key_map: key_map,

            last_time: Instant::now(),
        }
    }

    ///Updates the camera view information
    fn update_view(&mut self){

        let delta_time: f32 ={
            //Get the time and / 1_000_000_000 for second
            (self.last_time.elapsed().subsec_nanos()) as f32
            /
            1_000_000_000.0
        };
        //and update "last time" for the next frame
        self.last_time = Instant::now();

        //println!("Delta_Seconds: {}", delta_time.clone() );

        //Corrected Camera Speed
        let camera_speed = 25.0 * delta_time;

        //copy us a easy key map
        let key_map_inst = {
            let glob_key_map_inst = self.key_map.clone();
            let glob_key_map_lck = glob_key_map_inst
            .lock()
            .expect("failed to lock global key map");

            let return_key_map = (*glob_key_map_lck).clone();
            return_key_map
        };


        //Input processing
        {
            if key_map_inst.a == true {
                self.cameraPos = self.cameraPos + (self.cameraFront.cross(&self.cameraUp).normalize()) * camera_speed;
            }
            if key_map_inst.w == true {
                self.cameraPos = self.cameraPos - self.cameraFront * camera_speed;
            }
            if key_map_inst.s == true {
                self.cameraPos = self.cameraPos + self.cameraFront * camera_speed;
            }
            if key_map_inst.d == true {
                self.cameraPos = self.cameraPos - (self.cameraFront.cross(&self.cameraUp).normalize()) * camera_speed;
            }
            if (key_map_inst.ctrl_l == true) | (key_map_inst.q == true) {
                self.cameraPos = self.cameraPos - Vector3::new(0.0, 0.0, camera_speed);
            }
            if (key_map_inst.shift_l == true) | (key_map_inst.e == true) {
                self.cameraPos = self.cameraPos + Vector3::new(0.0, 0.0, camera_speed);
            }
        }

        let sensitivity = 20.0;

        //Fixed camera gittering by slowing down so one integer delta = movement of
        // delta * sensitvity * time_delta * slowdown (virtual speed up)
        let virtual_speedup = 1.0;
        let x_offset: f32 = key_map_inst.mouse_delta_y as f32 * sensitivity * delta_time * virtual_speedup;
        let y_offset: f32 = key_map_inst.mouse_delta_x as f32 * sensitivity * delta_time * virtual_speedup;
        //needed to exchange these beacuse of the z-is-up system
        self.yaw += y_offset;
        self.pitch += x_offset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let mut front = Vector3::new(0.0, 0.0, 0.0);
        front.x = to_radians(self.yaw).cos() * to_radians(self.pitch).cos();
        front.z = to_radians(self.pitch).sin();
        front.y =  to_radians(self.yaw).sin() * to_radians(self.pitch).cos();
        self.cameraFront = front.normalize();

    }

    //Return view matrix as [[f32; 4]; 4]
    fn get_view_matrix(&self) -> Matrix4<f32> {

        let tmp_target = self.cameraPos - self.cameraFront;

        let view = Isometry3::look_at_rh(
            &Point3::new(self.cameraPos.x, self.cameraPos.y, self.cameraPos.z),
            &Point3::new(tmp_target.x, tmp_target.y, tmp_target.z),
            &Vector3::new(self.cameraUp.x, self.cameraUp.y, self.cameraUp.z)
        ).to_homogeneous();
        view
    }

    ///Returns the direction the camera is facing
    fn get_direction(&self) -> Vector3<f32> {
        self.cameraFront
    }

    ///Sets the direction of the camera to a Vector3<f32>
    fn set_direction(&mut self, new_direction: Vector3<f32>){
        self.cameraFront = new_direction.normalize();
    }

    ///Returns the position of the camera as Vector3<f32>
    fn get_position(&self) -> Vector3<f32> {
        self.cameraPos
    }

    ///Sets the position
    fn set_position(&mut self, new_pos: Vector3<f32>){
        self.cameraPos = new_pos;
    }

    ///Sets the field of view for this camera
    fn set_fov(&mut self, new_fov: f32){
        self.fov = new_fov;
    }

    ///Sets the frustum far and near plane
    fn set_frustum_planes(&mut self, near: f32, far: f32) {
        self.far_plane = far;
        self.near_plane = near;
    }

    //Calculates the perspective based on the engine and camera settings
    fn get_perspective(&self) -> Matrix4<f32>{
        //TODO update the perspective to use current engine settings
        let (mut width, mut height) = (800, 600);
        {
            let engine_settings_inst = self.settings.clone();
            let mut engine_settings_lck = engine_settings_inst.lock().expect("Faield to lock settings");

            width = (*engine_settings_lck).get_dimensions()[0];
            height = (*engine_settings_lck).get_dimensions()[1];
        }


        Perspective3::new(
            (width as f32 / height as f32), //Aspect
            to_radians(self.fov),   //fov
            self.near_plane,
            self.far_plane
        ).to_homogeneous()
    }

    ///Returns the frustum bound of this camera as a AABB
    fn get_frustum_bound(&self) -> nc::bounding_volume::AABB<Point3<f32>>{

        let (mut width, mut height) = (800, 600);
        {
            let engine_settings_inst = self.settings.clone();
            let mut engine_settings_lck = engine_settings_inst.lock().expect("Faield to lock settings");

            width = (*engine_settings_lck).get_dimensions()[0];
            height = (*engine_settings_lck).get_dimensions()[1];
        }


        //Reference: http://www.lighthouse3d.com/tutorials/view-frustum-culling/geometric-approach-extracting-the-planes/
        //NOTE see commend for computing width/height of far/near

        let camera_right = self.cameraUp.cross(&self.cameraFront);
        //follows: Wnear/2 = tan(fov/2)*nd; // tan – is sin/cos
        let width_near = ((to_radians(self.fov/2.0)).tan() * self.near_plane) * 2.0;
        //follows Wfar/2 = tan((ALPHA / aspect ratio)/2)*nd; // tan – is sin/cos
        let height_near = ((to_radians(((self.fov/(width as f32/height as f32))/2.0))).tan() * self.near_plane) * 2.0;
        //follows: Wnear/2 = tan(fov/2)*nd; // tan – is sin/cos
        let width_far = ((to_radians(self.fov/2.0)).tan() * self.far_plane) * 2.0;
        //follows Wfar/2 = tan((ALPHA / aspect ratio)/2)*nd; // tan – is sin/cos
        let height_far = ((to_radians(((self.fov/(width as f32/height as f32))/2.0))).tan() * self.far_plane) * 2.0;

        let p = self.cameraPos.clone();
        let d = self.cameraFront.clone();
        let up = self.cameraUp.clone();

        //compute the points of this frustum
        let fc = p + d * self.far_plane;

        let ftl = fc + (up * height_far/2.0) - (camera_right * width_far/2.0);
        let ftr = fc + (up * height_far/2.0) + (camera_right * width_far/2.0);
        let fbl = fc - (up * height_far/2.0) - (camera_right * width_far/2.0);
        let fbr = fc - (up * height_far/2.0) + (camera_right * width_far/2.0);

        let nc = p + d * self.near_plane;

        let ntl = nc + (up * height_near/2.0) - (camera_right * width_near/2.0);
        let ntr = nc + (up * height_near/2.0) + (camera_right * width_near/2.0);
        let nbl = nc - (up * height_near/2.0) - (camera_right * width_near/2.0);
        let nbr = nc - (up * height_near/2.0) + (camera_right * width_near/2.0);

        ///Convert to points in 3d space
        let p_ftl = Point3::new(ftl.x, ftl.y, ftl.z);
        let p_ftr = Point3::new(ftr.x, ftr.y, ftr.z);
        let p_fbl = Point3::new(fbl.x, fbl.y, fbl.z);
        let p_fbr = Point3::new(fbr.x, fbr.y, fbr.z);

        let p_ntl = Point3::new(ntl.x, ntl.y, ntl.z);
        let p_ntr = Point3::new(ntr.x, ntr.y, ntr.z);
        let p_nbl = Point3::new(nbl.x, nbl.y, nbl.z);
        let p_nbr = Point3::new(nbr.x, nbr.y, nbr.z);

        let point_groupe = vec!(p_ftl,p_ftr,p_fbl,p_fbr,p_ntl,p_ntr,p_nbl,p_nbr);

        let frustum_shape = nc::shape::ConvexHull::new(point_groupe);
        //hope that it works o.o
        frustum_shape.bounding_volume(&geometry::Isometry3::identity())
    }
}




//Helper function for calculating the view
fn to_radians(degree: f32) -> f32 {
    degree * (consts::PI / 180.0) as f32
}
