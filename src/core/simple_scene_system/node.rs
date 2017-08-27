///Defines a node and its behavoir int the tree

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

use na;
use nc;
use nc::bounding_volume::BoundingVolume;


use rt_error;
use core;
use core::NodeMember;
use core::simple_scene_system::node_member;
use core::resources::mesh;
use core::resources::light;
use core::resources::empty;
use core::resources::camera;
use core::resources::camera::Camera;
///All possible types of content a Node can hold.
///This enum as well as all `match` sequenzes in the `impl Node for GenereicNode` have to be
///Changed in order to apply a new type


#[derive(Clone)]
pub enum ContentTag {
    StaticMesh,
    DynamicMesh,
    LightPoint,
    LightDirectional,
    LightSpot,
    Empty,
    Custom,
}

///The normal Node of this Scene Tree
///
/// *Why a Vector and no HashMap?*
/// I decided to use a Vector of Structs where the name is in the struct mainly because of
/// performance reasons. With small datasets (5-100 entries) the HashMap is faster and provides
/// some comfort (you can store the name as a String as key value). However, if you have bigger
/// datasets (over 1,000,000) the vector is MUCH faster, as specially in `--release` mode.
#[derive(Clone)]
pub struct GenericNode {

    children: Vec<GenericNode>,
    ///There is a difference between a `Node`'s name and its `content` name
    pub name: String,
    ///Location of this node in world space
    location: na::Vector3<f32>,
    rotation: na::Rotation3<f32>,
    scale: na::Vector3<f32>,
    ///The bounds of this note, takes the own `content` bound as well as the max and min values of
    ///all its children into consideration
    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
    ///The content is a contaier from the `ContentTypes` type which can hold any implemented
    ///Enum value
    content: Arc<NodeMember + Send + Sync>,
    content_tag: ContentTag,
}

///Implementation of the Node trait for Generic node
impl GenericNode{
    ///Creates a new, empty node
    pub fn new_empty(name: &str)-> Self{

        let mut tmp_bound = nc::bounding_volume::AABB::new(na::Point3::new(0.0, 0.0, 0.0), na::Point3::new(1.0, 1.0, 1.0));

        GenericNode{
            children: Vec::new(),
            name: String::from(name),
            location: na::Vector3::new(0.0, 0.0, 0.0),
            rotation: na::Rotation3::from_axis_angle(&na::Vector3::x_axis(),0.0),
            scale: na::Vector3::new(0.0, 0.0, 0.0),

            bound: tmp_bound,

            content: Arc::new(
                node_member::SimpleNodeMember::from_empty(
                    Arc::new(
                        Mutex::new(
                            empty::Empty::new("Empty")
                        )
                    )
                )
            ),

            content_tag: ContentTag::Empty,
        }
    }

    ///Should return an node
    pub fn new(name: &str, content: Arc<NodeMember + Send + Sync>)->Self{

        let mut tmp_bound = nc::bounding_volume::AABB::new(na::Point3::new(0.0, 0.0, 0.0), na::Point3::new(1.0, 1.0, 1.0));
        //Building node bound from mesh

        let mut imported_content_tag = content.get_content_type().clone();

        // and bound
        {
            tmp_bound = nc::bounding_volume::AABB::new(
                content.get_bound_min().clone(), content.get_bound_max().clone()
            );
        }

        GenericNode{
            children: Vec::new(),
            name: String::from(name),
            location: na::Vector3::new(0.0, 0.0, 0.0),
            rotation: na::Rotation3::from_axis_angle(&na::Vector3::x_axis(),0.0),
            scale: na::Vector3::new(0.0, 0.0, 0.0),

            bound: tmp_bound,

            content: content,
            content_tag: imported_content_tag,
        }
    }

    ///should release a node from memory
    pub fn release(&mut self, name: &str){
        //Only remove if in Vec
        for i in 0..self.children.len(){
            if self.children[i].name == String::from(name.clone()){
                self.children.remove(i);
            }
        }
    }

    ///Destroy this node and all its children
    pub fn destroy(&mut self){
        //First delete all children
        for i in self.children.iter_mut(){
            i.destroy();
        }
        //then self
        drop(self);
    }

    ///Adds a child node to this node
    pub fn add_child(&mut self, child: Arc<NodeMember + Send + Sync>){

        //find out the right name
        let name: String = child.get_name();

        let tmp_name: &str = &name.to_string();
        //deside how to add, based on type
        let tmp_child = GenericNode::new(tmp_name, child);
        self.children.push(tmp_child);

    }

    ///Adds a already prepared node, good for merging different trees
    pub fn add_node(&mut self, node: GenericNode){
        //Add it based on its own name
        self.children.push(node);
    }

