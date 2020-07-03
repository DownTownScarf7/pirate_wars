fn main() {
    let pirate1 = create_pirate("Jhonny".to_string());
    let mut pirate2 = create_pirate("Jimmy".to_string());

    pirate1.attack(&mut pirate2);
}

struct Range(u32, u32);

struct Coords {
    x: f32,
    y: f32,
    z: f32,
}

impl Coords {
    fn distance(&self, other: &Coords) -> f32 {
        let Coords {x: x1, y: y1, z: z1} = self;
        let Coords {x: x2, y: y2, z: z2} = other;

        f32::sqrt((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0) + (z2 - z1).powf(2.0))
    }
}

struct Weapon {
    name: String,
    damage: u32,
    range: Range,
}

struct Pirate {
    coords: Coords,
    name: String,
    health: u8,
    weapon: Weapon,
}

impl Pirate {
    fn attack(&self, other: &mut Pirate) {
        let distance = self.coords.distance(&other.coords);
        let Range(min_range, max_range) = self.weapon.range;
        if min_range as f32 <= distance && distance <= max_range as f32 {
            println!(
                "{attacker} attacks {target} with {weapon}...",
                attacker=self.name,
                target=other.name,
                weapon=self.weapon.name,
            );
            other.hurt(self.weapon.damage);
        } else {
            println!("{} is too far away from {} to attack", self.name, other.name);
        }
    }

    fn hurt(&mut self, damage: u32) {
        let damage: u8 = if damage > u8::MAX.into() {255_u8} else {damage as u8};
        let damage: u8 = if damage > self.health {self.health} else {damage};

        self.health -= damage;

        println!(
            "{name} has taken {amount} damage, Ouch!",
            name=self.name,
            amount=damage,
        );

        if self.health <= 0 {
            // self.die();
        }
    }

    fn die(self) {
        println!("{} has died, RIP!", self.name);
    }
}

fn create_pirate(name: String) -> Pirate {
    let coords = Coords {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let sword = Weapon {
        name: "Sword".to_string(),
        damage: 20,
        range: Range(0, 10),
    };

    Pirate {
        coords,
        name,
        health: 100,
        weapon: sword,
    }
}
