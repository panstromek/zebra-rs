
/*
   epcstat.ccc

   Automatically created by CORRELA2 on Sat Nov 23 16:29:22 2002
*/
use crate::src::libc;


pub static mut end_mean: [[f32; 9]; 61] =
    [[0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 1.98f32, -0.47f32, 1.65f32, -0.73f32, 1.42f32,
      -0.92f32, 1.31f32, -0.92f32],
     [0.0f64 as f32, 2.17f32, -0.30f32, 1.82f32, -0.48f32, 1.74f32,
      -0.62f32, 1.54f32, -0.73f32],
     [0.0f64 as f32, 1.99f32, -0.58f32, 1.52f32, -0.83f32, 1.41f32,
      -0.90f32, 1.28f32, -0.90f32],
     [0.0f64 as f32, 2.29f32, -0.10f32, 2.00f32, -0.38f32, 1.74f32,
      -0.51f32, 1.58f32, -0.63f32],
     [0.0f64 as f32, 1.84f32, -0.62f32, 1.54f32, -0.79f32, 1.36f32,
      -0.84f32, 1.28f32, -0.76f32],
     [0.0f64 as f32, 2.39f32, -0.11f32, 1.94f32, -0.34f32, 1.74f32,
      -0.48f32, 1.43f32, -0.56f32],
     [0.0f64 as f32, 1.83f32, -0.63f32, 1.46f32, -0.81f32, 1.31f32,
      -0.78f32, 1.20f32, -0.72f32],
     [0.0f64 as f32, 2.38f32, -0.04f32, 1.90f32, -0.34f32, 1.54f32,
      -0.48f32, 1.33f32, -0.65f32],
     [0.0f64 as f32, 1.87f32, -0.49f32, 1.57f32, -0.57f32, 1.34f32,
      -0.56f32, 1.34f32, -0.60f32],
     [0.0f64 as f32, 2.29f32, -0.26f32, 1.61f32, -0.40f32, 1.37f32,
      -0.59f32, 1.24f32, -0.66f32],
     [0.0f64 as f32, 2.04f32, -0.28f32, 1.54f32, -0.40f32, 1.48f32,
      -0.49f32, 1.37f32, -0.48f32],
     [0.0f64 as f32, 2.00f32, -0.12f32, 1.59f32, -0.43f32, 1.40f32,
      -0.52f32, 1.21f32, -0.56f32],
     [0.0f64 as f32, 1.76f32, -0.30f32, 1.57f32, -0.40f32, 1.40f32,
      -0.44f32, 1.29f32, -0.71f32],
     [0.0f64 as f32, 1.89f32, -0.22f32, 1.51f32, -0.37f32, 1.34f32,
      -0.42f32, 1.43f32, -0.17f32],
     [0.0f64 as f32, 2.03f32, -0.06f32, 1.58f32, -0.31f32, 1.34f32,
      -0.63f32, 0.92f32, -0.76f32],
     [0.0f64 as f32, 1.77f32, -0.12f32, 1.48f32, -0.28f32, 1.53f32,
      -0.09f32, 1.49f32, -0.10f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32]];

pub static mut end_sigma: [[f32; 9]; 61] =
    [[0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 4.03f32, 3.54f32, 3.42f32, 3.02f32, 2.93f32,
      2.74f32, 2.71f32, 2.45f32],
     [0.0f64 as f32, 4.12f32, 3.63f32, 3.39f32, 3.19f32, 3.02f32,
      2.93f32, 2.66f32, 2.59f32],
     [0.0f64 as f32, 4.13f32, 3.62f32, 3.56f32, 3.32f32, 3.19f32,
      2.85f32, 2.76f32, 2.56f32],
     [0.0f64 as f32, 4.19f32, 3.96f32, 3.71f32, 3.48f32, 3.11f32,
      2.99f32, 2.81f32, 2.71f32],
     [0.0f64 as f32, 4.41f32, 3.92f32, 3.87f32, 3.38f32, 3.29f32,
      3.02f32, 2.89f32, 2.58f32],
     [0.0f64 as f32, 4.41f32, 4.09f32, 3.69f32, 3.51f32, 3.30f32,
      3.14f32, 2.80f32, 2.72f32],
     [0.0f64 as f32, 4.70f32, 4.07f32, 3.96f32, 3.57f32, 3.45f32,
      3.02f32, 2.96f32, 2.66f32],
     [0.0f64 as f32, 4.66f32, 4.40f32, 3.97f32, 3.72f32, 3.26f32,
      3.17f32, 2.87f32, 2.78f32],
     [0.0f64 as f32, 4.84f32, 4.22f32, 4.09f32, 3.53f32, 3.50f32,
      3.14f32, 3.08f32, 2.83f32],
     [0.0f64 as f32, 4.95f32, 4.47f32, 3.91f32, 3.79f32, 3.41f32,
      3.31f32, 3.01f32, 2.90f32],
     [0.0f64 as f32, 5.04f32, 4.26f32, 4.20f32, 3.71f32, 3.61f32,
      3.27f32, 3.17f32, 2.89f32],
     [0.0f64 as f32, 4.73f32, 4.52f32, 4.07f32, 3.94f32, 3.59f32,
      3.46f32, 3.10f32, 3.05f32],
     [0.0f64 as f32, 5.18f32, 4.43f32, 4.40f32, 3.95f32, 3.83f32,
      3.41f32, 3.35f32, 3.18f32],
     [0.0f64 as f32, 4.83f32, 4.65f32, 4.23f32, 4.08f32, 3.67f32,
      3.60f32, 3.32f32, 3.27f32],
     [0.0f64 as f32, 5.08f32, 4.55f32, 4.43f32, 3.96f32, 3.88f32,
      3.60f32, 3.52f32, 3.24f32],
     [0.0f64 as f32, 4.91f32, 4.70f32, 4.23f32, 4.07f32, 3.87f32,
      3.72f32, 3.44f32, 3.20f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32],
     [0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32, 0.0f64 as f32,
      0.0f64 as f32]];

pub static mut end_stats_available: [[i16; 9]; 61] =
    [[0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16, 1 as i32 as i16,
      1 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16],
     [0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16, 0 as i32 as i16,
      0 as i32 as i16]];
