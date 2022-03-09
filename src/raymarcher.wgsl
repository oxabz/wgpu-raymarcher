struct Sphere{ //align(16)
    pos : vec3<f32>; //offset(0) align(16) size(12)
    radius : f32; // offset(12) align(4) size(4)
};

struct Shape{ //align(16)
    color: vec3<f32>; //offset(0) align(16) size(12)
    shape_type: u32; //offset(12) align(4) size(4)
    index: u32; //offset(16) align(4) size(4)
    //padding(12)
};

@group(0) @binding(0)
var target_texture: texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(0)
var<storage> shapes: array<Shape>;
@group(1) @binding(1)
var<storage> spheres: array<Sphere>;


fn sphere_distance(a: vec3<f32>, b:Sphere)->f32{
    return distance(a,b.pos) - b.radius;
};

fn sphere_lighting(point: vec3<f32>, light_direction:vec3<f32>, sphere:Sphere)->f32{
    return max(dot(normalize(light_direction),normalize(point-sphere.pos)),0.0);
};

fn shape_distance(pos: vec3<f32>, index:u32)-> f32{
    let shape = shapes[index];
    var ret : f32 = 0.0;
    switch(shape.shape_type){
        case 0u:{
            ret = sphere_distance(pos, spheres[shape.index]);
        }
        default:{
            ret = 99999999.0;
        }
    }
    return ret;
};

fn shape_lighting(point: vec3<f32>, light_direction:vec3<f32>, index:u32)-> f32{
    let shape = shapes[index];
    var ret : f32 = 0.0;
    switch(shape.shape_type){
        case 0u:{
            ret = sphere_lighting(point, light_direction, spheres[shape.index]);
        }
        default:{
            ret = 0.0;
        }
    }
    return ret;
};

struct RayParams{
    max_length: f32;
    max_step: u32;
    threshold: f32;
};

struct Hit{
    hit_shape: i32;
    step_count: u32;
    hit_pos: vec3<f32>;
    ray_length: f32;
};

fn send_ray(origin:vec3<f32>, direction:vec3<f32>, params: RayParams)->Hit{
    var res: Hit;
    var step_count = 0u;
    var ray_length = 0.0;
    var ray_pos = origin;
    var closest_shape = -1;
    //Params
    let threshold = params.threshold;
    let max_step = params.max_step;
    let max_length = params.max_length;
    res.hit_shape = -1;
    loop {
        var closest_distance : f32= 9999999999.0;
        closest_shape = -1;
        for(var i:u32 = 0u; i < 5u && threshold < closest_distance; i=i+1u){
            let shape_dist = shape_distance(ray_pos, i);
            if(closest_distance > shape_dist){
                closest_shape = i32(i);
                closest_distance = shape_dist;
            }
        }
        ray_pos += direction * closest_distance;
        ray_length +=  closest_distance;
        step_count += 1u;

        if !( step_count < max_step
                && threshold < closest_distance
                && ray_length < max_length ){
            break;
        }
    }

    if(threshold > closest_distance){
        res.hit_shape = closest_shape;
    }
    res.ray_length = ray_length;
    res.step_count = step_count;
    res.hit_pos = ray_pos;
    return res;
};

@stage(compute) @workgroup_size(16,16)
fn render(@builtin(global_invocation_id) global_invocation_id: vec3<u32>){
    let target_size = textureDimensions(target_texture);
    let x = global_invocation_id.x;
    let y = global_invocation_id.y;
    let width = target_size[0];
    let height = target_size[1];

    let step_cap = 100u;
    let render_distance = 2000.0;
    let hit_threshold = 0.01;
    let background_color = vec4<f32>(0.05, 0.0, 0.1, 1.0);
    let light_direction = vec3<f32>(-1.0, -1.0, -0.2);

    let shape_count = 5u;

    let depth = 2.0;
    let ray_direction = normalize(vec3<f32>(-f32(i32(x)-width/2)/f32(width), -f32(i32(y)-height/2)/f32(height), depth));


    var ray : RayParams;
    ray.max_length = render_distance;
    ray.max_step = step_cap;
    ray.threshold = hit_threshold;

    var color: vec4<f32> = vec4<f32>(0.0,0.0,0.0,0.0);
    let hit = send_ray(vec3<f32>(0.0,0.0,0.0), ray_direction, ray);

    if(hit.hit_shape < 0){
        color = background_color;
    }else{
        color = vec4<f32>(shapes[hit.hit_shape].color, 1.0);
    }
    textureStore(target_texture, vec2<i32>(i32(x),i32(y)), color);
}