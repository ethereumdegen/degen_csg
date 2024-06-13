{
  "CSGModel": {
    "shapes": [
      {
        "type": "Cube",
        "id": 1,
        "size": 2.0,
        "position": { "x": 0, "y": 0, "z": 0 },
        "rotation": { "x": 0, "y": 0, "z": 0 }
      },
      {
        "type": "Sphere",
        "id": 2,
        "radius": 1.5,
        "position": { "x": 3, "y": 0, "z": 0 },
        "rotation": { "x": 0, "y": 0, "z": 0 }
      }
    ],
    "operations": [
      {
        "operation": "Union",
        "left": 1,
        "right": 2
      }
    ]
  }
}