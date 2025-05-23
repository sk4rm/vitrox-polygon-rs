//! Sample input:
//! ```
//! 7
//! 2 6
//! 4 8
//! 8 4
//! 7 2
//! 6 0
//! 2 0
//! 0 4
//! ```
//!
//! Sample output:
//! ```
//! 4 8
//! 8 4
//! 6 0
//! 2 0
//! 0 4
//! 2
//! ```

struct Vertex(i32, i32);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _vertex_count: i32 = 0;
    let mut vertices: Vec<Vertex> = Vec::new();

    // region Input

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;

    _vertex_count = buffer.trim().parse::<i32>()?;

    for _ in 0.._vertex_count {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        let tokens = buffer
            .trim()
            .split(" ")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        vertices.push(Vertex(tokens[0], tokens[1]));
    }

    // endregion

    let mut necessary_vertices: Vec<&Vertex> = Vec::new();
    let mut redundant_vertices: Vec<&Vertex> = Vec::new();

    // region Solve

    let mut prev: &Vertex = vertices.last().unwrap();
    let mut prev_gradient = f32::NAN;
    let mut first = true;
    for vertex in vertices.iter() {
        let gradient: f32 = ((vertex.1 - prev.1) as f32) / ((vertex.0 - prev.0) as f32);

        if first {
            first = false;
            prev_gradient = gradient;
            continue;
        }

        if gradient == prev_gradient {
            // Previous vertex was collinear.
            redundant_vertices.push(prev);
        } else {
            necessary_vertices.push(prev);
        }

        prev = vertex;
        prev_gradient = gradient;
    }

    // Edge case: check collinearity of { v[n], v[0], v[1] }
    let vertex = vertices.first().unwrap();
    let gradient = ((vertex.1 - prev.1) as f32) / ((vertex.0 - prev.0) as f32);
    if gradient == prev_gradient {
        redundant_vertices.push(prev);
    } else {
        necessary_vertices.push(prev);
    }

    // endregion

    println!(
        "Redundant vertices: {}\nThe rest:",
        redundant_vertices.len()
    );
    for vertex in necessary_vertices {
        println!("{} {}", vertex.0, vertex.1);
    }

    Ok(())
}
