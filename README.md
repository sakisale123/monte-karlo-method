# Analiza serijske i paralelne implementacije algoritama baziranih na Monte-Karlo metodi

  Problem: 
Aproksimacija broja Pi
Problem se rešava simulacijom nasumičnog bacanja tačaka u kvadrat stranice 2, unutar kojeg je upisan krug poluprečnika 1.
Vrednost Pi se računa po formuli: Pi = 4*N(pogodaka)/N(ukupno)
Gde je N(pogodaka) broj tacaka koje zadovoljavaju uslov x(2) + y(2) <= 1

  Metodologija rada:
U okviru projekta biće implementirana dva rešenja:
Serijska (sekvencijalna) implementacija:
Algoritam se izvršava na jednom procesorskom jezgru.
Program prati napredak kroz iteracije i beleži promenu procenjene vrednosti Pi u izlaznu datoteku. Ovo omogućava uvid u to kako se preciznost povećava sa brojem uzoraka.
Paralelna implementacija:
Ukupan broj uzoraka (npr. 100 miliona tačaka) deli se na više niti koristeći Rust-ov model konkurentnosti (std::thread).
Svaka nit nezavisno računa broj pogodaka za svoj set podataka.
Glavna nit (main thread) sakuplja rezultate iz svih niti i vrši finalni proračun.
Rezultati se beleže u izlaznu datoteku radi provere konzistentnosti sa serijskom verzijom.