    ///Adds a `node_to_add` as a child to a node with `node_name` as name
    ///good merging node trees at a specific point
    pub fn add_node_at_sub_node(&mut self, node_name: &str,  node_to_add: GenericNode){
        let node = self.get_node(node_name);
        match node{
            None => rt_error("NODE: ", "Could not find subnode while trying to add"),
            Some(nd)=> nd.add_node(node_to_add),
        }

    }

    ///Returns a node with this name (the name of a node is pulled from the name of its content)
    pub fn get_node(&mut self, node_name: &str)-> Option<&mut Self>{

        let mut tmp_return: Option<&mut Self> = None;

        if self.name == String::from(node_name.clone()){
            return Some(self);
        }
        //nothing new: if it's not self, it cycles trough the children
        match tmp_return{
            //if something was found return it
            Some(_) => {},
            None=>{
                for i in self.children.iter_mut(){
                    match tmp_return{
                        None=> tmp_return = i.get_node(node_name.clone()),
                        Some(value)=> return Some(value),
                    }
                }
            }
        }
        //if the function comes here tmp_return will be `None`
        tmp_return
    }

    ///Returns the transform matrix
    pub fn get_transform_matrix(&self) -> na::Matrix4<f32>{

        let translation = na::Translation::from_vector(self.location);
        let mut isometry = na::Isometry3::identity();

        isometry.append_translation_mut(&translation);

    //    println!("Returning Matrix: {:?}", isometry.to_homogeneous());

        isometry.to_homogeneous()

    }

    ///Translates this node by `translation` and all its children
    pub fn translate(&mut self, translation: na::Vector3<f32>){
        //for self
        println!("OldSelfIs: {:?}", self.location);
        self.location = translation;
        //for all children
        for child in self.children.iter_mut(){
            child.translate(translation);
        }
        println!("Translated by {:?}", translation);
        println!("New Self is: {:?}", self.location);
    }

    ///Sets the location to `location` and changes the location of all its children as well
    pub fn set_location(&mut self, location: na::Vector3<f32>){
        //get the difference of the current and the new position
        let difference = self.location - location;
        //Set it for self
        self.translate(difference);
        //And for all children
        for child in self.children.iter_mut(){
            child.translate(difference);
        }
    }



    ///Returns a mesh from childs with this name
    pub fn get_mesh(&mut self, name: &str)-> Option<Arc<Mutex<core::resources::mesh::Mesh>>>{

        let mut result_value: Option<Arc<Mutex<core::resources::mesh::Mesh>>> = None;

        //first have a look if self's content is the searched one
        //NOTE if the searched value is somewhere in the tree, this should return
        //NOTE Some(value) once

        let content_type = self.content_tag.clone();

        match content_type{
            ContentTag::StaticMesh | ContentTag::DynamicMesh=> {
                if self.content.get_name() == String::from(name.clone()){

                    //Have a look for a dynamic mesh
                    match self.content.get_static_mesh(){
                        Some(mesh) => result_value = Some(mesh),
                        None => {},
                    }
                    //if it wasnt a staic mesh, have a look for a dynamic one
                    if result_value.is_none() {
                        match self.content.get_dynamic_mesh(){
                            Some(mesh) => result_value = Some(mesh),
                            None => {},
                        }
                    }
                }
            }
            //if not selfs content search in children
            _=>{}
        }

        //Have a look if we found it in the content
        //if not search in childs
        match result_value{
            //if we already found somthing, don't do anything
            Some(_)=> {},
            None=> {
                //Cycling though the children till we got any Some(x)
                for i in self.children.iter_mut(){
                    //make sure we dont overwrite the right value with a none of the next value
                    match result_value{
                        None=> result_value = i.get_mesh(name.clone()),
                        //if tmp holds something overwerite the result_value
                        //the early return makes sure we dont overwrite our found falue with another
                        //none
                        Some(value)=> return Some(value),
                    }

                }
            }

        }
        result_value
    }

