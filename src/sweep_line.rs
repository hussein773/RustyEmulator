use crate::structure::*;
use std::collections::BTreeSet;
use ordered_float::OrderedFloat;

pub fn find_intersections(segments: &[WireSegment]) {
    let mut events: Vec<Event> = Vec::new();
    let mut verticals: Vec<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)> = Vec::new(); 

    // Convert wire segments into events
    for seg in segments {
        if seg.start.y == seg.end.y {
            // Horizontal wire
            let (x1, x2) = if seg.start.x < seg.end.x {
                (seg.start.x, seg.end.x)
            } else {
                (seg.end.x, seg.start.x)
            };
            let y = seg.start.y;
            events.push(Event::HorizontalStart(y, x1, x2));
            events.push(Event::HorizontalEnd(y, x1, x2));
        } else {
            // Vertical wire
            let (y1, y2) = if seg.start.y < seg.end.y {
                (seg.start.y, seg.end.y)
            } else {
                (seg.end.y, seg.start.y)
            };
            let x = seg.start.x;
            verticals.push((OrderedFloat(x), OrderedFloat(y1), OrderedFloat(y2))); // Store for V-V check
            events.push(Event::Vertical(x, y1, y2));
        }
    }

    // Sort events first x then y
    events.sort_by(|a, b| match (a, b) {
        (Event::Vertical(x1, _, _), Event::Vertical(x2, _, _)) => x1.total_cmp(x2),
        (Event::Vertical(x, _, _), _) => x.total_cmp(&match b {
            Event::HorizontalStart(y, _, _) => *y,
            Event::HorizontalEnd(y, _, _) => *y,
            _ => unreachable!(),
        }),
        (_, Event::Vertical(x, _, _)) => match a {
            Event::HorizontalStart(y, _, _) => y.total_cmp(x),
            Event::HorizontalEnd(y, _, _) => y.total_cmp(x),
            _ => unreachable!(),
        },
        (Event::HorizontalStart(y1, _, _), Event::HorizontalStart(y2, _, _)) => y1.total_cmp(y2),
        (Event::HorizontalStart(y1, _, _), Event::HorizontalEnd(y2, _, _)) => y1.total_cmp(y2),
        (Event::HorizontalEnd(y1, _, _), Event::HorizontalEnd(y2, _, _)) => y1.total_cmp(y2),
        (Event::HorizontalEnd(y1, _, _), Event::HorizontalStart(y2, _, _)) => y1.total_cmp(y2),
    });

    let mut active_horizontal: BTreeSet<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>)> = BTreeSet::new(); // (y, x1, x2)

    // Sweep Line Processing
    for event in events {
        match event {
            // Handle Horizontal Start
            Event::HorizontalStart(y, x1, x2) => {
                println!("Adding Horizontal: y={}, x1={}, x2={}", y, x1, x2);
                active_horizontal.insert((OrderedFloat(y), OrderedFloat(x1), OrderedFloat(x2)));

                // Check for intersections with other horizontal segments
                for &(y2, hx1, hx2) in active_horizontal.range(
                    (OrderedFloat(y), OrderedFloat(f32::MIN), OrderedFloat(f32::MIN))..=
                    (OrderedFloat(y), OrderedFloat(f32::MAX), OrderedFloat(f32::MAX))
                ) {
                    // Skip self-intersection (avoid checking the same wire against itself)
                    if (hx1.into_inner(), hx2.into_inner()) == (x1, x2) {
                        continue;
                    }
                    if x1 <= hx2.into_inner() && hx1.into_inner() <= x2 {
                        println!("Horizontal-Horizontal Intersection at y={} between x=[{},{}] and x=[{},{}]", y, x1, x2, hx1, hx2);
                    }
                }
            }
            // Handle Horizontal End
            Event::HorizontalEnd(y, x1, x2) => {
                // Defer removal until after vertical checks
                println!("Pending removal of Horizontal: y={}, x1={}, x2={}", y, x1, x2);
            }
            // Handle Vertical Event
            Event::Vertical(x, y1, y2) => {
                println!("Processing Vertical at x={}, range y=[{}, {}]", x, y1, y2);
                // check for intersections with active horizontal wires
                for &(y, hx1, hx2) in active_horizontal.range(
                    (OrderedFloat(y1), OrderedFloat(f32::MIN), OrderedFloat(f32::MIN))..=
                    (OrderedFloat(y2), OrderedFloat(f32::MAX), OrderedFloat(f32::MAX))
                ) {
                    if hx1.into_inner() <= x && x <= hx2.into_inner() {
                        println!("Intersection Found: Horizontal at y={} and Vertical at x={}", y, x);
                    }
                }
            }
        }
        
        // Now perform the remval after vertical processing
        if let Event::HorizontalEnd(y, x1, x2) = event {
            println!("Removing Horizontal: y={}, x1={}, x2={}", y, x1, x2);
            active_horizontal.remove(&(OrderedFloat(y), OrderedFloat(x1), OrderedFloat(x2)));
            println!("Active Set After Removal: {:?}", active_horizontal);
        }
    }

    // Vertical-Vertical Intersection Check
    for i in 0..verticals.len() {
        for j in i + 1..verticals.len() {
            let (x1, y1_min, y1_max) = verticals[i];
            let (x2, y2_min, y2_max) = verticals[j];

            if x1 == x2 {
                let y_min = y1_min.max(y2_min);
                let y_max = y1_max.min(y2_max);
                if y_min < y_max {
                    println!("Vertical-Vertical Intersection at x={} between y=[{}, {}] and y=[{}, {}]", x1, y1_min, y1_max, y2_min, y2_max);
                }
            }
        }
    }
}
