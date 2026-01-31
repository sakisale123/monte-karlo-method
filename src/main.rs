use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let all_dots = 1000000;

    let mut inside_dot = 0;

    for _ in 1..all_dots {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0 .. 1.0);

        let square_distance = x.powi(2) + y.powi(2);

        if square_distance <= 1.0 {
            // println!("Tacka ({},{}) je UNUTAR kruga",x,y);
            inside_dot += 1;
        } else {
            // println!("Tacka ({},{}) je VAN kruga",x,y)
        }
    }

    let estimate_pi = 4.0 * (inside_dot as f64) / (all_dots as f64);

    println!("Nakon {} uzoraka, procena PI je {}",all_dots,estimate_pi);
}
