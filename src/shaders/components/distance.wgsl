var<private> shape_stack: array<i32,20u>;
var<private> shape_stack_pointer : u32 = 0u;
var<private> res_stack: array<f32,20u>;
var<private> res_stack_pointer : u32 = 0u;

fn add_sstack(s: i32){
    shape_stack[shape_stack_pointer] = s;
    shape_stack_pointer=shape_stack_pointer+ 1u;
};

fn pop_sstack()->i32{
    shape_stack_pointer = shape_stack_pointer - 1u;
    let res = shape_stack[shape_stack_pointer];
    return res;
};

fn clear_sstack(){
    shape_stack_pointer = 0u;
};


fn add_rstack(s:f32){
    res_stack[res_stack_pointer] = s;
    res_stack_pointer = res_stack_pointer+1u;
};

fn pop_rstack()->f32{
    res_stack_pointer = res_stack_pointer - 1u;
    let res = res_stack[res_stack_pointer];
    return res;
};

fn clear_rstack(){
    res_stack_pointer = 0u;
};


fn shape_distance(point: vec3<f32>, root:u32, skip:i32)-> DistRes{
    clear_rstack();
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
                    if(c.t == 2u && i32(c.a) == skip){
                        skip_sign = -1.0;
                    }
                }
                default:{}
            }
        }else{
            let index = u32(current);
            let shape = shapes[index];

            switch(shape.shape_type){
                case 0u:{
                    var d = sphere_distance(point, spheres[shape.index]);
                    if (i32(index) == skip){
                        d = 9999999.0 * skip_sign;
                    }
                    if(mdist>abs(d)){
                        mdist = abs(d);
                        midx = index;
                    }
                    add_rstack(d);
                }
                case 1u:{
                    var d = cube_distance(point, cuboids[shape.index]);
                    if (i32(index) == skip){
                        d = 9999999.0 * skip_sign;
                    }
                    if(mdist>abs(d)){
                        mdist = abs(d);
                        midx = index;
                    }
                    add_rstack(d);
                }
                case 9u:{
                    let c = composites[shape.index];
                    let a = pop_rstack();
                    let b = pop_rstack();
                    switch(c.t){
                        case 0u:{
                            add_rstack(min(a,b));
                        }
                        case 1u:{
                            add_rstack(max(a,b));
                        }
                        case 2u:{
                            add_rstack(max(b,-a));
                        }
                        case 3u:{
                            add_rstack(smooth_max(a,b,-c.alpha));
                        }
                        default:{}
                    }
                }
                default:{}
            }
        }
    }
    var res: DistRes;
    res.distance = pop_rstack();
    res.index = midx;
    return res;
};