    ///Returns the first light point with this name
    pub fn get_light_point(&mut self, name: &str) -> Option<Arc<Mutex<core::resources::light::LightPoint>>>{
        let mut result_value: Option<Arc<Mutex<core::resources::light::LightPoint>>> = None;

        //first have a look if self's content is the searched one
        //NOTE if the searched value is somewhere in the tree, this should return
        //NOTE Some(value) once

        let content_type = self.content_tag.clone();


        match content_type{
            ContentTag::LightPoint => {
                if self.content.get_name() == String::from(name.clone()){

                    //Have a look for a light
                    match self.content.get_light_point(){
                        Some(light) => result_value = Some(light),
                        None => {},
                    }

                }
            }
            //if not selfs content search in children
            _=>{}
        }

        //Have a look if we found it in the content
        //if not search in childs
        match result_value{
            //if we already found somthing, don't do anything
            Some(_)=> {},
            None=> {
                //Cycling though the children till we got any Some(x)
                for i in self.children.iter_mut(){
                    //make sure we dont overwrite the right value with a none of the next value
                    match result_value{
                        None=> result_value = i.get_light_point(name.clone()),
                        //if tmp holds something overwerite the result_value
                        //the early return makes sure we dont overwrite our found falue with another
                        //none
                        Some(value)=> return Some(value),
                    }

                }
            }

        }
        result_value
    }

    ///Returns the first light directional with this name
    pub fn get_light_directional(&mut self, name: &str) -> Option<Arc<Mutex<core::resources::light::LightDirectional>>>{
        let mut result_value: Option<Arc<Mutex<core::resources::light::LightDirectional>>> = None;

        //first have a look if self's content is the searched one
        //NOTE if the searched value is somewhere in the tree, this should return
        //NOTE Some(value) once

        let content_type = self.content_tag.clone();


        match content_type{
            ContentTag::LightDirectional => {
                if self.content.get_name() == String::from(name.clone()){

                    //Have a look for a light
                    match self.content.get_light_directional(){
                        Some(light) => result_value = Some(light),
                        None => {},
                    }

                }
            }
            //if not selfs content search in children
            _=>{}
        }

        //Have a look if we found it in the content
        //if not search in childs
        match result_value{
            //if we already found somthing, don't do anything
            Some(_)=> {},
            None=> {
                //Cycling though the children till we got any Some(x)
                for i in self.children.iter_mut(){
                    //make sure we dont overwrite the right value with a none of the next value
                    match result_value{
                        None=> result_value = i.get_light_directional(name.clone()),
                        //if tmp holds something overwerite the result_value
                        //the early return makes sure we dont overwrite our found falue with another
                        //none
                        Some(value)=> return Some(value),
                    }

                }
            }

        }
        result_value
    }

    //TODO get specific light spot

