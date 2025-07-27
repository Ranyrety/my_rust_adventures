Raylib and Rust Game Development Roadmap: Step-by-Step Progression
This roadmap provides a structured approach to building games with Raylib and Rust, breaking down each project into high-level tasks (under 1 hour each) and their estimated intermediate steps.

1. Pong Clone
Difficulty: Beginner

Total Estimated Time: 5-7 hours

High-Level Tasks (Max 1 hour each):

Project Setup & Window Initialization (45 min)

Intermediate Steps:

Create new Rust project (cargo new pong_clone). (5 min)

Add raylib crate to Cargo.toml. (5 min)

Write basic main.rs to initialize a window and set FPS. (20 min)

Run and verify empty window. (15 min)

Raylib/Rust Concepts: cargo new, Cargo.toml dependencies, raylib::init(), RaylibHandle, RaylibThread, set_target_fps().

Define & Draw Game Objects (45 min)

Intermediate Steps:

Create Paddle struct (position, size, color). (15 min)

Create Ball struct (position, radius, velocity, color). (15 min)

Implement drawing logic for paddles and ball using draw_rectangle() and draw_circle() within the game loop. (15 min)

Raylib/Rust Concepts: struct, Vector2, Color, RaylibDrawHandle::draw_rectangle(), draw_circle().

Player Input for Paddles (45 min)

Intermediate Steps:

Add update() method to Paddle struct. (15 min)

Inside update(), check is_key_down() for KEY_UP/KEY_DOWN (player 1) and KEY_W/KEY_S (player 2). (15 min)

Adjust paddle y position based on input. (15 min)

Raylib/Rust Concepts: RaylibHandle::is_key_down(), KeyboardKey enum.

Ball Movement & Wall Collisions (45 min)

Intermediate Steps:

Add update() method to Ball struct. (15 min)

Update ball position based on its velocity. (10 min)

Implement collision logic for top and bottom walls, reversing y velocity. (20 min)

Raylib/Rust Concepts: Basic arithmetic, Vector2.

Paddle Collision & Scoring (1 hour)

Intermediate Steps:

Implement check_collision_circle_rec() between ball and each paddle. (20 min)

On collision, reverse ball x velocity. (15 min)

Add score variables for each player. (10 min)

Detect when ball goes off-screen left/right, update score for opposing player, and reset ball position. (15 min)

Raylib/Rust Concepts: check_collision_circle_rec(), i32 for scores.

Display Score & Game Over (45 min)

Intermediate Steps:

Use draw_text() to display scores on screen. (20 min)

Implement a simple "Game Over" state when a score limit is reached. (15 min)

Display "Game Over" message and prompt to restart. (10 min)

Raylib/Rust Concepts: RaylibDrawHandle::draw_text(), format!(), enum GameState.

Refinement & Basic Sounds (1 hour)

Intermediate Steps:

Adjust speeds and paddle sizes for better gameplay. (20 min)

Load a simple sound effect for ball-paddle collision. (20 min)

Play sound on collision using play_sound(). (20 min)

Raylib/Rust Concepts: RaylibHandle::load_sound(), play_sound(), unload_sound().

2. Asteroids-like Shooter
Difficulty: Easy

Total Estimated Time: 7-10 hours

High-Level Tasks (Max 1 hour each):

Project Setup & Spaceship Drawing (1 hour)

Intermediate Steps:

New Rust project with raylib crate. (15 min)

Initialize window. (10 min)

Load a spaceship texture (placeholder image or simple shape). (20 min)

Draw the spaceship texture at a fixed position. (15 min)

Raylib/Rust Concepts: load_texture(), draw_texture().

Spaceship Movement & Rotation (1 hour)

Intermediate Steps:

Define Player struct (position, velocity, rotation, speed). (15 min)

Implement input for rotation (KEY_LEFT/KEY_RIGHT) and thrust (KEY_UP). (20 min)

Update position based on velocity and rotation. (15 min)

Draw spaceship using draw_texture_ex() or draw_texture_pro() to apply rotation. (10 min)

Raylib/Rust Concepts: Vector2, draw_texture_ex(), get_frame_time().

Bullet Spawning & Movement (1 hour)

Intermediate Steps:

Define Bullet struct (position, velocity, radius). (15 min)

On KEY_SPACE press, create a new bullet with velocity based on player's rotation. (20 min)

Store bullets in a Vec<Bullet>. (10 min)

Update all active bullets' positions in the game loop. (15 min)

Raylib/Rust Concepts: Vec, push().

Asteroid Spawning & Basic Movement (1 hour)

Intermediate Steps:

Define Asteroid struct (position, velocity, radius, size_tier). (15 min)

Implement a function to randomly spawn initial asteroids off-screen. (20 min)

Update all active asteroids' positions. (15 min)

