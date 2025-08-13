# Solar System Simulator

A 3D solar system built with Rust and Bevy. The planets actually orbit using gravity physics.

Note: Still working on this. The basic orbital mechanics are functional but I'm still tweaking the physics to make the orbits more stable and realistic. Some planets might still drift or act weird occasionally.

## What it does

- Sun in the center with three planets orbiting around it
- Real gravity calculations make the orbits work
- Fly around with WASD and mouse to watch from different angles
- Physics run smooth regardless of framerate

## Controls

- WASD to move camera
- Right click + mouse to look around  
- Space/Shift to go up/down

## Running it

Need Rust installed, then:

```bash
git clone https://github.com/yourusername/solar-system-simulator.git
cd solar-system-simulator
cargo run --release
```

## How it works

Built using Bevy's Entity Component System. Each planet has:
- Mass (for gravity calculations)
- Velocity (current movement)
- Position (where it is)

The gravity system runs every frame and calculates forces between all objects using F = G(m1*m2)/r². Then the movement system updates positions based on velocities.

## Code structure

Everything's in `main.rs` for simplicity:
- Components define what data objects have
- Systems define what happens each frame
- Setup function creates the initial scene

The tricky part was getting stable orbits. Too fast and planets fly away, too slow and they spiral into the sun.

## Why I made this

Wanted to learn Rust better and understand how game engines work. Physics simulations are fun and this shows off:
- Rust's ownership system keeping things memory safe
- ECS architecture for game logic
- Real-time physics integration
- 3D graphics programming

## Dependencies

Just Bevy engine. Check Cargo.toml for version.

## License

MIT
