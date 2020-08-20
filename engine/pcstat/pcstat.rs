#[derive(Copy, Clone)]
#[repr(C)]
pub struct Correlation {
    pub const_base: f32,
    pub const_slope: f32,
    pub sigma_base: f32,
    pub sigma_slope: f32,
}
/*
   pcstat.c

   Automatically created by CORRELAT on Tue Sep 07 19:41:49 1999
*/

pub static mut mid_corr: [[Correlation; 9]; 61] =
    [[{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.244f64 as f32,
                          const_slope: -0.374f64 as f32,
                          sigma_base: 1.778f64 as f32,
                          sigma_slope: 0.023f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.457f64 as f32,
                          const_slope: -0.022f64 as f32,
                          sigma_base: 1.245f64 as f32,
                          sigma_slope: 0.039f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.593f64 as f32,
                          const_slope: -0.343f64 as f32,
                          sigma_base: 0.132f64 as f32,
                          sigma_slope: 0.125f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.592f64 as f32,
                          const_slope: -0.050f64 as f32,
                          sigma_base: -0.148f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.361f64 as f32,
                          const_slope: -0.252f64 as f32,
                          sigma_base: 0.455f64 as f32,
                          sigma_slope: 0.096f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -1.174f64 as f32,
                          const_slope: 0.042f64 as f32,
                          sigma_base: -0.592f64 as f32,
                          sigma_slope: 0.120f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.073f64 as f32,
                          const_slope: -0.202f64 as f32,
                          sigma_base: 0.626f64 as f32,
                          sigma_slope: 0.005f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -2.226f64 as f32,
                          const_slope: 0.171f64 as f32,
                          sigma_base: 0.012f64 as f32,
                          sigma_slope: 0.063f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.373f64 as f32,
                          const_slope: -0.256f64 as f32,
                          sigma_base: 1.457f64 as f32,
                          sigma_slope: 0.024f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.244f64 as f32,
                          const_slope: 0.056f64 as f32,
                          sigma_base: 1.513f64 as f32,
                          sigma_slope: 0.024f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.359f64 as f32,
                          const_slope: -0.232f64 as f32,
                          sigma_base: 0.985f64 as f32,
                          sigma_slope: 0.038f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.157f64 as f32,
                          const_slope: 0.052f64 as f32,
                          sigma_base: 0.305f64 as f32,
                          sigma_slope: 0.084f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.556f64 as f32,
                          const_slope: -0.192f64 as f32,
                          sigma_base: 0.259f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.226f64 as f32,
                          const_slope: 0.047f64 as f32,
                          sigma_base: 0.407f64 as f32,
                          sigma_slope: 0.061f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.289f64 as f32,
                          const_slope: -0.188f64 as f32,
                          sigma_base: 0.421f64 as f32,
                          sigma_slope: 0.044f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.719f64 as f32,
                          const_slope: 0.070f64 as f32,
                          sigma_base: 0.148f64 as f32,
                          sigma_slope: 0.050f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.048f64 as f32,
                          const_slope: -0.135f64 as f32,
                          sigma_base: 1.533f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.252f64 as f32,
                          const_slope: 0.089f64 as f32,
                          sigma_base: 1.489f64 as f32,
                          sigma_slope: 0.025f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.145f64 as f32,
                          const_slope: -0.119f64 as f32,
                          sigma_base: 0.626f64 as f32,
                          sigma_slope: 0.101f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.101f64 as f32,
                          const_slope: 0.075f64 as f32,
                          sigma_base: 1.037f64 as f32,
                          sigma_slope: 0.034f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.207f64 as f32,
                          const_slope: -0.093f64 as f32,
                          sigma_base: 0.427f64 as f32,
                          sigma_slope: 0.100f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.150f64 as f32,
                          const_slope: 0.056f64 as f32,
                          sigma_base: 0.669f64 as f32,
                          sigma_slope: 0.046f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.015f64 as f32,
                          const_slope: -0.120f64 as f32,
                          sigma_base: 0.358f64 as f32,
                          sigma_slope: 0.053f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.118f64 as f32,
                          const_slope: 0.034f64 as f32,
                          sigma_base: 0.199f64 as f32,
                          sigma_slope: 0.071f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.058f64 as f32,
                          const_slope: -0.125f64 as f32,
                          sigma_base: 1.518f64 as f32,
                          sigma_slope: 0.044f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.421f64 as f32,
                          const_slope: 0.066f64 as f32,
                          sigma_base: 1.505f64 as f32,
                          sigma_slope: 0.023f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.204f64 as f32,
                          const_slope: -0.115f64 as f32,
                          sigma_base: 0.667f64 as f32,
                          sigma_slope: 0.080f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.169f64 as f32,
                          const_slope: 0.061f64 as f32,
                          sigma_base: 0.761f64 as f32,
                          sigma_slope: 0.061f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.104f64 as f32,
                          const_slope: -0.084f64 as f32,
                          sigma_base: 0.444f64 as f32,
                          sigma_slope: 0.091f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.135f64 as f32,
                          const_slope: 0.046f64 as f32,
                          sigma_base: 0.566f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.982f64 as f32,
                          const_slope: -0.113f64 as f32,
                          sigma_base: 0.428f64 as f32,
                          sigma_slope: 0.045f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.234f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 0.310f64 as f32,
                          sigma_slope: 0.060f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.112f64 as f32,
                          const_slope: -0.091f64 as f32,
                          sigma_base: 1.647f64 as f32,
                          sigma_slope: 0.043f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.318f64 as f32,
                          const_slope: 0.084f64 as f32,
                          sigma_base: 1.401f64 as f32,
                          sigma_slope: 0.028f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.155f64 as f32,
                          const_slope: -0.084f64 as f32,
                          sigma_base: 0.803f64 as f32,
                          sigma_slope: 0.070f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.219f64 as f32,
                          const_slope: 0.078f64 as f32,
                          sigma_base: 0.807f64 as f32,
                          sigma_slope: 0.053f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.218f64 as f32,
                          const_slope: -0.070f64 as f32,
                          sigma_base: 0.666f64 as f32,
                          sigma_slope: 0.070f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.186f64 as f32,
                          const_slope: 0.060f64 as f32,
                          sigma_base: 0.815f64 as f32,
                          sigma_slope: 0.030f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.884f64 as f32,
                          const_slope: -0.099f64 as f32,
                          sigma_base: 0.589f64 as f32,
                          sigma_slope: 0.031f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.321f64 as f32,
                          const_slope: 0.049f64 as f32,
                          sigma_base: 0.491f64 as f32,
                          sigma_slope: 0.042f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.141f64 as f32,
                          const_slope: -0.080f64 as f32,
                          sigma_base: 1.709f64 as f32,
                          sigma_slope: 0.045f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.227f64 as f32,
                          const_slope: 0.076f64 as f32,
                          sigma_base: 1.389f64 as f32,
                          sigma_slope: 0.047f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: -0.073f64 as f32,
                          sigma_base: 1.055f64 as f32,
                          sigma_slope: 0.051f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.294f64 as f32,
                          const_slope: 0.075f64 as f32,
                          sigma_base: 0.846f64 as f32,
                          sigma_slope: 0.055f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.126f64 as f32,
                          const_slope: -0.057f64 as f32,
                          sigma_base: 0.750f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.215f64 as f32,
                          const_slope: 0.060f64 as f32,
                          sigma_base: 0.839f64 as f32,
                          sigma_slope: 0.027f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.636f64 as f32,
                          const_slope: -0.074f64 as f32,
                          sigma_base: 0.511f64 as f32,
                          sigma_slope: 0.040f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.417f64 as f32,
                          const_slope: 0.056f64 as f32,
                          sigma_base: 0.514f64 as f32,
                          sigma_slope: 0.034f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.010f64 as f32,
                          const_slope: -0.071f64 as f32,
                          sigma_base: 1.712f64 as f32,
                          sigma_slope: 0.055f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.195f64 as f32,
                          const_slope: 0.076f64 as f32,
                          sigma_base: 1.450f64 as f32,
                          sigma_slope: 0.048f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.132f64 as f32,
                          const_slope: -0.066f64 as f32,
                          sigma_base: 1.099f64 as f32,
                          sigma_slope: 0.053f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.270f64 as f32,
                          const_slope: 0.074f64 as f32,
                          sigma_base: 0.892f64 as f32,
                          sigma_slope: 0.048f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.187f64 as f32,
                          const_slope: -0.056f64 as f32,
                          sigma_base: 0.829f64 as f32,
                          sigma_slope: 0.052f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.331f64 as f32,
                          const_slope: 0.067f64 as f32,
                          sigma_base: 0.770f64 as f32,
                          sigma_slope: 0.035f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.577f64 as f32,
                          const_slope: -0.070f64 as f32,
                          sigma_base: 0.574f64 as f32,
                          sigma_slope: 0.037f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.434f64 as f32,
                          const_slope: 0.060f64 as f32,
                          sigma_base: 0.497f64 as f32,
                          sigma_slope: 0.038f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.021f64 as f32,
                          const_slope: -0.071f64 as f32,
                          sigma_base: 1.758f64 as f32,
                          sigma_slope: 0.060f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.182f64 as f32,
                          const_slope: 0.066f64 as f32,
                          sigma_base: 1.493f64 as f32,
                          sigma_slope: 0.047f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.094f64 as f32,
                          const_slope: -0.064f64 as f32,
                          sigma_base: 1.062f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.276f64 as f32,
                          const_slope: 0.067f64 as f32,
                          sigma_base: 0.978f64 as f32,
                          sigma_slope: 0.040f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.275f64 as f32,
                          const_slope: -0.061f64 as f32,
                          sigma_base: 0.800f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.329f64 as f32,
                          const_slope: 0.062f64 as f32,
                          sigma_base: 0.874f64 as f32,
                          sigma_slope: 0.028f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.685f64 as f32,
                          const_slope: -0.080f64 as f32,
                          sigma_base: 0.527f64 as f32,
                          sigma_slope: 0.046f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.424f64 as f32,
                          const_slope: 0.057f64 as f32,
                          sigma_base: 0.633f64 as f32,
                          sigma_slope: 0.029f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.151f64 as f32,
                          const_slope: -0.076f64 as f32,
                          sigma_base: 1.927f64 as f32,
                          sigma_slope: 0.060f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.225f64 as f32,
                          const_slope: 0.055f64 as f32,
                          sigma_base: 1.578f64 as f32,
                          sigma_slope: 0.046f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.034f64 as f32,
                          const_slope: -0.065f64 as f32,
                          sigma_base: 1.107f64 as f32,
                          sigma_slope: 0.052f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.259f64 as f32,
                          const_slope: 0.057f64 as f32,
                          sigma_base: 1.038f64 as f32,
                          sigma_slope: 0.039f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.304f64 as f32,
                          const_slope: -0.064f64 as f32,
                          sigma_base: 0.914f64 as f32,
                          sigma_slope: 0.051f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.277f64 as f32,
                          const_slope: 0.053f64 as f32,
                          sigma_base: 0.751f64 as f32,
                          sigma_slope: 0.045f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.652f64 as f32,
                          const_slope: -0.079f64 as f32,
                          sigma_base: 0.609f64 as f32,
                          sigma_slope: 0.045f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.291f64 as f32,
                          const_slope: 0.044f64 as f32,
                          sigma_base: 0.483f64 as f32,
                          sigma_slope: 0.045f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.162f64 as f32,
                          const_slope: -0.082f64 as f32,
                          sigma_base: 1.973f64 as f32,
                          sigma_slope: 0.064f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.161f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.581f64 as f32,
                          sigma_slope: 0.060f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.094f64 as f32,
                          const_slope: -0.073f64 as f32,
                          sigma_base: 1.231f64 as f32,
                          sigma_slope: 0.047f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.296f64 as f32,
                          const_slope: 0.048f64 as f32,
                          sigma_base: 1.056f64 as f32,
                          sigma_slope: 0.046f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.370f64 as f32,
                          const_slope: -0.073f64 as f32,
                          sigma_base: 0.902f64 as f32,
                          sigma_slope: 0.054f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.232f64 as f32,
                          const_slope: 0.044f64 as f32,
                          sigma_base: 0.636f64 as f32,
                          sigma_slope: 0.057f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.543f64 as f32,
                          const_slope: -0.075f64 as f32,
                          sigma_base: 0.498f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.218f64 as f32,
                          const_slope: 0.035f64 as f32,
                          sigma_base: 0.278f64 as f32,
                          sigma_slope: 0.065f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: -0.084f64 as f32,
                          sigma_base: 2.021f64 as f32,
                          sigma_slope: 0.060f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.130f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 1.568f64 as f32,
                          sigma_slope: 0.061f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.069f64 as f32,
                          const_slope: -0.073f64 as f32,
                          sigma_base: 1.131f64 as f32,
                          sigma_slope: 0.064f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.246f64 as f32,
                          const_slope: 0.043f64 as f32,
                          sigma_base: 0.994f64 as f32,
                          sigma_slope: 0.059f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.408f64 as f32,
                          const_slope: -0.076f64 as f32,
                          sigma_base: 0.831f64 as f32,
                          sigma_slope: 0.065f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.145f64 as f32,
                          const_slope: 0.036f64 as f32,
                          sigma_base: 0.604f64 as f32,
                          sigma_slope: 0.063f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.509f64 as f32,
                          const_slope: -0.074f64 as f32,
                          sigma_base: 0.534f64 as f32,
                          sigma_slope: 0.056f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.032f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 0.323f64 as f32,
                          sigma_slope: 0.062f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: -0.075f64 as f32,
                          sigma_base: 2.047f64 as f32,
                          sigma_slope: 0.061f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.109f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 1.558f64 as f32,
                          sigma_slope: 0.066f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.032f64 as f32,
                          const_slope: -0.064f64 as f32,
                          sigma_base: 1.108f64 as f32,
                          sigma_slope: 0.080f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.254f64 as f32,
                          const_slope: 0.043f64 as f32,
                          sigma_base: 1.028f64 as f32,
                          sigma_slope: 0.065f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.334f64 as f32,
                          const_slope: -0.065f64 as f32,
                          sigma_base: 0.775f64 as f32,
                          sigma_slope: 0.075f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.165f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 0.531f64 as f32,
                          sigma_slope: 0.077f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.437f64 as f32,
                          const_slope: -0.064f64 as f32,
                          sigma_base: 0.432f64 as f32,
                          sigma_slope: 0.069f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.030f64 as f32,
                          const_slope: 0.019f64 as f32,
                          sigma_base: 0.200f64 as f32,
                          sigma_slope: 0.076f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.146f64 as f32,
                          const_slope: -0.065f64 as f32,
                          sigma_base: 2.088f64 as f32,
                          sigma_slope: 0.072f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.111f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 1.588f64 as f32,
                          sigma_slope: 0.075f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.084f64 as f32,
                          const_slope: -0.058f64 as f32,
                          sigma_base: 1.164f64 as f32,
                          sigma_slope: 0.080f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.202f64 as f32,
                          const_slope: 0.040f64 as f32,
                          sigma_base: 1.039f64 as f32,
                          sigma_slope: 0.067f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.398f64 as f32,
                          const_slope: -0.065f64 as f32,
                          sigma_base: 0.798f64 as f32,
                          sigma_slope: 0.075f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.180f64 as f32,
                          const_slope: 0.036f64 as f32,
                          sigma_base: 0.506f64 as f32,
                          sigma_slope: 0.081f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.508f64 as f32,
                          const_slope: -0.069f64 as f32,
                          sigma_base: 0.474f64 as f32,
                          sigma_slope: 0.071f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.075f64 as f32,
                          const_slope: 0.022f64 as f32,
                          sigma_base: 0.291f64 as f32,
                          sigma_slope: 0.072f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.189f64 as f32,
                          const_slope: -0.067f64 as f32,
                          sigma_base: 2.110f64 as f32,
                          sigma_slope: 0.087f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.114f64 as f32,
                          const_slope: 0.031f64 as f32,
                          sigma_base: 1.584f64 as f32,
                          sigma_slope: 0.082f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.059f64 as f32,
                          const_slope: -0.059f64 as f32,
                          sigma_base: 1.215f64 as f32,
                          sigma_slope: 0.084f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.164f64 as f32,
                          const_slope: 0.034f64 as f32,
                          sigma_base: 1.000f64 as f32,
                          sigma_slope: 0.076f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.354f64 as f32,
                          const_slope: -0.063f64 as f32,
                          sigma_base: 0.772f64 as f32,
                          sigma_slope: 0.083f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.161f64 as f32,
                          const_slope: 0.031f64 as f32,
                          sigma_base: 0.502f64 as f32,
                          sigma_slope: 0.090f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.488f64 as f32,
                          const_slope: -0.066f64 as f32,
                          sigma_base: 0.454f64 as f32,
                          sigma_slope: 0.074f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.058f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 0.185f64 as f32,
                          sigma_slope: 0.085f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.245f64 as f32,
                          const_slope: -0.058f64 as f32,
                          sigma_base: 2.200f64 as f32,
                          sigma_slope: 0.100f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.175f64 as f32,
                          const_slope: 0.027f64 as f32,
                          sigma_base: 1.633f64 as f32,
                          sigma_slope: 0.091f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.106f64 as f32,
                          const_slope: -0.054f64 as f32,
                          sigma_base: 1.277f64 as f32,
                          sigma_slope: 0.091f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.045f64 as f32,
                          const_slope: 0.024f64 as f32,
                          sigma_base: 1.070f64 as f32,
                          sigma_slope: 0.078f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.372f64 as f32,
                          const_slope: -0.061f64 as f32,
                          sigma_base: 0.828f64 as f32,
                          sigma_slope: 0.085f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.119f64 as f32,
                          const_slope: 0.022f64 as f32,
                          sigma_base: 0.493f64 as f32,
                          sigma_slope: 0.096f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.421f64 as f32,
                          const_slope: -0.061f64 as f32,
                          sigma_base: 0.522f64 as f32,
                          sigma_slope: 0.076f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.012f64 as f32,
                          const_slope: 0.011f64 as f32,
                          sigma_base: 0.139f64 as f32,
                          sigma_slope: 0.095f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.273f64 as f32,
                          const_slope: -0.050f64 as f32,
                          sigma_base: 2.172f64 as f32,
                          sigma_slope: 0.106f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.147f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 1.639f64 as f32,
                          sigma_slope: 0.091f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.109f64 as f32,
                          const_slope: -0.047f64 as f32,
                          sigma_base: 1.281f64 as f32,
                          sigma_slope: 0.095f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.091f64 as f32,
                          const_slope: 0.028f64 as f32,
                          sigma_base: 1.000f64 as f32,
                          sigma_slope: 0.088f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.329f64 as f32,
                          const_slope: -0.054f64 as f32,
                          sigma_base: 0.825f64 as f32,
                          sigma_slope: 0.089f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.134f64 as f32,
                          const_slope: 0.026f64 as f32,
                          sigma_base: 0.428f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.319f64 as f32,
                          const_slope: -0.049f64 as f32,
                          sigma_base: 0.472f64 as f32,
                          sigma_slope: 0.083f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.059f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 0.042f64 as f32,
                          sigma_slope: 0.106f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.302f64 as f32,
                          const_slope: -0.049f64 as f32,
                          sigma_base: 2.208f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.128f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 1.630f64 as f32,
                          sigma_slope: 0.097f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.088f64 as f32,
                          const_slope: -0.045f64 as f32,
                          sigma_base: 1.280f64 as f32,
                          sigma_slope: 0.098f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.116f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 1.002f64 as f32,
                          sigma_slope: 0.094f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.311f64 as f32,
                          const_slope: -0.051f64 as f32,
                          sigma_base: 0.900f64 as f32,
                          sigma_slope: 0.086f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.097f64 as f32,
                          const_slope: 0.024f64 as f32,
                          sigma_base: 0.480f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.244f64 as f32,
                          const_slope: -0.043f64 as f32,
                          sigma_base: 0.448f64 as f32,
                          sigma_slope: 0.088f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.049f64 as f32,
                          const_slope: 0.014f64 as f32,
                          sigma_base: 0.083f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.278f64 as f32,
                          const_slope: -0.047f64 as f32,
                          sigma_base: 2.241f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.134f64 as f32,
                          const_slope: 0.023f64 as f32,
                          sigma_base: 1.672f64 as f32,
                          sigma_slope: 0.093f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.110f64 as f32,
                          const_slope: -0.045f64 as f32,
                          sigma_base: 1.322f64 as f32,
                          sigma_slope: 0.100f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.137f64 as f32,
                          const_slope: 0.025f64 as f32,
                          sigma_base: 1.006f64 as f32,
                          sigma_slope: 0.097f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.249f64 as f32,
                          const_slope: -0.047f64 as f32,
                          sigma_base: 0.878f64 as f32,
                          sigma_slope: 0.090f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.101f64 as f32,
                          const_slope: 0.022f64 as f32,
                          sigma_base: 0.520f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.201f64 as f32,
                          const_slope: -0.038f64 as f32,
                          sigma_base: 0.352f64 as f32,
                          sigma_slope: 0.098f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.080f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 0.126f64 as f32,
                          sigma_slope: 0.102f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.293f64 as f32,
                          const_slope: -0.038f64 as f32,
                          sigma_base: 2.259f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.169f64 as f32,
                          const_slope: 0.021f64 as f32,
                          sigma_base: 1.721f64 as f32,
                          sigma_slope: 0.096f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.079f64 as f32,
                          const_slope: -0.036f64 as f32,
                          sigma_base: 1.269f64 as f32,
                          sigma_slope: 0.114f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.102f64 as f32,
                          const_slope: 0.022f64 as f32,
                          sigma_base: 1.066f64 as f32,
                          sigma_slope: 0.096f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.208f64 as f32,
                          const_slope: -0.038f64 as f32,
                          sigma_base: 0.895f64 as f32,
                          sigma_slope: 0.095f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.079f64 as f32,
                          const_slope: 0.019f64 as f32,
                          sigma_base: 0.571f64 as f32,
                          sigma_slope: 0.102f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.142f64 as f32,
                          const_slope: -0.029f64 as f32,
                          sigma_base: 0.440f64 as f32,
                          sigma_slope: 0.095f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.063f64 as f32,
                          const_slope: 0.013f64 as f32,
                          sigma_base: 0.135f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.264f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 2.308f64 as f32,
                          sigma_slope: 0.110f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.169f64 as f32,
                          const_slope: 0.020f64 as f32,
                          sigma_base: 1.738f64 as f32,
                          sigma_slope: 0.100f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.030f64 as f32,
                          const_slope: -0.029f64 as f32,
                          sigma_base: 1.270f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.084f64 as f32,
                          const_slope: 0.020f64 as f32,
                          sigma_base: 1.107f64 as f32,
                          sigma_slope: 0.102f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.191f64 as f32,
                          const_slope: -0.032f64 as f32,
                          sigma_base: 0.834f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.035f64 as f32,
                          const_slope: 0.015f64 as f32,
                          sigma_base: 0.605f64 as f32,
                          sigma_slope: 0.101f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.103f64 as f32,
                          const_slope: -0.023f64 as f32,
                          sigma_base: 0.450f64 as f32,
                          sigma_slope: 0.098f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.006f64 as f32,
                          const_slope: 0.007f64 as f32,
                          sigma_base: 0.123f64 as f32,
                          sigma_slope: 0.104f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.294f64 as f32,
                          const_slope: -0.026f64 as f32,
                          sigma_base: 2.333f64 as f32,
                          sigma_slope: 0.114f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.161f64 as f32,
                          const_slope: 0.024f64 as f32,
                          sigma_base: 1.723f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.048f64 as f32,
                          const_slope: -0.024f64 as f32,
                          sigma_base: 1.279f64 as f32,
                          sigma_slope: 0.126f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.044f64 as f32,
                          const_slope: 0.021f64 as f32,
                          sigma_base: 1.124f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.139f64 as f32,
                          const_slope: -0.026f64 as f32,
                          sigma_base: 0.856f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.043f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 0.653f64 as f32,
                          sigma_slope: 0.102f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.026f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 0.436f64 as f32,
                          sigma_slope: 0.102f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.055f64 as f32,
                          const_slope: 0.011f64 as f32,
                          sigma_base: 0.126f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.229f64 as f32,
                          const_slope: -0.022f64 as f32,
                          sigma_base: 2.299f64 as f32,
                          sigma_slope: 0.116f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.139f64 as f32,
                          const_slope: 0.022f64 as f32,
                          sigma_base: 1.725f64 as f32,
                          sigma_slope: 0.110f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.041f64 as f32,
                          const_slope: -0.020f64 as f32,
                          sigma_base: 1.310f64 as f32,
                          sigma_slope: 0.128f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.003f64 as f32,
                          const_slope: 0.017f64 as f32,
                          sigma_base: 1.120f64 as f32,
                          sigma_slope: 0.108f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.132f64 as f32,
                          const_slope: -0.022f64 as f32,
                          sigma_base: 0.849f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.016f64 as f32,
                          const_slope: 0.012f64 as f32,
                          sigma_base: 0.687f64 as f32,
                          sigma_slope: 0.101f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.007f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 0.387f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.056f64 as f32,
                          const_slope: 0.009f64 as f32,
                          sigma_base: 0.133f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.160f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 2.444f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.122f64 as f32,
                          const_slope: 0.023f64 as f32,
                          sigma_base: 1.826f64 as f32,
                          sigma_slope: 0.116f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.009f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 1.341f64 as f32,
                          sigma_slope: 0.129f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.007f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.177f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.045f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 0.847f64 as f32,
                          sigma_slope: 0.113f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.032f64 as f32,
                          const_slope: 0.013f64 as f32,
                          sigma_base: 0.698f64 as f32,
                          sigma_slope: 0.103f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.068f64 as f32,
                          const_slope: 0.001f64 as f32,
                          sigma_base: 0.401f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.157f64 as f32,
                          const_slope: 0.017f64 as f32,
                          sigma_base: 0.206f64 as f32,
                          sigma_slope: 0.100f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.170f64 as f32,
                          const_slope: -0.002f64 as f32,
                          sigma_base: 2.511f64 as f32,
                          sigma_slope: 0.112f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.130f64 as f32,
                          const_slope: 0.020f64 as f32,
                          sigma_base: 1.868f64 as f32,
                          sigma_slope: 0.118f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.026f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: 1.443f64 as f32,
                          sigma_slope: 0.121f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.001f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 1.193f64 as f32,
                          sigma_slope: 0.112f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.010f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: 0.892f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.025f64 as f32,
                          const_slope: 0.011f64 as f32,
                          sigma_base: 0.701f64 as f32,
                          sigma_slope: 0.106f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.063f64 as f32,
                          const_slope: 0.003f64 as f32,
                          sigma_base: 0.442f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.161f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 0.195f64 as f32,
                          sigma_slope: 0.104f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.222f64 as f32,
                          const_slope: 0.001f64 as f32,
                          sigma_base: 2.581f64 as f32,
                          sigma_slope: 0.115f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.156f64 as f32,
                          const_slope: 0.017f64 as f32,
                          sigma_base: 1.930f64 as f32,
                          sigma_slope: 0.121f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.041f64 as f32,
                          const_slope: -0.002f64 as f32,
                          sigma_base: 1.477f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.039f64 as f32,
                          const_slope: 0.011f64 as f32,
                          sigma_base: 1.185f64 as f32,
                          sigma_slope: 0.118f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.033f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: 0.978f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.041f64 as f32,
                          const_slope: 0.004f64 as f32,
                          sigma_base: 0.750f64 as f32,
                          sigma_slope: 0.106f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.138f64 as f32,
                          const_slope: 0.008f64 as f32,
                          sigma_base: 0.482f64 as f32,
                          sigma_slope: 0.106f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.182f64 as f32,
                          const_slope: 0.014f64 as f32,
                          sigma_base: 0.196f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.217f64 as f32,
                          const_slope: 0.002f64 as f32,
                          sigma_base: 2.570f64 as f32,
                          sigma_slope: 0.115f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.176f64 as f32,
                          const_slope: 0.012f64 as f32,
                          sigma_base: 1.929f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.003f64 as f32,
                          const_slope: 0.002f64 as f32,
                          sigma_base: 1.500f64 as f32,
                          sigma_slope: 0.121f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: 0.003f64 as f32,
                          sigma_base: 1.184f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.020f64 as f32,
                          const_slope: -0.000f64 as f32,
                          sigma_base: 1.006f64 as f32,
                          sigma_slope: 0.104f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.149f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 0.724f64 as f32,
                          sigma_slope: 0.112f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.144f64 as f32,
                          const_slope: 0.010f64 as f32,
                          sigma_base: 0.506f64 as f32,
                          sigma_slope: 0.104f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.064f64 as f32,
                          const_slope: 0.001f64 as f32,
                          sigma_base: 0.136f64 as f32,
                          sigma_slope: 0.118f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.239f64 as f32,
                          const_slope: 0.007f64 as f32,
                          sigma_base: 2.620f64 as f32,
                          sigma_slope: 0.114f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.195f64 as f32,
                          const_slope: 0.008f64 as f32,
                          sigma_base: 2.013f64 as f32,
                          sigma_slope: 0.125f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.017f64 as f32,
                          const_slope: 0.007f64 as f32,
                          sigma_base: 1.511f64 as f32,
                          sigma_slope: 0.129f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.084f64 as f32,
                          const_slope: 0.002f64 as f32,
                          sigma_base: 1.260f64 as f32,
                          sigma_slope: 0.121f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.025f64 as f32,
                          const_slope: 0.006f64 as f32,
                          sigma_base: 1.030f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.137f64 as f32,
                          const_slope: -0.007f64 as f32,
                          sigma_base: 0.760f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.227f64 as f32,
                          const_slope: 0.020f64 as f32,
                          sigma_base: 0.495f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.071f64 as f32,
                          const_slope: 0.001f64 as f32,
                          sigma_base: 0.172f64 as f32,
                          sigma_slope: 0.116f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.207f64 as f32,
                          const_slope: 0.010f64 as f32,
                          sigma_base: 2.601f64 as f32,
                          sigma_slope: 0.112f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.186f64 as f32,
                          const_slope: 0.004f64 as f32,
                          sigma_base: 1.994f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.037f64 as f32,
                          const_slope: 0.010f64 as f32,
                          sigma_base: 1.561f64 as f32,
                          sigma_slope: 0.125f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.067f64 as f32,
                          const_slope: -0.001f64 as f32,
                          sigma_base: 1.265f64 as f32,
                          sigma_slope: 0.120f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.028f64 as f32,
                          const_slope: 0.009f64 as f32,
                          sigma_base: 1.055f64 as f32,
                          sigma_slope: 0.107f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.151f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 0.787f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.212f64 as f32,
                          const_slope: 0.020f64 as f32,
                          sigma_base: 0.501f64 as f32,
                          sigma_slope: 0.108f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.041f64 as f32,
                          const_slope: -0.002f64 as f32,
                          sigma_base: 0.113f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.159f64 as f32,
                          const_slope: 0.017f64 as f32,
                          sigma_base: 2.642f64 as f32,
                          sigma_slope: 0.112f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.150f64 as f32,
                          const_slope: 0.006f64 as f32,
                          sigma_base: 2.023f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.095f64 as f32,
                          const_slope: 0.019f64 as f32,
                          sigma_base: 1.589f64 as f32,
                          sigma_slope: 0.127f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.101f64 as f32,
                          const_slope: -0.001f64 as f32,
                          sigma_base: 1.328f64 as f32,
                          sigma_slope: 0.118f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.052f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 1.060f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.172f64 as f32,
                          const_slope: -0.012f64 as f32,
                          sigma_base: 0.881f64 as f32,
                          sigma_slope: 0.105f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.204f64 as f32,
                          const_slope: 0.023f64 as f32,
                          sigma_base: 0.532f64 as f32,
                          sigma_slope: 0.109f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.052f64 as f32,
                          const_slope: -0.002f64 as f32,
                          sigma_base: 0.253f64 as f32,
                          sigma_slope: 0.111f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.123f64 as f32,
                          const_slope: 0.017f64 as f32,
                          sigma_base: 2.694f64 as f32,
                          sigma_slope: 0.115f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.180f64 as f32,
                          const_slope: -0.005f64 as f32,
                          sigma_base: 2.039f64 as f32,
                          sigma_slope: 0.127f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.059f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.620f64 as f32,
                          sigma_slope: 0.131f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.184f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 1.294f64 as f32,
                          sigma_slope: 0.125f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.014f64 as f32,
                          const_slope: 0.013f64 as f32,
                          sigma_base: 0.986f64 as f32,
                          sigma_slope: 0.121f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.241f64 as f32,
                          const_slope: -0.024f64 as f32,
                          sigma_base: 0.767f64 as f32,
                          sigma_slope: 0.118f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.162f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 0.406f64 as f32,
                          sigma_slope: 0.127f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.016f64 as f32,
                          const_slope: -0.012f64 as f32,
                          sigma_base: 0.146f64 as f32,
                          sigma_slope: 0.125f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.093f64 as f32,
                          const_slope: 0.030f64 as f32,
                          sigma_base: 2.713f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.150f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 2.071f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.125f64 as f32,
                          const_slope: 0.032f64 as f32,
                          sigma_base: 1.619f64 as f32,
                          sigma_slope: 0.137f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.171f64 as f32,
                          const_slope: -0.009f64 as f32,
                          sigma_base: 1.405f64 as f32,
                          sigma_slope: 0.126f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.072f64 as f32,
                          const_slope: 0.026f64 as f32,
                          sigma_base: 1.026f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.210f64 as f32,
                          const_slope: -0.019f64 as f32,
                          sigma_base: 0.760f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.220f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 0.394f64 as f32,
                          sigma_slope: 0.132f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.078f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 0.164f64 as f32,
                          sigma_slope: 0.128f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.116f64 as f32,
                          const_slope: 0.026f64 as f32,
                          sigma_base: 2.713f64 as f32,
                          sigma_slope: 0.122f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.133f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 2.111f64 as f32,
                          sigma_slope: 0.136f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.104f64 as f32,
                          const_slope: 0.027f64 as f32,
                          sigma_base: 1.636f64 as f32,
                          sigma_slope: 0.136f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.173f64 as f32,
                          const_slope: -0.016f64 as f32,
                          sigma_base: 1.362f64 as f32,
                          sigma_slope: 0.134f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.018f64 as f32,
                          const_slope: 0.019f64 as f32,
                          sigma_base: 1.003f64 as f32,
                          sigma_slope: 0.126f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.172f64 as f32,
                          const_slope: -0.021f64 as f32,
                          sigma_base: 0.692f64 as f32,
                          sigma_slope: 0.133f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.187f64 as f32,
                          const_slope: 0.023f64 as f32,
                          sigma_base: 0.252f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.012f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 0.167f64 as f32,
                          sigma_slope: 0.133f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.098f64 as f32,
                          const_slope: 0.038f64 as f32,
                          sigma_base: 2.729f64 as f32,
                          sigma_slope: 0.123f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.111f64 as f32,
                          const_slope: -0.001f64 as f32,
                          sigma_base: 2.182f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.087f64 as f32,
                          const_slope: 0.037f64 as f32,
                          sigma_base: 1.688f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.141f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 1.321f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.049f64 as f32,
                          const_slope: 0.028f64 as f32,
                          sigma_base: 0.993f64 as f32,
                          sigma_slope: 0.129f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.155f64 as f32,
                          const_slope: -0.015f64 as f32,
                          sigma_base: 0.639f64 as f32,
                          sigma_slope: 0.141f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.207f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 0.220f64 as f32,
                          sigma_slope: 0.152f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.028f64 as f32,
                          const_slope: -0.005f64 as f32,
                          sigma_base: 0.210f64 as f32,
                          sigma_slope: 0.131f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.094f64 as f32,
                          const_slope: 0.034f64 as f32,
                          sigma_base: 2.780f64 as f32,
                          sigma_slope: 0.124f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.128f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 2.211f64 as f32,
                          sigma_slope: 0.138f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.122f64 as f32,
                          const_slope: 0.035f64 as f32,
                          sigma_base: 1.697f64 as f32,
                          sigma_slope: 0.139f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.077f64 as f32,
                          const_slope: -0.012f64 as f32,
                          sigma_base: 1.362f64 as f32,
                          sigma_slope: 0.137f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.119f64 as f32,
                          const_slope: 0.031f64 as f32,
                          sigma_base: 1.012f64 as f32,
                          sigma_slope: 0.130f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.120f64 as f32,
                          const_slope: -0.015f64 as f32,
                          sigma_base: 0.593f64 as f32,
                          sigma_slope: 0.146f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.345f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 0.151f64 as f32,
                          sigma_slope: 0.162f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.050f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: 0.159f64 as f32,
                          sigma_slope: 0.141f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.139f64 as f32,
                          const_slope: 0.033f64 as f32,
                          sigma_base: 2.797f64 as f32,
                          sigma_slope: 0.126f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.181f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 2.195f64 as f32,
                          sigma_slope: 0.143f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.093f64 as f32,
                          const_slope: 0.033f64 as f32,
                          sigma_base: 1.693f64 as f32,
                          sigma_slope: 0.142f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.072f64 as f32,
                          const_slope: -0.015f64 as f32,
                          sigma_base: 1.360f64 as f32,
                          sigma_slope: 0.143f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.109f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 0.985f64 as f32,
                          sigma_slope: 0.135f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.164f64 as f32,
                          const_slope: -0.019f64 as f32,
                          sigma_base: 0.562f64 as f32,
                          sigma_slope: 0.152f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.238f64 as f32,
                          const_slope: 0.032f64 as f32,
                          sigma_base: 0.179f64 as f32,
                          sigma_slope: 0.163f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.114f64 as f32,
                          const_slope: -0.017f64 as f32,
                          sigma_base: 0.108f64 as f32,
                          sigma_slope: 0.149f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.157f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 2.697f64 as f32,
                          sigma_slope: 0.142f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.184f64 as f32,
                          const_slope: -0.009f64 as f32,
                          sigma_base: 2.130f64 as f32,
                          sigma_slope: 0.146f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.081f64 as f32,
                          const_slope: 0.038f64 as f32,
                          sigma_base: 1.663f64 as f32,
                          sigma_slope: 0.153f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.060f64 as f32,
                          const_slope: -0.011f64 as f32,
                          sigma_base: 1.274f64 as f32,
                          sigma_slope: 0.152f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.139f64 as f32,
                          const_slope: 0.035f64 as f32,
                          sigma_base: 0.976f64 as f32,
                          sigma_slope: 0.142f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.126f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 0.511f64 as f32,
                          sigma_slope: 0.164f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.314f64 as f32,
                          const_slope: 0.040f64 as f32,
                          sigma_base: 0.199f64 as f32,
                          sigma_slope: 0.167f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: -0.015f64 as f32,
                          sigma_base: -0.060f64 as f32,
                          sigma_slope: 0.169f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.193f64 as f32,
                          const_slope: 0.044f64 as f32,
                          sigma_base: 2.646f64 as f32,
                          sigma_slope: 0.145f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.227f64 as f32,
                          const_slope: -0.007f64 as f32,
                          sigma_base: 2.119f64 as f32,
                          sigma_slope: 0.150f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.089f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.619f64 as f32,
                          sigma_slope: 0.164f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.106f64 as f32,
                          const_slope: -0.011f64 as f32,
                          sigma_base: 1.280f64 as f32,
                          sigma_slope: 0.158f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.108f64 as f32,
                          const_slope: 0.035f64 as f32,
                          sigma_base: 0.880f64 as f32,
                          sigma_slope: 0.159f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.189f64 as f32,
                          const_slope: -0.019f64 as f32,
                          sigma_base: 0.439f64 as f32,
                          sigma_slope: 0.175f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.306f64 as f32,
                          const_slope: 0.039f64 as f32,
                          sigma_base: 0.079f64 as f32,
                          sigma_slope: 0.183f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.144f64 as f32,
                          const_slope: -0.018f64 as f32,
                          sigma_base: -0.112f64 as f32,
                          sigma_slope: 0.179f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.151f64 as f32,
                          const_slope: 0.047f64 as f32,
                          sigma_base: 2.615f64 as f32,
                          sigma_slope: 0.149f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.221f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 2.108f64 as f32,
                          sigma_slope: 0.152f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.101f64 as f32,
                          const_slope: 0.045f64 as f32,
                          sigma_base: 1.627f64 as f32,
                          sigma_slope: 0.166f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.098f64 as f32,
                          const_slope: -0.011f64 as f32,
                          sigma_base: 1.293f64 as f32,
                          sigma_slope: 0.159f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.141f64 as f32,
                          const_slope: 0.039f64 as f32,
                          sigma_base: 0.827f64 as f32,
                          sigma_slope: 0.166f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.166f64 as f32,
                          const_slope: -0.017f64 as f32,
                          sigma_base: 0.396f64 as f32,
                          sigma_slope: 0.180f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.266f64 as f32,
                          const_slope: 0.039f64 as f32,
                          sigma_base: 0.029f64 as f32,
                          sigma_slope: 0.189f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.188f64 as f32,
                          const_slope: -0.021f64 as f32,
                          sigma_base: -0.126f64 as f32,
                          sigma_slope: 0.180f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.136f64 as f32,
                          const_slope: 0.052f64 as f32,
                          sigma_base: 2.654f64 as f32,
                          sigma_slope: 0.151f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.204f64 as f32,
                          const_slope: -0.006f64 as f32,
                          sigma_base: 2.137f64 as f32,
                          sigma_slope: 0.155f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.106f64 as f32,
                          const_slope: 0.049f64 as f32,
                          sigma_base: 1.634f64 as f32,
                          sigma_slope: 0.173f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.010f64 as f32,
                          const_slope: -0.005f64 as f32,
                          sigma_base: 1.307f64 as f32,
                          sigma_slope: 0.162f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.202f64 as f32,
                          const_slope: 0.046f64 as f32,
                          sigma_base: 0.807f64 as f32,
                          sigma_slope: 0.173f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.086f64 as f32,
                          const_slope: -0.009f64 as f32,
                          sigma_base: 0.403f64 as f32,
                          sigma_slope: 0.182f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.355f64 as f32,
                          const_slope: 0.048f64 as f32,
                          sigma_base: -0.022f64 as f32,
                          sigma_slope: 0.197f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.115f64 as f32,
                          const_slope: -0.012f64 as f32,
                          sigma_base: -0.136f64 as f32,
                          sigma_slope: 0.184f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.079f64 as f32,
                          const_slope: 0.059f64 as f32,
                          sigma_base: 2.632f64 as f32,
                          sigma_slope: 0.152f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.188f64 as f32,
                          const_slope: -0.002f64 as f32,
                          sigma_base: 2.089f64 as f32,
                          sigma_slope: 0.156f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.083f64 as f32,
                          const_slope: 0.053f64 as f32,
                          sigma_base: 1.635f64 as f32,
                          sigma_slope: 0.170f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.047f64 as f32,
                          const_slope: -0.004f64 as f32,
                          sigma_base: 1.267f64 as f32,
                          sigma_slope: 0.167f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.175f64 as f32,
                          const_slope: 0.047f64 as f32,
                          sigma_base: 0.787f64 as f32,
                          sigma_slope: 0.175f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.112f64 as f32,
                          const_slope: -0.010f64 as f32,
                          sigma_base: 0.353f64 as f32,
                          sigma_slope: 0.190f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.322f64 as f32,
                          const_slope: 0.046f64 as f32,
                          sigma_base: -0.055f64 as f32,
                          sigma_slope: 0.202f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.134f64 as f32,
                          const_slope: -0.013f64 as f32,
                          sigma_base: -0.164f64 as f32,
                          sigma_slope: 0.189f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.069f64 as f32,
                          const_slope: 0.063f64 as f32,
                          sigma_base: 2.653f64 as f32,
                          sigma_slope: 0.153f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.157f64 as f32,
                          const_slope: 0.003f64 as f32,
                          sigma_base: 2.125f64 as f32,
                          sigma_slope: 0.160f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.110f64 as f32,
                          const_slope: 0.058f64 as f32,
                          sigma_base: 1.581f64 as f32,
                          sigma_slope: 0.177f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.004f64 as f32,
                          const_slope: 0.003f64 as f32,
                          sigma_base: 1.248f64 as f32,
                          sigma_slope: 0.174f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.239f64 as f32,
                          const_slope: 0.055f64 as f32,
                          sigma_base: 0.735f64 as f32,
                          sigma_slope: 0.182f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.076f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: 0.300f64 as f32,
                          sigma_slope: 0.198f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.376f64 as f32,
                          const_slope: 0.054f64 as f32,
                          sigma_base: -0.131f64 as f32,
                          sigma_slope: 0.212f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.076f64 as f32,
                          const_slope: -0.007f64 as f32,
                          sigma_base: -0.112f64 as f32,
                          sigma_slope: 0.188f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.108f64 as f32,
                          const_slope: 0.057f64 as f32,
                          sigma_base: 2.660f64 as f32,
                          sigma_slope: 0.158f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.237f64 as f32,
                          const_slope: -0.007f64 as f32,
                          sigma_base: 2.135f64 as f32,
                          sigma_slope: 0.166f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.042f64 as f32,
                          const_slope: 0.049f64 as f32,
                          sigma_base: 1.565f64 as f32,
                          sigma_slope: 0.183f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.010f64 as f32,
                          const_slope: -0.005f64 as f32,
                          sigma_base: 1.220f64 as f32,
                          sigma_slope: 0.181f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.215f64 as f32,
                          const_slope: 0.047f64 as f32,
                          sigma_base: 0.690f64 as f32,
                          sigma_slope: 0.187f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.075f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 0.170f64 as f32,
                          sigma_slope: 0.211f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.392f64 as f32,
                          const_slope: 0.052f64 as f32,
                          sigma_base: -0.234f64 as f32,
                          sigma_slope: 0.225f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.142f64 as f32,
                          const_slope: -0.013f64 as f32,
                          sigma_base: -0.247f64 as f32,
                          sigma_slope: 0.204f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.121f64 as f32,
                          const_slope: 0.070f64 as f32,
                          sigma_base: 2.612f64 as f32,
                          sigma_slope: 0.158f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.198f64 as f32,
                          const_slope: 0.008f64 as f32,
                          sigma_base: 2.174f64 as f32,
                          sigma_slope: 0.165f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.107f64 as f32,
                          const_slope: 0.064f64 as f32,
                          sigma_base: 1.567f64 as f32,
                          sigma_slope: 0.184f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.111f64 as f32,
                          const_slope: 0.013f64 as f32,
                          sigma_base: 1.302f64 as f32,
                          sigma_slope: 0.177f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.323f64 as f32,
                          const_slope: 0.064f64 as f32,
                          sigma_base: 0.739f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.068f64 as f32,
                          const_slope: 0.012f64 as f32,
                          sigma_base: 0.271f64 as f32,
                          sigma_slope: 0.204f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.489f64 as f32,
                          const_slope: 0.067f64 as f32,
                          sigma_base: -0.119f64 as f32,
                          sigma_slope: 0.212f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.048f64 as f32,
                          const_slope: 0.002f64 as f32,
                          sigma_base: -0.181f64 as f32,
                          sigma_slope: 0.199f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.204f64 as f32,
                          const_slope: 0.059f64 as f32,
                          sigma_base: 2.577f64 as f32,
                          sigma_slope: 0.160f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.269f64 as f32,
                          const_slope: -0.001f64 as f32,
                          sigma_base: 2.149f64 as f32,
                          sigma_slope: 0.166f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.024f64 as f32,
                          const_slope: 0.050f64 as f32,
                          sigma_base: 1.543f64 as f32,
                          sigma_slope: 0.188f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.054f64 as f32,
                          const_slope: 0.002f64 as f32,
                          sigma_base: 1.109f64 as f32,
                          sigma_slope: 0.188f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.215f64 as f32,
                          const_slope: 0.048f64 as f32,
                          sigma_base: 0.623f64 as f32,
                          sigma_slope: 0.197f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.007f64 as f32,
                          const_slope: 0.001f64 as f32,
                          sigma_base: 0.165f64 as f32,
                          sigma_slope: 0.216f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.443f64 as f32,
                          const_slope: 0.056f64 as f32,
                          sigma_base: -0.133f64 as f32,
                          sigma_slope: 0.215f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.001f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: -0.310f64 as f32,
                          sigma_slope: 0.213f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.203f64 as f32,
                          const_slope: 0.073f64 as f32,
                          sigma_base: 2.620f64 as f32,
                          sigma_slope: 0.172f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.288f64 as f32,
                          const_slope: 0.016f64 as f32,
                          sigma_base: 2.074f64 as f32,
                          sigma_slope: 0.172f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.034f64 as f32,
                          const_slope: 0.063f64 as f32,
                          sigma_base: 1.591f64 as f32,
                          sigma_slope: 0.193f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.093f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.146f64 as f32,
                          sigma_slope: 0.187f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.345f64 as f32,
                          const_slope: 0.065f64 as f32,
                          sigma_base: 0.578f64 as f32,
                          sigma_slope: 0.207f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.097f64 as f32,
                          const_slope: 0.005f64 as f32,
                          sigma_base: 0.152f64 as f32,
                          sigma_slope: 0.218f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.541f64 as f32,
                          const_slope: 0.071f64 as f32,
                          sigma_base: -0.028f64 as f32,
                          sigma_slope: 0.210f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.218f64 as f32,
                          const_slope: -0.013f64 as f32,
                          sigma_base: -0.453f64 as f32,
                          sigma_slope: 0.227f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.301f64 as f32,
                          const_slope: 0.044f64 as f32,
                          sigma_base: 2.540f64 as f32,
                          sigma_slope: 0.184f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.417f64 as f32,
                          const_slope: -0.013f64 as f32,
                          sigma_base: 1.850f64 as f32,
                          sigma_slope: 0.189f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.007f64 as f32,
                          const_slope: 0.036f64 as f32,
                          sigma_base: 1.460f64 as f32,
                          sigma_slope: 0.208f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.032f64 as f32,
                          const_slope: -0.008f64 as f32,
                          sigma_base: 1.208f64 as f32,
                          sigma_slope: 0.197f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.359f64 as f32,
                          const_slope: 0.048f64 as f32,
                          sigma_base: 0.586f64 as f32,
                          sigma_slope: 0.210f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.145f64 as f32,
                          const_slope: -0.014f64 as f32,
                          sigma_base: 0.144f64 as f32,
                          sigma_slope: 0.225f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.556f64 as f32,
                          const_slope: 0.064f64 as f32,
                          sigma_base: -0.101f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.326f64 as f32,
                          const_slope: -0.029f64 as f32,
                          sigma_base: -0.692f64 as f32,
                          sigma_slope: 0.258f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.227f64 as f32,
                          const_slope: 0.073f64 as f32,
                          sigma_base: 2.319f64 as f32,
                          sigma_slope: 0.199f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.456f64 as f32,
                          const_slope: 0.007f64 as f32,
                          sigma_base: 1.794f64 as f32,
                          sigma_slope: 0.191f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.112f64 as f32,
                          const_slope: 0.053f64 as f32,
                          sigma_base: 1.469f64 as f32,
                          sigma_slope: 0.211f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.186f64 as f32,
                          const_slope: -0.005f64 as f32,
                          sigma_base: 1.003f64 as f32,
                          sigma_slope: 0.216f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.077f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 0.389f64 as f32,
                          sigma_slope: 0.238f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.467f64 as f32,
                          const_slope: -0.032f64 as f32,
                          sigma_base: -0.004f64 as f32,
                          sigma_slope: 0.248f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.032f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 0.029f64 as f32,
                          sigma_slope: 0.215f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.807f64 as f32,
                          const_slope: -0.070f64 as f32,
                          sigma_base: -0.858f64 as f32,
                          sigma_slope: 0.273f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.139f64 as f32,
                          const_slope: 0.120f64 as f32,
                          sigma_base: 2.337f64 as f32,
                          sigma_slope: 0.209f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.272f64 as f32,
                          const_slope: 0.044f64 as f32,
                          sigma_base: 1.885f64 as f32,
                          sigma_slope: 0.186f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.052f64 as f32,
                          const_slope: 0.096f64 as f32,
                          sigma_base: 1.451f64 as f32,
                          sigma_slope: 0.225f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.095f64 as f32,
                          const_slope: 0.029f64 as f32,
                          sigma_base: 0.899f64 as f32,
                          sigma_slope: 0.229f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.299f64 as f32,
                          const_slope: 0.084f64 as f32,
                          sigma_base: 0.326f64 as f32,
                          sigma_slope: 0.258f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.232f64 as f32,
                          const_slope: 0.004f64 as f32,
                          sigma_base: -0.197f64 as f32,
                          sigma_slope: 0.281f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.718f64 as f32,
                          const_slope: 0.093f64 as f32,
                          sigma_base: -0.061f64 as f32,
                          sigma_slope: 0.242f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: -0.091f64 as f32,
                          const_slope: 0.008f64 as f32,
                          sigma_base: -0.974f64 as f32,
                          sigma_slope: 0.297f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }],
     [{
          let init =
              Correlation{const_base: 0.000f64 as f32,
                          const_slope: 0.000f64 as f32,
                          sigma_base: 0.000f64 as f32,
                          sigma_slope: 0.000f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.674f64 as f32,
                          const_slope: 0.069f64 as f32,
                          sigma_base: 2.236f64 as f32,
                          sigma_slope: 0.147f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.769f64 as f32,
                          const_slope: 0.018f64 as f32,
                          sigma_base: 1.900f64 as f32,
                          sigma_slope: 0.185f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.281f64 as f32,
                          const_slope: 0.041f64 as f32,
                          sigma_base: 1.415f64 as f32,
                          sigma_slope: 0.224f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.842f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: 1.152f64 as f32,
                          sigma_slope: 0.230f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.446f64 as f32,
                          const_slope: -0.003f64 as f32,
                          sigma_base: -0.167f64 as f32,
                          sigma_slope: 0.316f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 1.297f64 as f32,
                          const_slope: -0.102f64 as f32,
                          sigma_base: -0.467f64 as f32,
                          sigma_slope: 0.302f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.433f64 as f32,
                          const_slope: -0.033f64 as f32,
                          sigma_base: -0.813f64 as f32,
                          sigma_slope: 0.304f64 as f32,};
          init
      },
      {
          let init =
              Correlation{const_base: 0.967f64 as f32,
                          const_slope: -0.112f64 as f32,
                          sigma_base: -0.345f64 as f32,
                          sigma_slope: 0.255f64 as f32,};
          init
      }]];
