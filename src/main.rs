use binary_heap_plus::*;
use core::iter::Iterator;
use fxhash::FxHashSet;

type Pos = (i32, i32, i32);
type Dims = (i32, i32, i32);

// a block is its dimensions
type Block = Dims;

// a block def is all the ways it can look when rotated (pre-defined for simplicity)
type BlockDef = Vec<Dims>;

#[derive(Debug, Clone, Copy)]
struct Placement {
    block: Block,
    pos: Pos,
}

#[derive(Debug, Clone)]
struct State {
    placements: Vec<Placement>,
    boundary: FxHashSet<(i32, i32, i32)>,
    free: FxHashSet<(i32, i32, i32)>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            placements: vec![],
            boundary: FxHashSet::from_iter([(0, 0, 0)]),
            free: (0..5)
                .flat_map(move |x| {
                    //
                    (0..5).flat_map(move |y| {
                        //
                        (0..5).map(move |z| (x, y, z))
                    })
                })
                .collect(),
        }
    }
}

fn get_big_block_def() -> BlockDef {
    vec![
        // 6 x [2x2x3]
        (3, 2, 2),
        (2, 3, 2),
        (2, 2, 3),
    ]
}

fn get_flat_block_def() -> BlockDef {
    vec![
        // 6 x [2x4x1]
        (1, 2, 4),
        (1, 4, 2),
        (2, 1, 4),
        (2, 4, 1),
        (4, 1, 2),
        (4, 2, 1),
    ]
}

fn get_pixel_block_def() -> BlockDef {
    vec![
        // 5 x [1x1x1]
        (1, 1, 1),
    ]
}

fn get_block_def(no: usize) -> Option<BlockDef> {
    if no < 6 {
        Some(get_big_block_def())
    } else if no < 6 + 6 {
        Some(get_flat_block_def())
    } else if no < 6 + 6 + 5 {
        Some(get_pixel_block_def())
    } else {
        None
    }
}

impl State {
    fn space_left(&self) -> usize {
        self.free.len()
    }

    fn hash(&self) -> u128 {
        let mut grid = [[[false; 5]; 5]; 5];

        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    grid[x][y][z] = self.free.contains(&(x as i32, y as i32, z as i32));
                }
            }
        }

        grid_hash(&grid)
    }

    fn next(&self) -> Vec<State> {
        let Some(next_block_def) = get_block_def(self.placements.len()) else {
            panic!("Could not find next block from: {:?}", self);
        };

        let mut found = vec![];

        for (sx, sy, sz) in next_block_def {
            // find spots where it fits
            'find_spot: for &(x, y, z) in &self.free {
                for dx in 0..sx {
                    for dy in 0..sy {
                        for dz in 0..sz {
                            if !self.free.contains(&((x + dx, y + dy, z + dz))) {
                                continue 'find_spot;
                            }
                        }
                    }
                }

                // println!("CHECK that placement touches boundary {:?}", self.boundary);
                // println!(
                //     "{:?}",
                //     Placement {
                //         block: (sx, sy, sz),
                //         pos: (x, y, z),
                //     }
                // );

                // it fits at (x,y,z)! -> remove
                let mut s = self.clone();
                s.placements.push(Placement {
                    block: (sx, sy, sz),
                    pos: (x, y, z),
                });
                for dx in 0..sx {
                    for dy in 0..sy {
                        for dz in 0..sz {
                            s.free.remove(&(x + dx, y + dy, z + dz));
                            s.boundary.insert((x + dx - 1, y + dy, z + dz));
                            s.boundary.insert((x + dx + 1, y + dy, z + dz));
                            s.boundary.insert((x + dx, y + dy - 1, z + dz));
                            s.boundary.insert((x + dx, y + dy + 1, z + dz));
                            s.boundary.insert((x + dx, y + dy, z + dz - 1));
                            s.boundary.insert((x + dx, y + dy, z + dz + 1));
                        }
                    }
                }

                found.push(s);
            }
        }

        found
    }
}

