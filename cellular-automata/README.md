Cellular Automata
=================

Not part of the rust book, just playing around.

There are two parts, one that implements the standard [Elementary Cellular Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton), and one that turns a programming challenge into a Cellular Automaton that simulates object collisions in a one-dimensional space.

Elementary Cellular Automaton
-----------------------------

```
cellular-automata]$ cargo run -- simple
|                                                            X                                                            |
|                                                           X X                                                           |
|                                                          X   X                                                          |
|                                                         X X X X                                                         |
|                                                        X       X                                                        |
|                                                       X X     X X                                                       |
|                                                      X   X   X   X                                                      |
|                                                     X X X X X X X X                                                     |
|                                                    X               X                                                    |
|                                                   X X             X X                                                   |
|                                                  X   X           X   X                                                  |
|                                                 X X X X         X X X X                                                 |
|                                                X       X       X       X                                                |
|                                               X X     X X     X X     X X                                               |
|                                              X   X   X   X   X   X   X   X                                              |
|                                             X X X X X X X X X X X X X X X X                                             |
|                                            X                               X                                            |
|                                           X X                             X X                                           |
|                                          X   X                           X   X                                          |
|                                         X X X X                         X X X X                                         |
|                                        X       X                       X       X                                        |
|                                       X X     X X                     X X     X X                                       |
|                                      X   X   X   X                   X   X   X   X                                      |
|                                     X X X X X X X X                 X X X X X X X X                                     |
|                                    X               X               X               X                                    |
|                                   X X             X X             X X             X X                                   |
|                                  X   X           X   X           X   X           X   X                                  |
|                                 X X X X         X X X X         X X X X         X X X X                                 |
|                                X       X       X       X       X       X       X       X                                |
|                               X X     X X     X X     X X     X X     X X     X X     X X                               |
|                              X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X                              |
|                             X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                             |
```

The command is `simple` instead of `elementary` because I'm lazy at typing.

The rule and various other stuff can be customized, see `--help` for more info.

```
cargo run -- simple --rule 30
|                                                            X                                                            |
|                                                           XXX                                                           |
|                                                          XX  X                                                          |
|                                                         XX XXXX                                                         |
|                                                        XX  X   X                                                        |
|                                                       XX XXXX XXX                                                       |
|                                                      XX  X    X  X                                                      |
|                                                     XX XXXX  XXXXXX                                                     |
|                                                    XX  X   XXX     X                                                    |
|                                                   XX XXXX XX  X   XXX                                                   |
|                                                  XX  X    X XXXX XX  X                                                  |
|                                                 XX XXXX  XX X    X XXXX                                                 |
|                                                XX  X   XXX  XX  XX X   X                                                |
|                                               XX XXXX XX  XXX XXX  XX XXX                                               |
|                                              XX  X    X XXX   X  XXX  X  X                                              |
|                                             XX XXXX  XX X  X XXXXX  XXXXXXX                                             |
|                                            XX  X   XXX  XXXX X    XXX      X                                            |
|                                           XX XXXX XX  XXX    XX  XX  X    XXX                                           |
|                                          XX  X    X XXX  X  XX XXX XXXX  XX  X                                          |
|                                         XX XXXX  XX X  XXXXXX  X   X   XXX XXXX                                         |
|                                        XX  X   XXX  XXXX     XXXX XXX XX   X   X                                        |
|                                       XX XXXX XX  XXX   X   XX    X   X X XXX XXX                                       |
|                                      XX  X    X XXX  X XXX XX X  XXX XX X X   X  X                                      |
|                                     XX XXXX  XX X  XXX X   X  XXXX   X  X XX XXXXXX                                     |
|                                    XX  X   XXX  XXXX   XX XXXXX   X XXXXX X  X     X                                    |
|                                   XX XXXX XX  XXX   X XX  X    X XX X     XXXXX   XXX                                   |
|                                  XX  X    X XXX  X XX X XXXX  XX X  XX   XX    X XX  X                                  |
|                                 XX XXXX  XX X  XXX X  X X   XXX  XXXX X XX X  XX X XXXX                                 |
|                                XX  X   XXX  XXXX   XXXX XX XX  XXX    X X  XXXX  X X   X                                |
|                               XX XXXX XX  XXX   X XX    X  X XXX  X  XX XXXX   XXX XX XXX                               |
|                              XX  X    X XXX  X XX X X  XXXXX X  XXXXXX  X   X XX   X  X  X                              |
|                             XX XXXX  XX X  XXX X  X XXXX     XXXX     XXXX XX X X XXXXXXXXX                             |
```

Collision Simulator
-------------------

