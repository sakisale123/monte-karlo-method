# Analiza serijske i paralelne implementacije algoritama baziranih na Monte-Karlo metodi

Ciljani broj poena: 60 poena (maksimalna ocena 6)
Tehnologija: Rust

  Problem: 
Aproksimacija broja Pi
Problem se rešava simulacijom nasumičnog bacanja tačaka u kvadrat stranice 2, unutar kojeg je upisan krug poluprečnika 1.
Vrednost Pi se računa po formuli: Pi = 4*N(pogodaka)/N(ukupno)
Gde je N(pogodaka) broj tacaka koje zadovoljavaju uslov x^2 + y^2 <= 1

  Metodologija rada:
U okviru projekta biće implementirana dva rešenja:

Serijska (sekvencijalna) implementacija:
Program se izvršava na jednoj niti (Main thread).
Koristi se generator pseudoslučajnih brojeva (rand biblioteka) za generisanje koordinata tačaka.
Izlaz: Program će kreirati datoteku (npr. serial_log.txt) u koju će se upisivati trenutna procenjena vrednost broja Pi na svakih K iteracija, kako bi se demonstriralo konvergiranje rešenja ka tačnoj vrednosti.

Paralelna implementacija:
Program koristi std::thread modul za kreiranje više niti.
Ukupan broj iteracija N se deli na T niti (npr. 4, 8 ili 16 niti).
Svaka nit poseduje sopstveni lokalni brojač pogodaka i sopstvenu instancu generatora slučajnih brojeva kako bi se izbeglo zaključavanje (mutex contention) i omogućilo potpuno paralelno izvršavanje.
Glavna nit čeka završetak svih sporednih niti (join), sabira njihove parcijalne rezultate i računa finalnu vrednost broja Pi.
Izlaz: Rezultati i vreme izvršavanja se upisuju u izlaznu datoteku radi poređenja sa serijskom verzijom.