fn main() {
    test_hash_function();

    let mut queue = BinaryHeap::new_by_key(|s: &State| {
        // 20 - s.placements.len()
        128 - s.space_left()
    });

    queue.push(State::default());

    let mut i = 0;

    let mut seen = FxHashSet::default();
    let mut num_collisions = 0;

    let mut min_space_left: State = State::default();

    while let Some(state) = queue.pop() {
        i += 1;

        seen.insert(state.hash());

        // if i == 4 {
        //     panic!("done")
        // }

        if state.space_left() < min_space_left.space_left() {
            min_space_left = state.clone();
        }

        if i % 10_000 == 0 {
            println!(
                "working.. ({} in queue) {} placements, {} space left ({num_collisions} collisions)",
                queue.len(),
                state.placements.len(),
                state.space_left()
            );
        }

        if state.space_left() == 0 {
            println!("Found solution!");
            println!("{:?}", state);
            return;
        }

        for n in state.next() {
            if !seen.contains(&n.hash()) {
                queue.push(n);
            } else {
                num_collisions += 1;
            }
        }
    }

    /*
    SOLUTION!

    State {
        placements: [
            Placement { block: (3, 2, 2), pos: (0, 3, 1) },
            Placement { block: (3, 2, 2), pos: (2, 0, 2) },
            Placement { block: (2, 2, 3), pos: (0, 1, 0) },
            Placement { block: (2, 3, 2), pos: (1, 2, 3) },
            Placement { block: (2, 3, 2), pos: (2, 0, 0) },
            Placement { block: (2, 2, 3), pos: (3, 2, 2) },
            Placement { block: (1, 4, 2), pos: (4, 0, 0) },
            Placement { block: (1, 4, 2), pos: (0, 1, 3) },
            Placement { block: (2, 1, 4), pos: (0, 0, 0) },
            Placement { block: (2, 1, 4), pos: (3, 4, 1) },
            Placement { block: (4, 2, 1), pos: (1, 0, 4) },
            Placement { block: (4, 2, 1), pos: (0, 3, 0) },
            Placement { block: (1, 1, 1), pos: (1, 1, 3) },
            Placement { block: (1, 1, 1), pos: (0, 0, 4) },
            Placement { block: (1, 1, 1), pos: (3, 3, 1) },
            Placement { block: (1, 1, 1), pos: (4, 4, 0) },
            Placement { block: (1, 1, 1), pos: (2, 2, 2) }
        ]
    }
    */
}

pub fn grid_hash(grid: &[[[bool; 5]; 5]; 5]) -> u128 {
    // Generate all possible orientations of the grid
    // let mut all_orientations = Vec::new();
    let mut unique_hashes = FxHashSet::default();

    // Create a copy of the grid we can work with
    let mut temp_grid = [[[false; 5]; 5]; 5];
    copy_grid(grid, &mut temp_grid);

    // Generate all 48 possible orientations (24 rotations × 2 reflections)
    // We'll implement this using a series of transformations

    // Handle the 6 possible facing directions (permutations of axes)
    let permutations = [
        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
    ];

    for &(i, j, k) in &permutations {
        // Create a permutation of the grid
        let mut permuted = [[[false; 5]; 5]; 5];

        for x in 0..5 {
            for y in 0..5 {
                for z in 0..5 {
                    let new_indices = get_permuted_indices(x, y, z, i, j, k);
                    permuted[new_indices.0][new_indices.1][new_indices.2] = grid[x][y][z];
                }
            }
        }

        // For each permutation, add all rotations and reflections
        for _ in 0..4 {
            // Rotate around first axis
            for _ in 0..4 {
                // Rotate around second axis
                // Original orientation
                let hash = calculate_hash(&permuted);
                unique_hashes.insert(hash);

                // Flip across each axis
                for axis in 0..3 {
                    let mut flipped = [[[false; 5]; 5]; 5];
                    copy_grid(&permuted, &mut flipped);
                    flip_grid(&mut flipped, axis);

                    let flipped_hash = calculate_hash(&flipped);
                    unique_hashes.insert(flipped_hash);
                }

                // Rotate around second axis
                rotate_grid(&mut permuted, 1);
            }
            // Rotate around first axis
            rotate_grid(&mut permuted, 0);
        }
    }

    // The canonical hash is the minimum hash value
    *unique_hashes.iter().min().unwrap_or(&0)
}

