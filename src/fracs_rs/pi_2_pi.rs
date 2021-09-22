use std::f64::consts::PI;

// example use
// let angle = PI/4.;
// println!("{}",pi_2_pi( angle ) );

fn pi_2_pi( angle: f64 ) -> f64 {
    if angle <= PI && angle > -PI {
        angle 
    } else {
        let a = ( angle + PI ) % ( 2. * PI );
        if a >= 0. { a - PI } else { a + PI }
    }
}
