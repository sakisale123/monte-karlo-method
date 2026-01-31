use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
    sekvencijalno();
}

fn sekvencijalno() {
    let mut rng = rand::thread_rng();
    let all_dots = 1000000;

    let mut file = File::create("izlaz_sekvenciajlni").expect("Ne mogu da kreiram fajl");
    ;
    writeln!(file,"Trenutna procena PI:").expect("Ne moze da se upise");

    let mut inside_dot = 0;

    for i in 1..=all_dots {
        let x: f64 = rng.gen_range(-1.0..1.0);
        let y: f64 = rng.gen_range(-1.0 .. 1.0);

        let square_distance = x.powi(2) + y.powi(2);

        if square_distance <= 1.0 {
            // println!("Tacka ({},{}) je UNUTAR kruga",x,y);
            inside_dot += 1;
        } else {
            // println!("Tacka ({},{}) je VAN kruga",x,y)
        }

        if i % 100000 == 0 {
            let current_pi = 4.0 * (inside_dot as f64) / (all_dots as f64);
            writeln!(file, "Na {} uzoraka, procena je {}", i, current_pi).expect("Greska pri upisivanju");
        }
    }

    let estimate_pi = 4.0 * (inside_dot as f64) / (all_dots as f64);

    println!("Nakon {} uzoraka, procena PI je {}",all_dots,estimate_pi);
}



fn paralelno() {

}
