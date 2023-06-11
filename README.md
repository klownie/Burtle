# Burtle
A minimal rewrite of the python turtle in rust using bevy as the backend

## Exemple
```rust
use burtle::*;
use std::collections::HashMap;

fn derive_iter<'a>(þ: &'a mut String, n: usize, p: &[&'static str; 2]) {
    let mut rules: HashMap<char, String> = HashMap::new();

    for rule in p {
        let parts: Vec<&str> = rule.split("->").map(|s| s.trim()).collect();
        if let [lhs, rhs] = parts.as_slice() {
            let key = lhs.chars().next().unwrap();
            rules.insert(key, rhs.to_string());
        }
    }

    for _ in 0..n {
        let mut deriv_þ = String::new();
        for character in þ.chars() {
            if let Some(rule) = rules.get(&character) {
                deriv_þ.push_str(rule);
            } else {
                deriv_þ.push(character);
            }
        }
        *þ = deriv_þ;
    }
}

fn dessiner(burtle: &mut Burtle, chaine: &String, angle: f32, length: f32) {
    for character in chaine.chars() {
        match character {
            '+' => burtle.right(angle),

            '-' => burtle.left(angle),

            '[' => {}

            ']' => {}

            _ => burtle.forward(length),
        }
    }
}

fn main() {
    let mut þ = String::from("X");
    let p = ["F -> X+F+X", "X -> F-X-F"];
    derive_iter(&mut þ, 8, &p);

    let mut burtle = Burtle::new();
    burtle.pen_down();
    dessiner(&mut burtle, &þ, 60., 3.);
    burtle.setup(1000., 1000.)
}
``` 
