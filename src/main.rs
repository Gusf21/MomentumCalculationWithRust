use std::io;

fn main() {
    let m1: f64 = get_f64("mass", "A");
    let m2: f64 = get_f64("mass", "B");
    let mut v1: f64 = -1f64 * get_f64("velocity", "A");
    let mut v2: f64 = 0f64;
    println!("Mass of object A is {}kg\nMass of object B is {}kg\nVelocity of object A is {}m s\nVelocity of object B is {}m s", m1, m2, v1, v2);
    println!("Starting Simulation");

    let mut collision_status: (bool, bool) = collision_check(v1, v2);
    let mut collisions: i32 = 0i32;

    while collision_status.0 == true {
        if collision_status.1 == true {
            let results = get_final_velocities(m1, m2, v1, v2);
            v1 = results.0;
            v2 = results.1;
            collisions += 1;
        } else {
            v2 = v2 * -1f64;
            collisions += 1;
        }
        //println!("Vel of A = {}\nVel of B = {}", v1, v2);
        collision_status = collision_check(v1, v2);
    }
    println!("Simulation Complete!\nTotal of {} collisions", collisions);
}

fn collision_check(v1: f64, v2: f64) -> (bool, bool) {
    let mut results: (bool, bool) = (false, false);
    if v2 < 0f64 {
        results = (true, false);
        //println!("Wall Collision");
    } else if v1 < 0f64 && v2 >= 0f64 {
        results = (true, true);
        //println!("Object Collision, -ve velocity");
    } else if (v1 > 0f64 && v2 > 0f64) && v2 > v1 {
        results = (true, true);
        //println!("Object Collision, +ve velocity");
    }
    return results;
}

fn get_final_velocities(m1: f64, m2: f64, u1: f64, u2: f64) -> (f64, f64) {
    let v1: f64 = (((m1 - m2) / (m1 + m2)) * u1)+(((2f64 * m2) / (m1 + m2)) * u2);
    let v2: f64 = (((2f64 * m1) / (m1 + m2)) * u1)+(((m2 - m1) / (m1 + m2)) * u2);
    return (v1, v2);
}

fn get_f64(property: &str, object: &str) -> f64 {
    let mut valid: bool = false;
    let mut val: f64 = 0f64;
    println!("Please enter the {} of object {}",property, object);
    while !valid {
        let mut string_val: String = String::new();
        match io::stdin().read_line(&mut string_val) {
            Ok(_n) => {}
            Err(error) => {
                println!("error: {}", error);
            }
        }
        let test = string_val.trim().parse::<f64>();
        match test {
            Ok(result) => {
                val = result;
                valid = true
            }
            Err(_error) => {
                println!("Invalid Input, please try again");
            }
        }
    }
    return val;
}
