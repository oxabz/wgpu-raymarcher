struct Sphere{ //align(16)
    pos : vec3<f32>; //offset(0) align(16) size(12)
    radius : f32; // offset(12) align(4) size(4)
};

struct Cuboid{ //align(16)
    pos : vec3<f32>; //offset(0) align(16) size(12)
    //pad 4
    scale : vec3<f32>; // offset(16) align(16) size(12)
    //pad 4
    rotation : mat3x3<f32>; // ofset(32) align(16) size(48)
};

struct Composite{ //align(16)
    a:u32;
    b:u32;
    t:u32;
    alpha:f32;
};

struct Shape{ //align(16)
    color: vec3<f32>; //offset(0) align(16) size(12)
    index: u32; //offset(12) align(4) size(4)
    shape_type: u32; //offset(16) align(4) size(4)
    reflectivity: f32; //offset(20) align(4) size(4)
    visible:u32;
    //padding(8)
};

struct Camera{
    ray_dir : mat3x3<f32>;
    position: vec3<f32>;
    ratio : f32;
    depth : f32;
};

struct DistRes{
    distance:f32;
    index:u32;
};

struct RayParams{
    max_length: f32;
    max_step: u32;
    threshold: f32;
    skip_shape: i32;
};

struct Hit{
    hit_shape: i32;
    step_count: u32;
    hit_pos: vec3<f32>;
    ray_length: f32;
    min_distance: f32;
};

struct ShapeCount{
    count:u32;
};