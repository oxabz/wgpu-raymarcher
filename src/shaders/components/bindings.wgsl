// Texture that recieve the result of the computations
@group(0) @binding(0)
var target_texture: texture_storage_2d<rgba8unorm, write>;

// Bind group related to the shapes
@group(1) @binding(0)
var<uniform> shape_count: ShapeCount;
@group(1) @binding(1)
var<storage> shapes: array<Shape>;
@group(1) @binding(2)
var<storage> spheres: array<Sphere>;
@group(1) @binding(3)
var<storage> cuboids: array<Cuboid>;
@group(1) @binding(4)
var<storage> composites: array<Composite>;

// Camera bind group
@group(2) @binding(0)
var<uniform> camera: Camera;