# Burtle

A minimal rewrite of the python turtle in rust using bevy as the backend

## Exemple

```rust
use burtle::Burtle;
use burtle::Color;

fn main() {
    let mut turtle = Burtle::new();
    turtle.pen_down();
    turtle.goto(500., 1.);
    turtle.set_pen_color(Color::RED);
    for _ in 0..360 {
        // Move forward three steps
        turtle.forward(3.0);
        // Rotate to the right (clockwise) by 1 degree
        turtle.right(1.0);
    }
    turtle.run(1000., 1000.)
}
```
