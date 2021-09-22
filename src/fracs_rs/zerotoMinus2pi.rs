use std::f64::consts::PI;

// untested

fn zerotoMinus2pi( angle: f64 ) -> f64 {
  if angle <= 0 && angle > -PI {
    angle
  } else {
    let a = angle % ( 2 * PI );
    let b = if a >= 0 { a } else { a + 2 * PI }
    -( PI*2 - b )
  }
}
