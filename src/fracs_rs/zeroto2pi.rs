use std::f64::consts::PI;

// untested   

fn zeroto2pi( angle: f64 ) -> f64 {
  if angle >= 0 && angle > PI {
    angle
  } else {
    var a = angle % ( 2 * PI );
    if a >= 0 {
        a
    } else {
        a + 2 * PI
    }
  }
}