Draw asteroids (simple circles or placeholder textures). (10 min)

Raylib/Rust Concepts: rand crate, Vector2.

Collision Detection (Ship-Asteroid, Bullet-Asteroid) (1 hour)

Intermediate Steps:

Implement check_collision_circles() for player-asteroid. (20 min)

Implement check_collision_circles() for bullet-asteroid. (20 min)

Handle removal of hit bullets and asteroids. (20 min)

Raylib/Rust Concepts: check_collision_circles(), Vec::retain().

Asteroid Splitting & Game Over (1 hour)

Intermediate Steps:

When a large asteroid is hit, spawn 2-3 smaller asteroids. (30 min)

Implement game over condition (player-asteroid collision). (15 min)

Display "Game Over" text. (15 min)

Raylib/Rust Concepts: enum AsteroidSize, draw_text().

Sound Effects & Polish (1 hour)

Intermediate Steps:

Load sound effects for shooting, asteroid explosion, and player explosion. (25 min)

Play sounds at appropriate times. (20 min)

Adjust game parameters (speeds, spawn rates) for balance. (15 min)

Raylib/Rust Concepts: load_sound(), play_sound().

3. Simple Top-Down Shooter (e.g., "Arena Shooter")
Difficulty: Medium

Total Estimated Time: 8-12 hours

High-Level Tasks (Max 1 hour each):

Core Game Loop & Player Setup (1 hour)

Intermediate Steps:

New project, window initialization. (15 min)

Define Player struct (position, health, speed, direction). (20 min)

Draw player as a simple rectangle/circle. (10 min)

Implement basic player movement (WASD or arrow keys). (15 min)

Raylib/Rust Concepts: Vector2, is_key_down().

Enemy Spawning & Basic Movement (1 hour)

Intermediate Steps:

Define Enemy struct (position, health, speed). (15 min)

Implement a simple enemy spawning mechanism. (20 min)

Enemies move randomly or in a fixed direction. (25 min)

Raylib/Rust Concepts: rand crate, Vector2.

Player Shooting & Projectiles (1 hour)

Intermediate Steps:

Define Projectile struct (position, velocity, damage). (15 min)

On mouse click, create a projectile from player's position towards mouse. (25 min)

Update and draw projectiles. (20 min)

Raylib/Rust Concepts: get_mouse_position(), Vector2::normalize().

Collision: Projectile-Enemy & Damage (1 hour)

Intermediate Steps:

Implement check_collision_recs() for projectiles and enemies. (25 min)

If collision, reduce enemy health by projectile damage. (20 min)

Remove projectile and defeated enemy. (15 min)

Raylib/Rust Concepts: check_collision_recs(), Vec::retain().

Collision: Player-Enemy & Player Damage (1 hour)

Intermediate Steps:

Implement check_collision_recs() for player and enemies. (25 min)

If collision, reduce player health. (20 min)

Implement a cooldown/invincibility frame for player after being hit. (15 min)

Raylib/Rust Concepts: check_collision_recs(), timer logic.

Health Bars & Score Display (1 hour)

Intermediate Steps:

Draw health bars above enemies. (30 min)

Draw player's health bar/text on screen. (15 min)

Add and display score for defeating enemies. (15 min)

Raylib/Rust Concepts: draw_rectangle(), draw_text().

Basic Enemy AI (Chase) (1 hour)

Intermediate Steps:

Modify enemy update() to calculate direction towards player. (30 min)

Move enemy towards player. (20 min)

Implement a simple "aggro range" for enemies. (10 min)

Raylib/Rust Concepts: Vector2::distance(), Vector2::normalize().

Game Over & Restart (45 min)

Intermediate Steps:

Detect player health reaching zero. (15 min)

Transition to a "Game Over" state. (15 min)

Display restart prompt. (15 min)

Raylib/Rust Concepts: enum GameState.

4. Platformer with Basic Physics
Difficulty: Medium-Hard

Total Estimated Time: 10-15 hours

High-Level Tasks (Max 1 hour each):

Project Setup & Player Sprite (1 hour)

Intermediate Steps:

New project, window initialization. (15 min)

Load a player sprite texture (single standing frame). (20 min)

Define Player struct (position, velocity, on_ground). (15 min)

Draw player sprite. (10 min)

Raylib/Rust Concepts: load_texture(), draw_texture().

Basic Gravity & Jumping (1 hour)

Intermediate Steps:

Apply constant downward acceleration (gravity) to player's y_velocity. (20 min)

Update player y position based on y_velocity. (15 min)

On KEY_SPACE press, apply upward y_velocity for jump. (25 min)

Raylib/Rust Concepts: get_frame_time(), basic physics equations.

Ground Collision Detection (1 hour)

Intermediate Steps:

Define Platform struct (rectangle). (15 min)

