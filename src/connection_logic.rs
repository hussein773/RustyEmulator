use std::collections::{HashMap, HashSet, VecDeque};
use ggez::graphics::Rect;
use crate::structure::{Hitbox, HitboxType, UnionFind};

// Insert hitbox in the grid 
fn insert_hitbox_in_grid(
    grid: &mut HashMap<(i32, i32), Vec<usize>>,
    hitbox: &Hitbox,
    index: usize,
    cell_size: f32
) {
    let min_x = (hitbox.rect.x / cell_size) as i32;
    let max_x = ((hitbox.rect.x + hitbox.rect.w) / cell_size) as i32;
    let min_y = (hitbox.rect.y / cell_size) as i32;
    let max_y = ((hitbox.rect.y + hitbox.rect.h) / cell_size) as i32;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            grid.entry((x, y)).or_insert_with(Vec::new).push(index);
        }
    }
}

// Detect collisions between hitboxes
pub fn detect_collisions(hitboxes: &[Hitbox], cell_size: f32) -> Vec<(usize, usize)> {
    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    let mut unique_collisions: HashSet<(usize, usize)> = HashSet::new();

    // Insert hitboxes into the grid
    for (i, hitbox) in hitboxes.iter().enumerate() {
        insert_hitbox_in_grid(&mut grid, hitbox, i, cell_size);
    }

    // Check collisions in each grid cell
    for hitbox_indices in grid.values() {
        for (i, &a) in hitbox_indices.iter().enumerate() {
            for &b in &hitbox_indices[i + 1..] {
                if hitboxes[a].rect.overlaps(&hitboxes[b].rect) {
                    let collision = if a < b { (a, b) } else { (b, a) };
                    unique_collisions.insert(collision);
                }
            }
        }
    }

    unique_collisions.into_iter().collect()
}

pub fn group_connected_pins(hitboxes: &[Hitbox], cell_size: f32) -> Vec<HashSet<usize>> {
    let collisions = detect_collisions(hitboxes, cell_size); // Get colliding pairs
    let mut uf = UnionFind::new(hitboxes.len());

    // Step 1: Union ALL hitboxes (wires and pins) that collide
    for &(a, b) in &collisions {
        uf.union(a, b);
    }

    // Collect groups by root parent
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 0..hitboxes.len() {
        let root = uf.find(i);
        groups.entry(root).or_insert_with(HashSet::new).insert(i);
    }

    // Filter groups to keep only pin hitboxes
    let mut pin_groups = Vec::new();

    for group in groups.values() {
        let pin_group: HashSet<usize> = group.iter()
            .filter(|&&i| matches!(hitboxes[i].r#type, HitboxType::Pin(..)))
            .cloned()
            .collect();

        if !pin_group.is_empty() {
            pin_groups.push(pin_group);
        }
    }

    // Print detected pin groups
    /*println!("\n==== Connected Pin Groups ====");
    for (i, group) in pin_groups.iter().enumerate() {
        let pins: Vec<String> = group.iter().map(|&index| {
            // Destructure the Pin type and extract the 3 usize values
            if let HitboxType::Pin(a, b, c) = hitboxes[index].r#type {
                format!("Pin {} (CID: {}, PID: {}, IOC: {})", index, a, b, c)
            } else {
                format!("Pin {}", index)
            }
        }).collect();
        println!("Group {}: {}", i + 1, pins.join(", "));
    }
    println!("==============================\n");*/

    pin_groups
}


