use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Instant;

fn main() {
    println!("---Pokretanje MonteKarlo racunanja---");

    let seq_time_start = Instant::now();
    sekvencijalno();
    let seq_time_end = seq_time_start.elapsed();

    let par_time_start = Instant::now();
    paralelno();
    let par_time_end = par_time_start.elapsed();

    println!("--REZULTATI--");
    println!("Vreme sekvencijalnog: {:?}", seq_time_end);
    println!("Vreme paralelnog: {:?}", par_time_end);

    let speedup = seq_time_end.as_secs_f64() / par_time_end.as_secs_f64();
    println!("Ubrzanje je {:.2}x",speedup);
}

fn sekvencijalno() {
    let mut rng = rand::thread_rng();
    let all_dots = 10000000;

    let mut file = File::create("izlaz_sekvenciajlni").expect("Ne mogu da kreiram fajl");

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
    let all_dots = 10000000;
    let threads = 4;
    let dots_per_threads = all_dots/threads;

    let mut handles = vec![];

    let mut file = File::create("izlaz_paralelni").expect("Ne mogu da kreiram fajl");

    writeln!(file,"Trenutna procena PI:").expect("Ne moze da se upise");

    let mut rng = rand::thread_rng();
    let mut inside_dot = 0;

    for _ in 1..=threads {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();

            for _ in 0..dots_per_threads{
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

            inside_dot

        });

        handles.push(handle);

    }

    let mut total_hits = 0;
    for handle in handles{
        let hits = handle.join().expect("Nit je pukla");
        total_hits += hits;
    }

    let estimate_pi = 4.0 * (total_hits as f64) / (all_dots as f64);

    writeln!(file,"Nakon {} uzoraka, procena PI je {}",all_dots,estimate_pi).expect("Greska pri upisivanju.");
}
