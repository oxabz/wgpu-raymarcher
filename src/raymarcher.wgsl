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

struct ShapeCount{
    count:u32;
};

@group(1) @binding(0)
var<uniform> shape_count: ShapeCount;
@group(1) @binding(1)
var<storage> shapes: array<Shape>;
@group(1) @binding(2)
var<storage> spheres: array<Sphere>;


fn sphere_distance(a: vec3<f32>, b:Sphere)->f32{
    return distance(a,b.pos) - b.radius;
};

fn sphere_normal(point: vec3<f32>, sphere:Sphere)->vec3<f32>{
    return normalize(point - sphere.pos);
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

fn shape_normal(point: vec3<f32>, index:u32)-> vec3<f32>{
    let shape = shapes[index];
    var ret : vec3<f32>;
    switch(shape.shape_type){
        case 0u:{
            ret = sphere_normal(point, spheres[shape.index]);
        }
        default:{
            ret = vec3<f32>(0.0, 0.0, 0.0);
        }
    }
    return ret;
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
    let skip_shape = params.skip_shape;
    res.hit_shape = -1;
    loop {
        var closest_distance : f32= 9999999999.0;
        closest_shape = -1;
        for(var i:u32 = 0u; i < shape_count.count && threshold < closest_distance; i=i+1u){
            if i32(i) == skip_shape{continue;}
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
    let background_color = vec3<f32>(0.02, 0.0, 0.05);
    let light_direction = vec3<f32>(-1.0, -1.0, 0.4);

    let shape_count = 5u;

    let depth = 2.0;
    let ray_direction = normalize(vec3<f32>(-f32(i32(x)-width/2)/f32(width), -f32(i32(y)-height/2)/f32(height), depth));


    var ray : RayParams;
    ray.max_length = render_distance;
    ray.max_step = step_cap;
    ray.threshold = hit_threshold;
    ray.skip_shape = -1;

    var color: vec3<f32> = vec3<f32>(0.0,0.0,0.0);
    let hit = send_ray(vec3<f32>(0.0,0.0,0.0), ray_direction, ray);
    if(hit.hit_shape < 0){
        color = background_color;
    }else{
        // Fetching the right color
        color = vec3<f32>(shapes[hit.hit_shape].color);

        // Applying mat lighting
        let self_oclusion = max(0.0,-dot(light_direction, shape_normal(hit.hit_pos,u32(hit.hit_shape))));
        if (self_oclusion>0.00001){
            var light_ray : RayParams;
            light_ray.max_length = 20.0;
            light_ray.max_step = 200u;
            light_ray.threshold = 0.001;
            light_ray.skip_shape = hit.hit_shape;
            let light_hit = send_ray(hit.hit_pos, -light_direction, light_ray);
        }
        color = color * self_oclusion * f32(-min(0, light_hit.hit_shape));
    }
    textureStore(target_texture, vec2<i32>(i32(x),i32(y)), vec4<f32>(color,1.0));
}