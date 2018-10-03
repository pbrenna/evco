use rand::{Rng, RngCore};

/// The tree generation mode in use. See `TreeGen`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TreeGenMode {
    /// Corresponds to `TreeGen::perfect`.
    Perfect(usize),
    /// Corresponds to `TreeGen::full`.
    Full,
    /// Corresponds to `TreeGen::full_ranged`.
    FullRanged(usize),
}

/// Configures depth and properties of GP trees.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TreeGen<R>
where
    R: Rng,
{
    /// Which tree depth logic to use.
    mode: TreeGenMode,
    /// A `rand::Rng` implementation for generating random tree nodes.
    rng: R,
    /// The minimum depth of trees to generate.
    min_depth: usize,
    /// The maximum depth of trees to generate.
    max_depth: usize,
}

impl<R> TreeGen<R>
where
    R: Rng,
{
    /// Generate perfect trees. All leaves are at the same depth in the range
    /// [min_depth, max_depth].
    ///
    /// **This is the equivalent of DEAP's `genFull`.**
    pub fn perfect(mut rng: R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Perfect(chosen_depth),
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Generate full trees with leaves at varying depths. Leaf depths are
    /// linearly distributed between min_depth and a chosen depth in the range.
    ///
    /// **This is NOT the same as DEAP's `genFull`. See `TreeGen::perfect`.**
    pub fn full(rng: R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        TreeGen {
            rng: rng,
            mode: TreeGenMode::Full,
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Generate full trees with leaves at varying depths. Leaf depths are
    /// linearly distributed between min_depth and a chosen depth in the range.
    ///
    /// **This is the equivalent of DEAP's `genGrow`.**
    pub fn full_ranged(mut rng: R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        let chosen_depth = rng.gen_range(min_depth, max_depth + 1);
        TreeGen {
            rng: rng,
            mode: TreeGenMode::FullRanged(chosen_depth),
            min_depth: min_depth,
            max_depth: max_depth,
        }
    }

    /// Randomly choose between `TreeGen::perfect` and `TreeGen::full_ranged`.
    ///
    /// **This is the equivalent of DEAP's `genHalfAndHalf`.**
    // @TODO: This choice needs to happen at runtime.
    pub fn half_and_half(mut rng: R, min_depth: usize, max_depth: usize) -> TreeGen<R> {
        if rng.gen() {
            Self::perfect(rng, min_depth, max_depth)
        } else {
            Self::full_ranged(rng, min_depth, max_depth)
        }
    }

    /// Chooses whether to generate a Leaf node. Used by `Tree::child`.
    pub fn have_reached_a_leaf(&mut self, current_depth: usize) -> bool {
        match self.mode {
            TreeGenMode::Perfect(chosen_depth) => current_depth == chosen_depth,
            TreeGenMode::Full => {
                // This given an equal 1-in-depth_interval chance at every intermediary depth.
                // Earlier checks ensure in the (1/depth)*(depth-1) case we reach chosen_depth,
                // we do finally place a Leaf.
                let depth_interval = self.max_depth - self.min_depth;
                // @TODO: Avoid converting depth_interval.
                current_depth == self.max_depth
                    || (current_depth >= self.min_depth)
                        && self.gen_bool(1.0 / depth_interval as f64)
            }
            TreeGenMode::FullRanged(chosen_depth) => {
                // This given an equal 1-in-depth_interval chance at every intermediary depth.
                // Earlier checks ensure in the (1/depth)*(depth-1) case we reach chosen_depth,
                // we do finally place a Leaf.
                let depth_interval = chosen_depth - self.min_depth;
                // @TODO: Avoid converting depth_interval.
                current_depth == chosen_depth
                    || (current_depth >= self.min_depth)
                        && self.gen_bool(1.0 / depth_interval as f64)
            }
        }
    }
}

impl<R> RngCore for TreeGen<R>
where
    R: Rng,
{
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    // some RNGs implement these more efficiently than the default, so
    // we might as well defer to them.
    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}
