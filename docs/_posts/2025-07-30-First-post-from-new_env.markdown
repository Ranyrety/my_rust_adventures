---
layout: post
title:  "Zaczynajmy"
date:   2025-07-30 14:55:44 +0200
categories: general update
---

# Wstep

Jest to pierwszy post w tym projekcie. Zamysl przy starcie 
byl taki. 

1. Znalezc nowy edytor i skonfirugorwac srodowisko wykorzystujace terminal i sie do niego pryzwyczaic. Pierwszy wybor padl na neovim i tak zostalo.
2. Rozpoczac nauke Rusta.
3. Pocwiczyc podzial duzych zadan na mniejsze i jak szacowanie ile potrwaja ma sie do aktualnego wykonania.

Ostatni punkt byl najwaznejsze podczas poczatkow planowania. Z niegopojawily sie dodatkowe poniewaz  nowy srodowisko i nowy jezyk programowania dodaja dodatkowych komplikacji przy szacowaniu.
  Sam projekt to nauka Rusta korzystajac raylib do tworzenia prostych gier. Zaczynajac od Ponga do bardziej skomplikowanych. Propozycje gier, pierwszy podzial na duze i srednie fragmenty oraz ich oszacowanie przygotowalem z chate gpt. Powinien być dostępny tutaj [generate_tasks.md]({% link generate_tasks.md %})
Częscią każdego zadania jets napisanie posta z podsumowaniem i podaniem następnego zadania. To już się nie udało. Najpierw problemy z środowkiskiem zepsuły pierwszą wersje tego posta? I jako test ponowny zacząłem prace na dzadaniem pierwszym i już skonczyłem ale już nie miałe sił na napisanie podsumowania.

# O pierwszym zadania

1. Create new Rust project (cargo new pong_clone). (5 min)
2. Add raylib crate to Cargo.toml. (5 min)
3. Write basic main.rs to initialize a window and set FPS. (20 min)
4. Run and verify empty window. (15 min)
Raylib/Rust Concepts: cargo new, Cargo.toml dependencies, raylib::init(), RaylibHandle, RaylibThread, set_target_fps().

5. Define & Draw Game Objects (45 min)
Intermediate Steps:
  - Create Paddle struct (position, size, color). (15 min)
  - Create Ball struct (position, radius, velocity, color). (15 min)
  - Implement drawing logic for paddles and ball using draw_rectangle() and draw_circle() within the game loop. (15 min)
  Raylib/Rust Concepts: struct, Vector2, Color, RaylibDrawHandle::draw_rectangle(), draw_circle().

6. Player Input for Paddles (45 min)
Intermediate Steps:
  - Add update() method to Paddle struct. (15 min)
  - Inside update(), check is_key_down() for KEY_UP/KEY_DOWN (player 1) and KEY_W/KEY_S (player 2). (15 min)
  - Adjust paddle y position based on input. (15 min)
Raylib/Rust Concepts: RaylibHandle::is_key_down(), KeyboardKey enum.

Kroki 1 do 4 zostały wykonane podczas tworzenia pierwszej strultury katalogu oraz przy testach środowiska. Nastepne zostały wykonane w Niedziele, Zadanie 5 zajęło więcej niżplanowane 45 minut w zwiazku z rozwiązywaniem problemów poruszania sie po repozytoriach z przykładami oraz definowania ogólneo procesu pracy. Sam proces aktualnei wygląda tak. Zapoznanie się z zadaniami, spradzenie w dokumentacji i przykłądach jak wyglądaja wymagane funkcje, i jak są wykorzystane. Aktalnie zadanie jest użyte trochę inaczej pominąłem fukncję update w paddle. Zadanie 6 zajęło znacznie mniej niż planowane tak, że całośc z dodatkowym fragmentem zajęła około 1:30 a samo dodanie obługi klawiszy zajeło 25 minut z czego większa częśc na porawkach litrówek oraz znalezieniu odpowiedniej bilbioteki i dodaniu use. Częściowo już zaczałem prace nad poruszaniem się piłki ale jest to wstepnie i dalszej części zadania nie zrobiłem. Ponizej plan na kolejne zadanie.

1. Ball Movement & Wall Collisions (45 min)
Intermediate Steps:
  - Add update() method to Ball struct. (15 min)
  - Update ball position based on its velocity. (10 min)
  - Implement collision logic for top and bottom walls, reversing y velocity. (20 min)
Raylib/Rust Concepts: Basic arithmetic, Vector2.




