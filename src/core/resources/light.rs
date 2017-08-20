
use na;
use nc;
use nc::shape::Cuboid;

use core::NodeMember;

///A Generic Point Light
pub struct LightPoint {
    pub name: String,
    intensity: f32,
    color: na::Vector3<f32>,

    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
}

///A stripped down version of a point light which can be passed to a shader
pub struct LightPointShaderInfo {
    intensity: f32,
    color: [f32; 3],
}

///A generic directional light i.e. a sun
pub struct LightDirectional {
    pub name: String,
    intensity: f32,
    color: na::Vector3<f32>,

    direction: na::Vector3<f32>,

    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
}

///A stripped down version of a directional light which can be passed to a shader
pub struct LightDirectionalShaderInfo {
    intensity: f32,
    color: [f32; 3],

    direction: [f32; 3],
}

///A generic spot light, like car lights or stage light
pub struct LightSpot {
    pub name: String,
    intensity: f32,
    color: na::Vector3<f32>,

    direction: na::Vector3<f32>,

    outer_radius: f32,
    inner_radius: f32,

    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
}

///A stripped down version of a spot light which can be passed to a shader
pub struct LightSpotShaderInfo {
    intensity: f32,
    color: [f32; 3],

    direction: [f32; 3],

    outer_radius: f32,
    inner_radius: f32,
}

///Custom PointLight implementation
impl LightPoint{
    ///Returns the Member with the passed `name`
    ///Special parameters light radius or color will have to be set later
    pub fn new(name: &str)->Self{
        //Creating the box extend from the location, there might be a better way
        let min = na::Point3::new(0.5, 0.5, 0.5, );
        let max = na::Point3::new(0.5, 0.5, 0.5, );

        LightPoint{
            name: String::from(name),
            intensity: 1.0,
            color: na::Vector3::new(1.0, 1.0, 1.0),

            bound: nc::bounding_volume::AABB::new(min, max),
        }
    }

    ///Returns this lught as its shader-useable instance
    pub fn as_shader_info(&self) -> LightPointShaderInfo{
        LightPointShaderInfo{
            intensity: self.intensity.clone(),
            color: self.color.into(),
        }
    }

    ///sets the lights intensity
    pub fn set_intensity(&mut self, new_itensity: f32){
        self.intensity = new_itensity;
    }

    ///returns the refernce to the intensity
    pub fn get_intensity(&mut self) -> &mut f32{
        &mut self.intensity
    }

    ///Sets its color, the value gets normalized, set the intensity via `set_intensity`
    pub fn set_color(&mut self, new_color: na::Vector3<f32>){
        self.color = new_color.normalize();
    }

    ///Returns the reference to its color
    pub fn get_color(&mut self) -> &mut na::Vector3<f32>{
        &mut self.color
    }
}

///Special functions for directional lights
impl LightDirectional{
    ///Returns the Member with the passed `name`
    ///Special parameters light radius or color will have to be set later
    pub fn new(name: &str)->Self{
        //Creating the box extend from the location, there might be a better way
        let min = na::Point3::new(0.5, 0.5, 0.5, );
        let max = na::Point3::new(0.5, 0.5, 0.5, );
        let direction = na::Vector3::new(1.0, 1.0, 1.0);

        LightDirectional{
            name: String::from(name),

            intensity: 1.0,
            color: na::Vector3::new(1.0, 1.0, 1.0),
            direction: direction,

            bound: nc::bounding_volume::AABB::new(min, max),
        }
    }

    ///Returns this lught as its shader-useable instance
    pub fn as_shader_info(&self) -> LightDirectionalShaderInfo{
        LightDirectionalShaderInfo{
            intensity: self.intensity.clone(),
            color: self.color.into(),
            direction: self.direction.into()
        }
    }

    ///Change the direction
    pub fn set_direction(&mut self, new_direction: na::Vector3<f32>){
        self.direction = new_direction;
    }

    ///Returns the direction reference
    pub fn get_direction(&mut self) -> &mut na::Vector3<f32>{
        &mut self.direction
    }

    ///set intensity
    pub fn set_intensity(&mut self, new_itensity: f32){
        self.intensity = new_itensity;
    }

    ///returns the refernce to the intensity
    pub fn get_intensity(&mut self) -> &mut f32{
        &mut self.intensity
    }

    ///Sets its color, the value gets normalized, set the intensity via `set_intensity`
    pub fn set_color(&mut self, new_color: na::Vector3<f32>){
        self.color = new_color.normalize();
    }

    ///Returns the reference to its color
    pub fn get_color(&mut self) -> &mut na::Vector3<f32>{
        &mut self.color
    }
}

///Special functions for the spot light
impl LightSpot{
    ///Returns the Member with the passed `name`
    ///Special parameters light radius or color will have to be set later
    pub fn new(name: &str)->Self{
        //Creating the box extend from the location, there might be a better way
        let min = na::Point3::new(0.5, 0.5, 0.5, );
        let max = na::Point3::new(0.5, 0.5, 0.5, );

        let direction = na::Vector3::new(1.0, 1.0, 1.0);
        let outer_radius = 50.0;
        let inner_radius = 40.0;

        LightSpot{
            name: String::from(name),
            intensity: 1.0,
            color: na::Vector3::new(1.0, 1.0, 1.0),
            direction: direction,

            outer_radius: outer_radius,
            inner_radius: inner_radius,

            bound: nc::bounding_volume::AABB::new(min, max),
        }
    }

    ///Returns this lught as its shader-useable instance
    pub fn as_shader_info(&self) -> LightSpotShaderInfo{
        LightSpotShaderInfo{
            intensity: self.intensity.clone(),
            color: self.color.into(),
            direction: self.direction.into(),
            outer_radius: self.outer_radius.clone(),
            inner_radius: self.inner_radius.clone(),
        }
    }

