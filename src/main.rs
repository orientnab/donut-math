use std::f32::consts::PI;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 60;

// const THETA_SPACING: f32 = 0.07;
// const PHI_SPACING: f32 = 0.02;
const THETA_PITCH: usize = 80;
const PHI_PITCH: usize = 315;

// TODO: Refactor so this name make some sense
const R1: f32 = 1.0;
const R2: f32 = 2.0;
const K2: f32 = 5.0;
const K1: f32 = SCREEN_WIDTH as f32 * K2 * 3.0 / (8.0 * (R1 + R2));

const CHARACTERS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

fn render_frame(a: f32, b: f32) {
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    // TODO: optimise in size for 1byte chars instead of 4byte chars.
    let mut output: Vec<Vec<char>> = vec![vec![' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
    let mut zbuffer: Vec<Vec<f32>> = vec![vec![0.0; SCREEN_HEIGHT]; SCREEN_WIDTH];

    // theta goes around the cross-sectional circle of a torus
    for angle_theta in 0..THETA_PITCH {
        let theta = 2.0 * PI / THETA_PITCH as f32 * angle_theta as f32;
        let costheta = theta.cos();
        let sintheta = theta.sin();

        // phi goes around the center of revolution of a torus
        for angle_phi in 0..PHI_PITCH {
            let phi = 2.0 * PI / PHI_PITCH as f32 * angle_phi as f32;
            let cosphi = phi.cos();
            let sinphi = phi.sin();

            let circlex = R2 + R1 * costheta;
            let circley = R1 * sintheta;

            let x = circlex * (cos_b * cosphi + sin_a * sin_b * sinphi) - circley * cos_a * sin_b;
            let y = circlex * (sin_b * cosphi - sin_a * cos_b * sinphi) + circley * cos_a * cos_b;
            let z = K2 + cos_a * circlex * sinphi + circley * sin_a;
            let ooz = 1.0 / z;

            let xp = (SCREEN_WIDTH as i32 / 2 + (K1 * ooz * x) as i32) as usize;
            // TODO: The index yp goes out of bounds for certain values.
            // This is just a hack so the value is clamped.
            // The real expression should be something like this
            // let yp = (SCREEN_HEIGHT / 2 + 1) - (K1 * ooz * y).round() as usize;
            let yp = std::cmp::min(
                std::cmp::max(
                    (SCREEN_HEIGHT as f32 / 2.0 - (K1 * ooz * y)).round() as i32,
                    0,
                ),
                SCREEN_HEIGHT as i32 - 1,
            ) as usize;

            let luminance =
                sin_b * cosphi * costheta - cos_a * sinphi * costheta - sin_a * sintheta
                    + cos_b * (cos_a * sintheta - sin_a * sinphi * costheta);
            // If it's < 0, the surface is pointing away from us, so we don't plot it.
            if luminance > 0.0 {
                if ooz > zbuffer[xp][yp] {
                    zbuffer[xp][yp] = ooz;
                    let luminance_index = (8.0 * luminance).floor() as usize;
                    // luminance_index is now in the range 0..11 (8*sqrt(2) = 11.3)
                    // now we lookup the character corresponding to the
                    // luminance and plot it in our output:
                    output[xp][yp] = CHARACTERS[luminance_index];
                }
            }
        }
    }
    print!("\x1b[H"); // bring cursor to "home" location, in ansi compliant terminals
    for j in 0..SCREEN_HEIGHT {
        for i in 0..SCREEN_WIDTH {
            print!("{}", output[i][j]);
        }
        println!("");
    }
}

fn main() {
    let mut a = 0.0;
    let mut b = 0.0;
    loop {
        a += 0.02;
        b += 0.01;
        render_frame(a, b);
    }
}