Create a few static platforms. (10 min)

Implement check_collision_recs() between player and platforms. (25 min)

If colliding from above, set on_ground = true, reset y_velocity to 0, and adjust player position to sit on platform. (10 min)

Raylib/Rust Concepts: check_collision_recs().

Horizontal Movement & Friction (1 hour)

Intermediate Steps:

Implement KEY_LEFT/KEY_RIGHT for horizontal x_velocity. (25 min)

Apply a small "friction" force to gradually reduce x_velocity when no input. (20 min)

Update player x position. (15 min)

Raylib/Rust Concepts: get_frame_time().

Camera Following Player (1 hour)

Intermediate Steps:

Initialize Camera2D struct. (20 min)

Set camera target to player's position. (20 min)

Begin drawing with begin_mode_2d() and end with end_mode_2d(). (20 min)

Raylib/Rust Concepts: Camera2D, begin_mode_2d(), end_mode_2d().

Basic Tile-based Level Layout (1 hour)

Intermediate Steps:

Represent level as a 2D array of tile types (e.g., Vec<Vec<u8>>). (20 min)

Draw tiles using draw_rectangle() or a single tile texture. (25 min)

Adjust collision detection to iterate over relevant tiles. (15 min)

Raylib/Rust Concepts: Vec<Vec>, draw_rectangle() or draw_texture_rec().

Collecting Items (45 min)

Intermediate Steps:

Define Collectable struct (position, type). (15 min)

Place some collectables in the level. (10 min)

Implement check_collision_recs() between player and collectables. (10 min)

Remove collected items and update a score/counter. (10 min)

Raylib/Rust Concepts: Vec::retain().

Simple Enemy (Patrol) (1 hour)

Intermediate Steps:

Define Enemy struct (position, patrol_points, current_target_index). (20 min)

Implement enemy movement between patrol points. (25 min)

Add basic player-enemy collision (e.g., player takes damage, or enemy is defeated). (15 min)

Raylib/Rust Concepts: Vector2::distance().

5. Simple Roguelike
Difficulty: Hard

Total Estimated Time: 12-18 hours

High-Level Tasks (Max 1 hour each):

Project Setup & Map Data Structure (1 hour)

Intermediate Steps:

New project, window init. (15 min)

Define TileType enum (Wall, Floor). (10 min)

Create Map struct holding Vec<Vec<TileType>> for the dungeon grid. (20 min)

Initialize map with all walls. (15 min)

Raylib/Rust Concepts: enum, Vec<Vec>.