    ///Change the direction
    pub fn set_direction(&mut self, new_direction: na::Vector3<f32>){
        self.direction = new_direction;
    }

    ///Returns the direction reference
    pub fn get_direction(&mut self) -> &mut na::Vector3<f32>{
        &mut self.direction
    }

    ///set intensity
    pub fn set_intensity(&mut self, new_itensity: f32){
        self.intensity = new_itensity;
    }

    ///returns the refernce to the intensity
    pub fn get_intensity(&mut self) -> &mut f32{
        &mut self.intensity
    }

    ///Sets its color, the value gets normalized, set the intensity via `set_intensity`
    pub fn set_color(&mut self, new_color: na::Vector3<f32>){
        self.color = new_color.normalize();
    }

    ///Returns the reference to its color
    pub fn get_color(&mut self) -> &mut na::Vector3<f32>{
        &mut self.color
    }

    ///Sets the outer radius (point where the fallof ends) of this spot light
    pub fn set_outer_radius(&mut self, new_radius: f32){
        self.outer_radius = new_radius;
    }

    ///Returns the reference to the outer radius
    pub fn get_outer_radius(&mut self) -> &mut f32{
        &mut self.outer_radius
    }

    ///Sets the inner radius (point where the fallof starts) of this spot light
    pub fn set_inner_radius(&mut self, new_radius: f32){
        self.inner_radius = new_radius;
    }

    ///Returns the reference to the inner radius
    pub fn get_inner_radius(&mut self) -> &mut f32{
        &mut self.inner_radius
    }
}

///NodeMember for LightPoint
impl NodeMember for LightPoint{


    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>{
        self.bound.maxs()
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>{
        self.bound.mins()
    }
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>){
        let min = na::Point3::new(
            min[0],
            min[1],
            min[2]
        );

        let max = na::Point3::new(
            max[0],
            max[1],
            max[2]
        );

        self.bound = nc::bounding_volume::AABB::new(min, max);
    }

    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>{
        let mut return_vector = Vec::new();

        let b_min = self.bound.mins().clone();
        let b_max = self.bound.maxs().clone();

        ///low
        return_vector.push(na::Vector3::new(b_min[0], b_min[1], b_min[2])); //Low
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2])); //+x
        return_vector.push(na::Vector3::new(b_min[0], b_min[1] + b_max[1], b_min[2])); //+y
        return_vector.push(na::Vector3::new(b_min[0],  b_min[1], b_min[2] + b_max[2])); // +z
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2])); //+xy
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2] + b_max[2])); //+xz
        return_vector.push(na::Vector3::new(b_min[0] , b_min[1] + b_max[1], b_min[2] + b_max[1])); //+yz
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2] + b_max[2])); //+xyz

        return_vector
    }
}

///NodeMember for LightDirectional
impl NodeMember for LightDirectional{

    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>{
        self.bound.maxs()
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>{
        self.bound.mins()
    }
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>){
        let min = na::Point3::new(
            min[0],
            min[1],
            min[2]
        );

        let max = na::Point3::new(
            max[0],
            max[1],
            max[2]
        );

        self.bound = nc::bounding_volume::AABB::new(min, max);
    }

    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>{
        let mut return_vector = Vec::new();

        let b_min = self.bound.mins().clone();
        let b_max = self.bound.maxs().clone();

        ///low
        return_vector.push(na::Vector3::new(b_min[0], b_min[1], b_min[2])); //Low
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2])); //+x
        return_vector.push(na::Vector3::new(b_min[0], b_min[1] + b_max[1], b_min[2])); //+y
        return_vector.push(na::Vector3::new(b_min[0],  b_min[1], b_min[2] + b_max[2])); // +z
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2])); //+xy
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2] + b_max[2])); //+xz
        return_vector.push(na::Vector3::new(b_min[0] , b_min[1] + b_max[1], b_min[2] + b_max[1])); //+yz
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2] + b_max[2])); //+xyz

        return_vector
    }
}

///NodeMember for the LightSpot
impl NodeMember for LightSpot{

    ///return the max size of its bound
    fn get_bound_max(&self)-> &na::Point3<f32>{
        self.bound.maxs()
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> &na::Point3<f32>{
        self.bound.mins()
    }
    ///Sets the bound to the new values (in mesh space)
    fn set_bound(&mut self, min: na::Point3<f32>, max: na::Point3<f32>){
        let min = na::Point3::new(
            min[0],
            min[1],
            min[2]
        );

        let max = na::Point3::new(
            max[0],
            max[1],
            max[2]
        );

        self.bound = nc::bounding_volume::AABB::new(min, max);
    }

    ///Returns the vertices of the bounding mesh, good for debuging
    fn get_bound_points(&mut self)-> Vec<na::Vector3<f32>>{
        let mut return_vector = Vec::new();

        let b_min = self.bound.mins().clone();
        let b_max = self.bound.maxs().clone();

        ///low
        return_vector.push(na::Vector3::new(b_min[0], b_min[1], b_min[2])); //Low
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2])); //+x
        return_vector.push(na::Vector3::new(b_min[0], b_min[1] + b_max[1], b_min[2])); //+y
        return_vector.push(na::Vector3::new(b_min[0],  b_min[1], b_min[2] + b_max[2])); // +z
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2])); //+xy
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1], b_min[2] + b_max[2])); //+xz
        return_vector.push(na::Vector3::new(b_min[0] , b_min[1] + b_max[1], b_min[2] + b_max[1])); //+yz
        return_vector.push(na::Vector3::new(b_min[0] + b_max[0], b_min[1] + b_max[1], b_min[2] + b_max[2])); //+xyz

        return_vector
    }
}
