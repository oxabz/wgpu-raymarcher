struct Sphere{ //align(16)
    pos : vec3<f32>; //offset(0) align(16) size(12)
    radius : f32; // offset(12) align(4) size(4)
};

struct Cuboid{ //align(16)
    pos : vec3<f32>; //offset(0) align(16) size(12)
    //pad 4
    scale : vec3<f32>; // offset(16) align(16) size(12)
    //pad 4
};

struct Shape{ //align(16)
    color: vec3<f32>; //offset(0) align(16) size(12)
    index: u32; //offset(12) align(4) size(4)
    shape_type: u32; //offset(16) align(4) size(4)
    reflectivity: f32; //offset(20) align(4) size(4)
    //padding(8)
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
@group(1) @binding(3)
var<storage> cuboids: array<Cuboid>;

struct Camera{
    ray_dir : mat3x3<f32>;
    position: vec3<f32>;
    ratio : f32;
    depth : f32;
};

@group(2) @binding(0)
var<uniform> camera: Camera;

fn cube_distance(a:vec3<f32>, b:Cuboid)->f32{
    let a_centered = a-b.pos;
    let half_size = b.scale/2.0;
    let offset = abs(a_centered)-half_size;
    var sign = 1.0;
    if offset[0]<0.0 && offset[1]<0.0 && offset[2]<0.0{
        return -length(offset);
    }else{
        return length(max(offset, vec3<f32>(0.0,0.0,0.0)));
    }
};

fn cube_normal(a:vec3<f32>, b:Cuboid)->vec3<f32>{
    let a_centered = a-b.pos;
    let half_size = b.scale/2.0;
    if(a_centered[0] >= half_size[0]){
        return vec3<f32>(1.0,0.0,0.0);
    }
    if(a_centered[0] <= -half_size[0]){
        return vec3<f32>(-1.0,0.0,0.0);
    }
    if(a_centered[1] >= half_size[1]){
        return vec3<f32>(0.0,1.0,0.0);
    }
    if(a_centered[1] <= -half_size[1]){
        return vec3<f32>(0.0,-1.0,0.0);
    }
    if(a_centered[2] >= half_size[2]){
        return vec3<f32>(0.0,0.0,1.0);
    }
    else{
        return vec3<f32>(0.0,0.0,-1.0);
    }
};

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
        case 1u:{
            ret = cube_distance(pos, cuboids[shape.index]);
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
        case 1u:{
            ret = cube_normal(point, cuboids[shape.index]);
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
    min_distance: f32;
};

fn send_ray(origin:vec3<f32>, direction:vec3<f32>, params: RayParams)->Hit{
    var res: Hit;
    var step_count = 0u;
    var ray_length = 0.0;
    var ray_pos = origin;
    var closest_shape = -1;
    var closest_distance_g = 9999999999.0;
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
        if (closest_distance < closest_distance_g){
            closest_distance_g = closest_distance;
        }

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
    res.min_distance = closest_distance_g;
    return res;
};

fn vcos(a:vec3<f32>, b:vec3<f32>) -> f32{
    return dot(a,b) /(length(a)*length(b));
};

fn reflection(incoming:vec3<f32>, normal:vec3<f32>)->vec3<f32>{
    return -2.0*dot(incoming,normal)/dot(normal,normal)*normal+incoming;
};

@stage(compute) @workgroup_size(16,16)
fn render(@builtin(global_invocation_id) global_invocation_id: vec3<u32>){
    let target_size = textureDimensions(target_texture);
    let x = global_invocation_id.x;
    let y = global_invocation_id.y;
    let width = f32(target_size[0]);
    let height = f32(target_size[1]);

    let step_cap = 100u;
    let render_distance = 100.0;
    let shadow_blur = 5.0;
    let hit_threshold = 0.01;
    let background_color = vec3<f32>(0.005, 0.0, 0.03);
    let light_direction = vec3<f32>(-1.0, -1.0, 0.4);
    let reflection_rays = 10u;
    let reflection_threshold = 0.01;

    let shape_count = 5u;

    let depth = 2.0;
    var ray_direction = normalize(vec3<f32>((-f32(x) / width + 0.5) * camera.ratio, (-f32(y) / height + 0.5), camera.depth) * camera.ray_dir);

    var ray : RayParams;
    ray.max_length = render_distance;
    ray.max_step = step_cap;
    ray.threshold = hit_threshold;
    ray.skip_shape = -1;

    var color: vec3<f32> = vec3<f32>(0.0,0.0,0.0);
    var color_weight:f32 = 1.0;
    var latest_hit:Hit;
    latest_hit.hit_pos = camera.position;
    latest_hit.hit_shape = -1;
    var bounce_count = 0u;
    loop {
        if (bounce_count >= reflection_rays || color_weight<reflection_threshold){
            color = color * (1.0/(1.0-color_weight));
            break;
        }
        ray.skip_shape = latest_hit.hit_shape;
        latest_hit = send_ray(latest_hit.hit_pos, ray_direction, ray);
        if (latest_hit.hit_shape < 0){
            color += background_color * color_weight;
            break;
        }
        var s = shapes[latest_hit.hit_shape];
        let reflectivity = s.reflectivity;
        let matness = 1.0 - reflectivity;

        var matcolor = vec3<f32>(shapes[latest_hit.hit_shape].color);

        let normal = shape_normal(latest_hit.hit_pos,u32(latest_hit.hit_shape));
        let diffuse = vcos(normal, -light_direction);
        matcolor = matcolor * diffuse ;
        // Applying mat lighting
        if (diffuse>0.00001){
            var light_ray : RayParams;
            light_ray.max_length = 20.0;
            light_ray.max_step = 200u;
            light_ray.threshold = 0.001;
            light_ray.skip_shape = latest_hit.hit_shape;
            let light_hit = send_ray(latest_hit.hit_pos, -light_direction, light_ray);
            matcolor = matcolor * min(1.0, shadow_blur * light_hit.min_distance);
        };

        //Specular lighting
        let light_reflection = reflection(light_direction, normal);
        let specular = reflectivity*pow(abs(vcos(light_reflection, ray_direction)),45.0)*max(0.0,diffuse);

        color+=vec3<f32>(specular,specular,specular);

        color += matcolor * color_weight * matness;
        color_weight = color_weight * reflectivity;
        ray_direction = reflection(ray_direction, normal);
        bounce_count += 1u;
    }
    textureStore(target_texture, vec2<i32>(i32(x),i32(y)), vec4<f32>(color,1.0));
}