    ///Returns all meshes in view frustum
    pub fn get_meshes_in_frustum(&mut self, camera: &camera::DefaultCamera) -> Vec<Arc<Mutex<mesh::Mesh>>>{

        let camera_volume = camera.get_frustum_bound().clone();

        let mut return_vector = Vec::new();


        match self.content.get_static_mesh(){
            //if selfs content is a mesh, check the bound
            Some(ref static_mesh) => {
                //check if self is in bound
                if self.bound.intersects(&camera_volume){
                    return_vector.push(static_mesh.clone());
                //if self is not in bound we can return as there won't be any lower mesh in there
                }else{
                    return return_vector;
                }
            },
            //if self is no mesh, just check the bound
            None => {
                //check if self is in bound
                if self.bound.intersects(&camera_volume){
                    //go further down the tree
                //if self is not in bound we can return as there won't be any lower mesh in there
                }else{
                    return return_vector;
                }
            }
        }


        //if not already return because the bound is too small, check the children
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_meshes_in_volume(&camera_volume));
        }
        return_vector
    }

    ///checks for bounds in a volume, view frustum or maybe for a locale collision check
    pub fn get_meshes_in_volume(&mut self, volume: &nc::bounding_volume::AABB<na::Point3<f32>>) -> Vec<Arc<Mutex<mesh::Mesh>>>{

        let mut return_vector = Vec::new();



        match self.content.get_static_mesh(){
            //if selfs content is a mesh, check the bound
            Some(ref static_mesh) => {
                //check if self is in bound
                if self.bound.intersects(&volume){
                    return_vector.push(static_mesh.clone());
                //if self is not in bound we can return as there won't be any lower mesh in there
                }else{
                    return return_vector;
                }
            },
            //if self is no mesh, just check the bound
            None => {
                //check if self is in bound
                if self.bound.intersects(&volume){
                    //go further down the tree
                //if self is not in bound we can return as there won't be any lower mesh in there
                }else{
                    return return_vector;
                }
            }
        }


        //if not already return because the bound is too small, check the children
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_meshes_in_volume(&volume));
        }
        return_vector
    }

    ///Gets all meshes from this node down
    pub fn get_all_meshes(&mut self) -> Vec<(Arc<Mutex<mesh::Mesh>>, na::Matrix4<f32>)>{
        let mut return_vector = Vec::new();



        let opt_mesh = self.content.get_static_mesh();
        let opt_dyn_mesh = self.content.get_static_mesh();

        //Test self
        match opt_mesh{
            Some(mesh) => return_vector.push((mesh.clone(), self.get_transform_matrix())),
            _ => {},
        }

        match opt_dyn_mesh{
            Some(mesh) => return_vector.push((mesh.clone(), self.get_transform_matrix())),
            _ => {},
        }

        //println!("Returning tanslation of: {:?}", self.get_transform_matrix());

        //Go down the tree
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_all_meshes());
        }
        return_vector
    }

    ///Gets all LightPoint from this node down
    pub fn get_all_light_points(&mut self) -> Vec<Arc<Mutex<core::resources::light::LightPoint>>>{
        let mut return_vector = Vec::new();

        //Check self
        match self.content.get_light_point(){
            Some(light) => return_vector.push(light.clone()),
            _ => {},
        }

        //Go down the tree
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_all_light_points());
        }
        return_vector
    }

    ///Gets all LightDir from this node down
    pub fn get_all_light_directionals(&mut self) -> Vec<Arc<Mutex<core::resources::light::LightDirectional>>>{
        let mut return_vector = Vec::new();
        //Check self
        match self.content.get_light_directional(){
            Some(light) => return_vector.push(light.clone()),
            _ => {},
        }
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_all_light_directionals());
        }
        return_vector
    }

    ///Gets all LightSpot from this node down
    pub fn get_all_light_spots(&mut self) -> Vec<Arc<Mutex<core::resources::light::LightSpot>>>{
        let mut return_vector = Vec::new();
        //Check self
        match self.content.get_light_spot(){
            Some(light) => return_vector.push(light.clone()),
            _ => {},
        }
        for i in self.children.iter_mut(){
            return_vector.append(&mut i.get_all_light_spots());
        }
        return_vector
    }

    ///Returns the bound of `content` in self as mutable reference
    pub fn get_bound(&mut self) -> &mut nc::bounding_volume::AABB<na::Point3<f32>>{
        &mut self.bound
    }

    ///Returns the maximum bound values from this node down
    pub fn get_bound_max(&mut self) -> na::Point3<f32>{

        let mut return_max = self.bound.maxs().clone();

        //Compare self with the children an their children etc.
        for i in self.children.iter_mut(){
            let child_max = i.get_bound_max();

            //Comapare per axis    X
            if child_max[0] > return_max[0]{
                return_max[0] = child_max[0].clone();
            }

            //Comapare per axis    Y
            if child_max[1] > return_max[1]{
                return_max[1] = child_max[1].clone();
            }

            //Comapare per axis    Z
            if child_max[2] > return_max[2]{
                return_max[2] = child_max[2].clone();
            }
        }
        //Retrurn the smallest values

        return_max
    }

    ///Returns the min bound values from this node down
    ///Compares per axis
    pub fn get_bound_min(&mut self) -> na::Point3<f32>{

        let mut return_min = self.bound.mins().clone();

        //Compare self with the children an their children etc.
        for i in self.children.iter_mut(){
            let child_min = i.get_bound_min();

            //Comapare per axis    X
            if child_min[0] < return_min[0]{
                return_min[0] = child_min[0].clone();
            }

            //Comapare per axis    Y
            if child_min[1] < return_min[1]{
                return_min[1] = child_min[1].clone();
            }

            //Comapare per axis    Z
            if child_min[2] < return_min[2]{
                return_min[2] = child_min[2].clone();
            }
        }
        //Retrurn the smallest values
        return_min
    }

    ///Rebuilds bounds for this node down
    ///should usually be applied to the root node only not
    ///if you are sure that the new bound doesnt extend the old parent bound of a node
    pub fn rebuild_bounds(&mut self){

        //First rebuild the bounds of all sub children
        for k in self.children.iter_mut(){
            k.rebuild_bounds();
        }
        //Then get the new max and min values
        let new_min = self.get_bound_min();
        let new_max = self.get_bound_max();
        //and use them for own bound
        self.bound = nc::bounding_volume::AABB::new(new_min, new_max);
    }

    ///prints a visual representation of the tree to the terminal
    pub fn print_member(&self, depth: u32){
        //add space
        for _ in 0..depth{
            print!("    ", );
        }
        //print name behind space
        //as well as its bound for debug reason
        print!("NAME: {} BOUNDS: ", self.name);
        print!("min: [{}, {}, {}]   max: [{}, {}, {}], Location: [{},{},{}] \n",
            self.bound.mins()[0],
            self.bound.mins()[1],
            self.bound.mins()[2],

            self.bound.maxs()[0],
            self.bound.maxs()[1],
            self.bound.maxs()[2],
            self.location.x,
            self.location.y,
            self.location.z,
        );
        for i in self.children.iter(){
            i.print_member(depth + 1);
        }
    }
}
