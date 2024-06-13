
use serde::Serialize;
use serde::Deserialize;
use bevy::prelude::*;

//just for now .. 
pub type Position = Vec3;
pub type Rotation = Vec3;



//maybe use bevy primitive shapes ? 
#[derive(Debug, Serialize, Deserialize)]
enum ShapeType {
    Cube,
    Sphere,
    Cylinder,
}
 

#[derive(Debug, Serialize, Deserialize)]
struct Shape {
    id: u32,
    shape_type: ShapeType,
    dimensions: Vec<f32>,  // Can vary: [size] for Cube, [radius] for Sphere, [height, radius] for Cylinder
    position: Position,
    rotation: Rotation,
}

#[derive(Debug, Serialize, Deserialize)]
enum OperationType {
    Union,
    Intersection,
    Subtraction
}

#[derive(Debug, Serialize, Deserialize)]
struct Operation {
    id: u32,
    operation_type: OperationType,
    inputs: Vec<u32>, // References to other Operation IDs or Shape IDs
}

//this is a Save file that would be saved or loaded 
#[derive(Debug, Serialize, Deserialize)]
struct CSGModel {
    shapes: Vec<Shape>,
    operations: Vec<Operation>,
    root_operation: u32,
}