/// Helper function to copy one grid to another
fn copy_grid(src: &[[[bool; 5]; 5]; 5], dst: &mut [[[bool; 5]; 5]; 5]) {
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                dst[x][y][z] = src[x][y][z];
            }
        }
    }
}

/// Helper function to get permuted indices
fn get_permuted_indices(
    x: usize,
    y: usize,
    z: usize,
    i: usize,
    j: usize,
    k: usize,
) -> (usize, usize, usize) {
    let coords = [x, y, z];
    (coords[i], coords[j], coords[k])
}

/// Helper function to rotate a grid 90 degrees around a specified axis
fn rotate_grid(grid: &mut [[[bool; 5]; 5]; 5], axis: usize) {
    let mut rotated = [[[false; 5]; 5]; 5];

    match axis {
        0 => {
            // Rotate around x-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        rotated[x][z][4 - y] = grid[x][y][z];
                    }
                }
            }
        }
        1 => {
            // Rotate around y-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        rotated[z][y][4 - x] = grid[x][y][z];
                    }
                }
            }
        }
        2 => {
            // Rotate around z-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        rotated[y][4 - x][z] = grid[x][y][z];
                    }
                }
            }
        }
        _ => panic!("Invalid axis"),
    }

    copy_grid(&rotated, grid);
}

/// Helper function to flip a grid across a specified axis
fn flip_grid(grid: &mut [[[bool; 5]; 5]; 5], axis: usize) {
    let mut flipped = [[[false; 5]; 5]; 5];

    match axis {
        0 => {
            // Flip across x-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        flipped[4 - x][y][z] = grid[x][y][z];
                    }
                }
            }
        }
        1 => {
            // Flip across y-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        flipped[x][4 - y][z] = grid[x][y][z];
                    }
                }
            }
        }
        2 => {
            // Flip across z-axis
            for x in 0..5 {
                for y in 0..5 {
                    for z in 0..5 {
                        flipped[x][y][4 - z] = grid[x][y][z];
                    }
                }
            }
        }
        _ => panic!("Invalid axis"),
    }

    copy_grid(&flipped, grid);
}

/// Calculate a standard hash for a grid
fn calculate_hash(grid: &[[[bool; 5]; 5]; 5]) -> u128 {
    let mut hash: u128 = 0;
    let mut bit_pos: u128 = 0;

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                if grid[x][y][z] {
                    hash |= 1 << bit_pos;
                }
                bit_pos += 1;
            }
        }
    }

    hash
}

/// Example test function to verify the hash function
pub fn test_hash_function() {
    // Create a simple test grid with a pattern
    let mut original_grid = [[[false; 5]; 5]; 5];
    original_grid[0][0][0] = true;
    original_grid[0][0][1] = true;
    original_grid[0][1][0] = true;

    // Calculate the hash
    let original_hash = grid_hash(&original_grid);
    println!("Original hash: {}", original_hash);

    // Test with a rotated version
    let mut rotated_grid = [[[false; 5]; 5]; 5];
    copy_grid(&original_grid, &mut rotated_grid);
    rotate_grid(&mut rotated_grid, 0);

    let rotated_hash = grid_hash(&rotated_grid);
    println!("Rotated hash: {}", rotated_hash);

    // Test with a flipped version
    let mut flipped_grid = [[[false; 5]; 5]; 5];
    copy_grid(&original_grid, &mut flipped_grid);
    flip_grid(&mut flipped_grid, 0);

    let flipped_hash = grid_hash(&flipped_grid);
    println!("Flipped hash: {}", flipped_hash);

    // Hashes should be the same if the function is rotation/flip invariant
    println!(
        "Are hashes equal? {}",
        original_hash == rotated_hash && rotated_hash == flipped_hash
    );

    // Test with a different grid
    let mut different_grid = [[[false; 5]; 5]; 5];
    different_grid[0][0][0] = true;
    different_grid[0][0][1] = true;
    different_grid[1][1][1] = true; // Different pattern

    let different_hash = grid_hash(&different_grid);
    println!("Different grid hash: {}", different_hash);
    println!(
        "Are different grids distinguished? {}",
        original_hash != different_hash
    );
}