Basic Dungeon Generation (Drunkard's Walk) (1 hour)

Intermediate Steps:

Implement a "Drunkard's Walk" algorithm to carve out paths and rooms. (40 min)

Fill map with generated floors and walls. (20 min)

Rust Concepts: rand crate, loop, conditional logic.

Draw Map (ASCII or Simple Tiles) (1 hour)

Intermediate Steps:

Iterate through the Map grid. (20 min)

Draw . for floor, # for wall using draw_text(). (25 min)

Set up a fixed-size font. (15 min)

Raylib/Rust Concepts: draw_text(), load_font().

Player & Turn-Based Movement (1 hour)

Intermediate Steps:

Define Player struct (position, health, stats). (15 min)

Place player on a random floor tile. (10 min)

Implement turn-based movement: on arrow key press, move player one tile if floor. (25 min)

Introduce a TurnState enum (PlayerTurn, EnemyTurn). (10 min)

Raylib/Rust Concepts: is_key_pressed().

Basic Enemy & Turn-Based AI (1 hour)

Intermediate Steps:

Define Enemy struct (position, health, type). (15 min)

Spawn a few enemies. (10 min)

In EnemyTurn state, iterate through enemies. (15 min)

For each enemy, move randomly or towards player if in range. (20 min)

Rust Concepts: enum, basic AI logic.

Combat System (1 hour)

Intermediate Steps:

If player tries to move into enemy tile, initiate combat. (20 min)

Calculate damage (player attack vs enemy defense). (15 min)

Reduce enemy health, remove if defeated. (15 min)

Display combat messages (e.g., "You hit the Goblin for 5 damage!"). (10 min)

Rust Concepts: Basic arithmetic, string formatting.

Field of View (Fog of War) (1 hour)

Intermediate Steps:

Implement a simple FOV algorithm (e.g., recursive shadowcasting). (40 min)

Only draw tiles within the player's FOV. (20 min)

Rust Concepts: Algorithms, HashSet or Vec<Vector2> for visible tiles.

Simple Inventory & Item Pickup (1 hour)

Intermediate Steps:

Define Item struct (name, type, effect). (15 min)

Place items on floor tiles. (10 min)

When player moves onto item tile, add to player's Vec<Item> inventory. (20 min)

Display inventory list. (15 min)

Rust Concepts: Vec, enum ItemType.

Player Stats & Leveling (1 hour)

Intermediate Steps:

Add XP to player when enemies are defeated. (20 min)

Implement a leveling up system (e.g., every 100 XP, increase max health/attack). (25 min)

Display player stats. (15 min)

Rust Concepts: i32 for stats.

6. Top-Down Adventure Game (Mini-RPG)
Difficulty: Very Hard

Total Estimated Time: 20-30+ hours

High-Level Tasks (Max 1 hour each):

Project Setup & Basic Tilemap Rendering (1 hour)

Intermediate Steps:

New project, window init. (15 min)

Load a simple tilesheet texture. (20 min)

Define a small hardcoded 2D array representing a map. (15 min)

Draw the map by iterating the array and using draw_texture_rec(). (10 min)

Raylib/Rust Concepts: load_texture(), draw_texture_rec().

Player Movement & Basic Sprite Animation (1 hour)

Intermediate Steps:

Define Player struct (position, speed, current_animation_state). (15 min)

Load a spritesheet for player (e.g., 4 directions, 3 frames each). (20 min)

Implement movement (WASD). (15 min)

Change animation frame based on time and movement direction. (10 min)

Raylib/Rust Concepts: draw_texture_rec(), get_frame_time(), Rectangle for sprite frames.

Tilemap Collision (1 hour)

Intermediate Steps:

Define which tiles are "walkable" in the map data. (20 min)

Before moving the player, check if the target tile is walkable. (25 min)

Prevent player from moving onto non-walkable tiles. (15 min)

Rust Concepts: Conditional logic, map indexing.

NPC Entity & Basic Interaction (1 hour)

Intermediate Steps:

Define NPC struct (position, name, dialogue_id). (20 min)

Place a few NPCs on the map. (10 min)

Detect player proximity to NPC. (15 min)

On KEY_E press near NPC, trigger a simple "Hello!" message. (15 min)

Raylib/Rust Concepts: check_collision_recs(), draw_text().

Simple Dialogue System (1 hour)

Intermediate Steps:

Define DialogueNode struct (text, options). (20 min)

Create a small, linear dialogue tree for an NPC. (20 min)

Display dialogue text and allow player to advance with a key press. (20 min)

Rust Concepts: enum DialogueState, Vec<String>.

Inventory Data Structure & UI (1 hour)

Intermediate Steps:

Define Item struct (name, description, icon_rect). (15 min)

Define Inventory struct (Vec<Option<Item>> for slots, capacity). (15 min)

Draw a simple inventory grid UI (rectangles for slots). (20 min)

Draw item icons/names in slots. (10 min)

Raylib/Rust Concepts: draw_rectangle(), draw_texture_rec(), draw_text().

Item Pickup & Inventory Management (1 hour)

Intermediate Steps:

Place items on the map. (10 min)

On player collision/interaction with item, add to inventory if space. (20 min)

Remove item from map. (10 min)

Implement basic inventory interaction (e.g., pressing I to toggle inventory UI). (20 min)

Rust Concepts: Vec::retain().

Basic Quest System (1 hour)

Intermediate Steps:

Define Quest struct (ID, title, description, objectives). (20 min)

Define Objective enum (e.g., TalkToNpc(NpcId), CollectItem(ItemId, u32)). (15 min)

Start a simple quest through NPC dialogue. (15 min)

Display current quest in a UI element. (10 min)

Rust Concepts: enum, struct, Vec.

Quest Progress Tracking & Completion (1 hour)

Intermediate Steps:

Implement logic to update quest objectives (e.g., increment item count on pickup, mark NPC as talked to). (30 min)

Check if all objectives for a quest are met. (15 min)

Mark quest as completed and provide a simple reward message. (15 min)

Rust Concepts: Conditional logic, state updates.

Game State Saving (1 hour)

Intermediate Steps:

Identify all game state data that needs to be saved (player, inventory, quests, map changes). (30 min)

Use serde to serialize relevant structs to JSON. (20 min)

Write JSON string to a file using std::fs::write(). (10 min)

Rust Concepts: serde, serde_json, std::fs::write().

Game State Loading (1 hour)

Intermediate Steps:

Implement a "Load Game" option on the main menu. (15 min)

Read JSON string from file using std::fs::read_to_string(). (15 min)

Use serde to deserialize JSON back into game state structs. (20 min)

Initialize game with loaded data. (10 min)

Rust Concepts: serde, serde_json, std::fs::read_to_string().

Camera with Bounds (1 hour)

Intermediate Steps:

Set up Camera2D to follow the player. (20 min)

Calculate map boundaries. (15 min)

Clamp camera target and offset to ensure it stays within map bounds. (25 min)

Raylib/Rust Concepts: Camera2D, clamp() function.