The simplest way to run is to generate a random initial state.
```
cargo run -- collider --random'
|<<>>X>X<X_<><X>X_X__>____<X___<<_>__X><>X_<_X>X<<<<<<_>X>X__X_>X<>X___<X<>X<<_<>|
|<__>X_>_X<_<_X_X_X___>__<_X__<<___>_X<__X<__X_><<<<<___X_>__X__X__X__<_<__><_<_X|
|X___X__><_<__X_X_X____><__X_<<_____><___X___X__><<<____X__>_X__X__X_<_<____><__X|
|X___X__<_<___X_X_X_____>__X<<______<____X___X___><_____X___>X__X__X<_<______>__X|
|X___X_<_<____X_X_X______>_X<______<_____X___X____>_____X____>__X__X_<________>_X|
|X___X<_<_____X_X_X_______><______<______X___X_____>____X_____>_X__X<__________>X|
|X___X_<______X_X_X_______<______<_______X___X______>___X______>X__X____________>|
|X___X<_______X_X_X______<______<________X___X_______>__X_______>__X____________X|
|X___X________X_X_X_____<______<_________X___X________>_X________>_X____________X|
|X___X________X_X_X____<______<__________X___X_________>X_________>X____________X|
|X___X________X_X_X___<______<___________X___X__________>__________>____________X|
|X___X________X_X_X__<______<____________X___X___________>__________>___________X|
|X___X________X_X_X_<______<_____________X___X____________>__________>__________X|
|X___X________X_X_X<______<______________X___X_____________>__________>_________X|
|X___X________X_X_<______<_______________X___X______________>__________>________X|
|X___X________X_X<______<________________X___X_______________>__________>_______X|
|X___X________X_<______<_________________X___X________________>__________>______X|
|X___X________X<______<__________________X___X_________________>__________>_____X|
|X___X________<______<___________________X___X__________________>__________>____X|
|X___X_______<______<____________________X___X___________________>__________>___X|
|X___X______<______<_____________________X___X____________________>__________>__X|
|X___X_____<______<______________________X___X_____________________>__________>_X|
|X___X____<______<_______________________X___X______________________>__________>X|
|X___X___<______<________________________X___X_______________________>__________X|
|X___X__<______<_________________________X___X________________________>_________X|
|X___X_<______<__________________________X___X_________________________>________X|
|X___X<______<___________________________X___X__________________________>_______X|
|X___X______<____________________________X___X___________________________>______X|
|X___X_____<_____________________________X___X____________________________>_____X|
|X___X____<______________________________X___X_____________________________>____X|
|X___X___<_______________________________X___X______________________________>___X|
|X___X__<________________________________X___X_______________________________>__X|
|X___X_<_________________________________X___X________________________________>_X|
|X___X<__________________________________X___X_________________________________>X|
|X___X___________________________________X___X__________________________________>|
|X___X___________________________________X___X__________________________________X|
Initial state:
' -3 -12 +23 +92 98 +68 39 -6 12 _ -75 +35 -93 18 +48 61 _ 13 _ _ +71 _ _ _ _ -21 70 _ _ _ -8 -82 _ +1 _ _ 32 +12 -81 +28 49 _ -15 _ 66 +99 8 -31 -43 -48 -77 -5 -38 _ +22 86 +82 14 _ _ 37 _ +12 63 -41 +31 49 _ _ _ -38 19 -25 +86 2 -85 -47 _ -27 +8 '
```

### Original Challenge Rules

> ### Simulate 1-D Collisions and Scale to Huge Inputs
> 
> You are given an array of integers representing objects moving along a one-dimensional track.
> - The absolute value of each integer represents the size (or mass) of the object.
> - The sign represents the direction of motion:
> - Positive → moving right
> - Negative → moving left
> - All objects move at the same speed.
> 
> Collision Rules
> - Objects move simultaneously.
> - A collision occurs only when a right-moving object meets a left-moving object.
> - When two objects collide:
> - The object with the smaller absolute value is destroyed.
> - If both have the same absolute value, both are destroyed.
> - Objects moving in the same direction never collide.
> - After a collision, the surviving object (if any) continues moving in the same direction and may collide again.
> 
> Goal
> - Return the state of the objects after all possible collisions have occurred.
> - The output should preserve the original relative order of surviving objects.
> 
> Example
> Input: `[5, 10, -5]`
> Output: `[5, 10]`
> 
> Example
> Input: `[8, -8]`
> Output: `[]`
> 
> Example
> Input: `[10, 2, -5]`
> Output: `[10]`

Inspiration was taken from the challenge, but I've deviated in a couple of ways:
- Included stationary objects, because what to do with `0` wasn't specified.
- The output is of the state after each step, so it'll look more interesting.
- Added bounds to the size of the 1D space, in the interests of more interesting visualization.
- Restricted the values of each object to `0..=100`, mostly to keep the debug output from getting out of hand.

