fn smooth_max(a:f32, b:f32, alpha:f32)->f32{
    return (a * exp2(a * alpha) + b * exp2(b * alpha))/(exp2(a * alpha) + exp2(b * alpha));
};
