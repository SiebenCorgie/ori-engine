use core::simple_scene_system::node;
use core::NodeMember;
use core::ReturnBoundInfo;
use na;
use nc;

pub struct Empty {
    pub name: String,
    bound: nc::bounding_volume::AABB<na::Point3<f32>>,
}

impl Empty{
    ///Returns an Empty with a 1x1x1 bound and `name` as name
    pub fn new(name: &str) -> Self{
        //Creating the box extend from the location, there might be a better way
        let min = na::Point3::new(0.5, 0.5, 0.5, );
        let max = na::Point3::new(0.5, 0.5, 0.5, );

        Empty{
            name: String::from(name),
            bound: nc::bounding_volume::AABB::new(min, max),
        }
    }
}

impl ReturnBoundInfo for Empty{
    ///return the max size of its bound
    fn get_bound_max(&self)-> na::Point3<f32>{
        self.bound.maxs().clone()
    }
    ///return the min size of its bound
    fn get_bound_min(&self)-> na::Point3<f32>{
        self.bound.mins().clone()
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
    fn get_bound_points(&self)-> Vec<na::Vector3<f32>>{
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

/*
impl NodeMember for Empty{


    ///Returns the name of this node
    fn get_name(&self) -> String{
        self.name.clone()
    }

    ///Returns the type of node this is
    fn get_content_type(&mut self) -> node::ContentTag{
        node::ContentTag::Empty
    }
}
*/