### Explanation of Options

The initial state can also be specified (note the leading space, `clap` has trouble understanding that it's not an argument if there's a leading `-`).
Output omitted because it's the same as the previous run, with the exception that the initial state is not printed after the run.
```
cargo run -- collider --state ' -3 -12 +23 +92 98 +68 39 -6 12 _ -75 +35 -93 18 +48 61 _ 13 _ _ +71 _ _ _ _ -21 70 _ _ _ -8 -82 _ +1 _ _ 32 +12 -81 +28 49 _ -15 _ 66 +99 8 -31 -43 -48 -77 -5 -38 _ +22 86 +82 14 _ _ 37 _ +12 63 -41 +31 49 _ _ _ -38 19 -25 +86 2 -85 -47 _ -27 +8 '
```

Additionally, flags allow enabling optional rules:

- `--bounce` makes object bounce off the side walls, instead of becoming stationary
- `--damage` makes object take damage, instead of the default "winner takes no damage"

These can change the output considerably:
```
cargo run -- collider --bounce --damage --state ' -3 -12 +23 +92 98 +68 39 -6 12 _ -75 +35 -93 18 +48 61 _ 13 _ _ +71 _ _ _ _ -21 70 _ _ _ -8 -82 _ +1 _ _ 32 +12 -81 +28 49 _ -15 _ 66 +99 8 -31 -43 -48 -77 -5 -38 _ +22 86 +82 14 _ _ 37 _ +12 63 -41 +31 49 _ _ _ -38 19 -25 +86 2 -85 -47 _ -27 +8 '
|<<>>X>X<X_<><X>X_X__>____<X___<<_>__X><>X_<_X>X<<<<<<_>X>X__X_>X<>X___<X<>X<<_<>|
|<__>X_>_X<_<_X_X_X___>__<_X__<<___>_X<__X<__X_><<<<<___X_>__X__X__X__<_<__<<_<_<|
|>___>__><_<__X_X_X____><__X_<<_____><___X___X__<<<<____X__>_X__X__X_<_<__<<_<_<_|
|_>___>_<_<___X_X_X_____>__X<<______<____X___X_<<<<_____X___>X__X__X<_<__<<_<_<__|
|__>___<_<____X_X_X______>_X<______<_____X___X<<<<______X____>__X__<_<__<<_<_<___|
|___>_<_<_____X_X_X_______><______<______X___X<<<_______X_____>_X_<_<__<<_<_<____|
|____<_<______X_X_X________>_____<_______X___<<<________X______>X<_<__<<_<_<_____|
|___<_<_______X_X_X_________>___<________X__<<<_________X_______>_<__<<_<_<______|
|__<_<________X_X_X__________>_<_________X_<<<__________X________<__<<_<_<_______|
|_<_<_________X_X_X___________<__________X<<<___________X_______<__<<_<_<________|
|<_<__________X_X_X__________<___________<<<____________X______<__<<_<_<_________|
|><___________X_X_X_________<___________<<<_____________X_____<__<<_<_<__________|
|<____________X_X_X________<___________<<<______________X____<__<<_<_<___________|
|>____________X_X_X_______<___________<<<_______________X___<__<<_<_<____________|
|_>___________X_X_X______<___________<<<________________X__<__<<_<_<_____________|
|__>__________X_X_X_____<___________<<<_________________X_<__<<_<_<______________|
|___>_________X_X_X____<___________<<<__________________X<__<<_<_<_______________|
|____>________X_X_X___<___________<<<___________________X__<<_<_<________________|
|_____>_______X_X_X__<___________<<<____________________X_<<_<_<_________________|
|______>______X_X_X_<___________<<<_____________________X<<_<_<__________________|
|_______>_____X_X_X<___________<<<______________________X<_<_<___________________|
|________>____X_X_X___________<<<_______________________X_<_<____________________|
|_________>___X_X_X__________<<<________________________X<_<_____________________|
|__________>__X_X_X_________<<<_________________________<_<______________________|
|___________>_X_X_X________<<<_________________________<_<_______________________|
|____________>X_X_X_______<<<_________________________<_<________________________|
|_____________>_X_X______<<<_________________________<_<_________________________|
|______________>X_X_____<<<_________________________<_<__________________________|
|_______________>_X____<<<_________________________<_<___________________________|
|________________>X___<<<_________________________<_<____________________________|
|_________________>__<<<_________________________<_<_____________________________|
|__________________><<<_________________________<_<______________________________|
|__________________<<<_________________________<_<_______________________________|
```

