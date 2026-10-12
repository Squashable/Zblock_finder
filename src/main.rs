use serde::Deserialize;
use std::collections::HashMap;

// --- STRUCTS MATCHING THE JSON SCHEMA ---

#[derive(Deserialize, Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct Classroom {
    code: String,
    floor: i32,
    center: Coordinate,
    bounds: Bounds,
}

#[derive(Deserialize, Debug)]
struct CampusData {
    building: String,
    campus: String,
    rooms: Vec<Classroom>,
}

// Represents the user's current live location
#[derive(Debug)]
struct UserLocation {
    floor: i32,
    x: i32,
    y: i32,
}

fn main() {
    // Simulated JSON data (in a real app, you would load this from "z_block_rooms.json")
    let json_data = r#"
    {
      "building": "Z Block",
      "campus": "UWE Frenchay",
      "rooms": [
        {
          "code": "Z0010",
          "floor": 0,
          "center": { "x": 150, "y": 4 },
          "bounds": { "min_x": 145, "max_x": 155, "min_y": 2, "max_y": 6 }
        },
        {
          "code": "Z1010",
          "floor": 1,
          "center": { "x": 50, "y": 30 },
          "bounds": { "min_x": 45, "max_x": 55, "min_y": 25, "max_y": 35 }
        }
      ]
    }
    "#;

    // --- STEP 1: PARSE JSON INTO RUST STRUCTS & MAP ---
    let campus: CampusData = serde_json::from_str(json_data).expect("Failed to parse JSON");
    
    // Convert the vector of rooms into a HashMap for fast code-based lookups (e.g., "Z1010")
    let mut room_map: HashMap<String, Classroom> = HashMap::new();
    for room in campus.rooms {
        room_map.insert(room.code.clone(), room);
    }

    println!("=== Loaded {} for {} ===", campus.building, campus.campus);

    // --- STEP 2: USER SELECTS DESTINATION ---
    let target_room_code = "Z1010";
    let target = match room_map.get(target_room_code) {
        Some(room) => room,
        None => {
            println!("Room {} not found!", target_room_code);
            return;
        }
    };

    // --- STEP 3: INITIALIZE USER LOCATION ---
    // Let's say the user starts on the Ground Floor (Floor 0), far away
    let mut user = UserLocation {
        floor: 0,
        x: 10,
        y: 10,
    };

    println!("User wants to navigate to: {}\n", target.code);

    // --- STEP 4A: REAL-TIME GUIDANCE - FLOOR MATCHING (Stairs / Lifts) ---
    while user.floor != target.floor {
        if user.floor < target.floor {
            println!(" You are on floor {}. Target is on floor {}.", user.floor, target.floor);
            println!("-> Action: Go UP one floor (take stairs or lift).");
            user.floor += 1; // Simulated user movement (e.g., triggered by barometer sensor)
        } else {
            println!(" You are on floor {}. Target is on floor {}.", user.floor, target.floor);
            println!("-> Action: Go DOWN one floor (take stairs or lift).");
            user.floor -= 1;
        }
    }

    println!("\n You are now on the correct floor (Floor {}). Starting proximity guidance...\n", user.floor);

    // --- STEP 4B: REAL-TIME GUIDANCE - WARMER / COLDER GAME ---
    // Using center coordinates to calculate distance changes
    let mut last_distance = (target.center.x - user.x).pow(2) + (target.center.y - user.y).pow(2);

    // Simulated user steps walking down the corridor on Floor 1
    let simulated_walk = vec![(20, 15), (35, 22), (48, 28), (50, 30)];

    for (new_x, new_y) in simulated_walk {
        user.x = new_x;
        user.y = new_y;

        // Check if user's coordinates are INSIDE the room's boundary box
        let inside_bounds = user.x >= target.bounds.min_x && user.x <= target.bounds.max_x &&
                            user.y >= target.bounds.min_y && user.y <= target.bounds.max_y;

        if inside_bounds {
            println!("🎉 ARRIVED! You have entered {}!", target.code);
            break;
        }

        // Calculate distance to room center
        let current_distance = (target.center.x - user.x).pow(2) + (target.center.y - user.y).pow(2);
        
        // Give left/right directional hint based on X-axis
        let direction = if target.center.x > user.x { "right" } else { "left" };

        if current_distance < last_distance {
            println!(" WARMER! Turn {} and keep walking.", direction);
        } else {
            println!(" COLDER! You're walking away. Try turning around.");
        }

        last_distance = current_distance;
    }
}