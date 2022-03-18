// Note we reuse the shape stack from the distance function
var<private> sres_stack: array<SurfaceInfo,20u>;
var<private> sres_stack_pointer : u32 = 0u;

fn add_srstack(s:SurfaceInfo){
    sres_stack[sres_stack_pointer] = s;
    sres_stack_pointer = sres_stack_pointer + 1u;
};

fn pop_srstack()->SurfaceInfo{
    sres_stack_pointer = sres_stack_pointer - 1u;
    let res = sres_stack[sres_stack_pointer];
    return res;
};

fn clear_srstack(){
    sres_stack_pointer = 0u;
};

fn shape_surface(point: vec3<f32>, root:u32)-> SurfaceInfo{
    clear_rstack();
    clear_srstack();
    clear_sstack();
    add_sstack(-i32(root+1u));

    var mdist : f32 = 99999999999.0;
    var midx: u32 = 0u;
    var skip_sign = 1.0;
    loop {
        if(shape_stack_pointer == 0u){break;}

        let current = pop_sstack();
        if (current<0){
            let index = u32(-current) - 1u;
            let shape = shapes[index];

            add_sstack(i32(index));
            switch(shape.shape_type){
                case 9u:{
                    let c = composites[shape.index];
                    add_sstack(-i32(c.a+1u));
                    add_sstack(-i32(c.b+1u));
                }
                default:{}
            }
        }else{
            let index = u32(current);
            let shape = shapes[index];

            switch(shape.shape_type){
                case 0u:{
                    var d = sphere_distance(point, spheres[shape.index]);
                    var surface_info : SurfaceInfo;
                    surface_info.color = shape.color;
                    surface_info.reflectivity = shape.reflectivity;
                    surface_info.normal = sphere_normal(point, spheres[shape.index]);
                    add_rstack(d);
                    add_srstack(surface_info);
                }
                case 1u:{
                    var d = cube_distance(point, cuboids[shape.index]);
                    var surface_info : SurfaceInfo;
                    surface_info.color = shape.color;
                    surface_info.reflectivity = shape.reflectivity;
                    surface_info.normal = cube_normal(point, cuboids[shape.index]);
                    add_rstack(d);
                    add_srstack(surface_info);
                }
                case 9u:{
                    let c = composites[shape.index];
                    let ad = pop_rstack();
                    var as = pop_srstack();
                    let bd = pop_rstack();
                    let bs = pop_srstack();

                    switch(c.t){
                        case 0u:{
                            if(ad<bd){
                                add_srstack(as);
                            }else{
                                add_srstack(bs);
                            }
                            add_rstack(min(ad,bd));
                        }
                        case 1u:{
                            if(ad>bd){
                                add_srstack(as);
                            }else{
                                add_srstack(bs);
                            }
                            add_rstack(max(ad,bd));
                        }
                        case 2u:{
                            if(-ad>bd){
                                as.normal = -as.normal;
                                add_srstack(as);
                            }else{
                                add_srstack(bs);
                            }
                            add_rstack(max(bd,-ad));
                        }
                        case 3u:{
                            if(ad<bd){
                                add_srstack(as);
                            }else{
                                add_srstack(bs);
                            }
                            add_rstack(smooth_max(ad,bd,-c.alpha));
                        }
                        default:{}
                    }
                }
                default:{}
            }
        }
    }
    return pop_srstack();
};