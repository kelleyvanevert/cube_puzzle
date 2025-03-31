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

    fn next(&self) -> Vec<State> {
        let Some(next_block_def) = get_block_def(self.placements.len()) else {
            panic!("Could not find next block from: {:?}", self);
        };

        let mut found = vec![];

        for (sx, sy, sz) in next_block_def {
            // find spots where it fits
            'find_spot: for &(x, y, z) in &self.free {
                let mut touching_boundary = false;

                for dx in 0..sx {
                    for dy in 0..sy {
                        for dz in 0..sz {
                            if self.boundary.contains(&((x + dx, y + dy, z + dz))) {
                                touching_boundary = true;
                            }

                            if !self.free.contains(&((x + dx, y + dy, z + dz))) {
                                continue 'find_spot;
                            }
                        }
                    }
                }

                // optimization
                if !touching_boundary {
                    continue 'find_spot;
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
    let mut queue = BinaryHeap::new_by_key(|s: &State| {
        // 20 - s.placements.len()
        128 - s.space_left()
    });

    queue.push(State::default());

    let mut i = 0;

    while let Some(state) = queue.pop() {
        i += 1;

        // if i == 4 {
        //     panic!("done")
        // }

        if i % 100_000 == 0 {
            println!(
                "considering.. ({}) {} -- {}",
                queue.len(),
                state.placements.len(),
                state.space_left()
            );
            // println!("{:?}", state);
            // considering..11 - -13
            // State { placements: [Placement { block: (3, 2, 2), pos: (0, 0, 0) }, Placement { block: (3, 2, 2), pos: (1, 1, 3) }, Placement { block: (3, 2, 2), pos: (2, 3, 0) }, Placement { block: (2, 3, 2), pos: (0, 2, 0) }, Placement { block: (2, 3, 2), pos: (3, 0, 0) }, Placement { block: (3, 2, 2), pos: (1, 3, 2) }, Placement { block: (1, 4, 2), pos: (4, 1, 3) }, Placement { block: (4, 2, 1), pos: (1, 1, 2) }, Placement { block: (4, 1, 2), pos: (0, 0, 3) }, Placement { block: (1, 4, 2), pos: (0, 1, 2) }, Placement { block: (4, 2, 1), pos: (0, 3, 4) }], free: {(2, 0, 2), (2, 2, 0), (0, 0, 2), (4, 0, 2), (3, 0, 2), (4, 3, 2), (0, 1, 4), (4, 0, 4), (2, 2, 1), (0, 2, 4), (4, 0, 3), (4, 4, 2), (1, 0, 2)} }
        }

        if state.space_left() == 0 {
            println!("Found solution!");
            println!("{:?}", state);
            return;
        }

        let n = state.next();
        queue.extend(n);
    }
}
