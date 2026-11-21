# Projectile behavior fixture

`projectile-behavior.json` contains 2,496 combat cases captured from the retired,
instruction-preserving Rust implementation during the native source migration.
The replacement was compared with that implementation for complete entity state,
vector contents, errors and rendered pixels before this fixture was frozen.
No executable reference code or source-generation step is needed to run it.

`src/game/projectile_tests.rs` replays each case against the maintained Rust.
Cases cover 12 projectile kinds, nine enemy/boss kinds, ownership, returning
projectiles, shields, sword reflection, solid tiles and room boundaries.

Each row is `[projectile_kind, enemy_kind, mode, [x, y], result, snapshot]`.
Mode bits select enemy ownership, returning, shield use and sword use; mode 15
also marks the room tiles solid. Results are the returned integer or exception
class. The snapshot contains:

- Player: health, hurt timer, projectile cooldown, boomerang status, world flags.
- Remaining projectiles: kind, X/Y, X/Y velocity, damage, ownership, returning, animation.
- Enemies: kind, health, hurt timer, behavior state, dying, animation, projectile status, hit count, state timer.
- A rolling checksum of the rendered frame.

The two entity lists are grouped together between player state and frame checksum.
The separate original-JAR oracle still checks opening gameplay and save bytes.
