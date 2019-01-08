use std::fmt::Debug;
use gp::*;
use rand::Rng;
use std::mem;

/// The crossover mode in use. See `Crossover`.
#[derive(PartialEq, Clone, Copy, Debug)]
enum CrossoverMode {
    /// Corresponds to `Crossover::one_point`.
    OnePoint,
    /// Corresponds to `Crossover::one_point_leaf_biased`.
    OnePointLeafBiased(f32),
    HardPrune(usize),
}

/// Configures crossover (mating) between GP individuals.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Crossover {
    mode: CrossoverMode,
}

impl Crossover {
    /// Get an operator to perform one-point crossover between two individuals.
    ///
    /// The subtree at a random position in one individual will be swapped with a random
    /// position in a second individual.
    pub fn one_point() -> Crossover {
        Crossover {
            mode: CrossoverMode::OnePoint,
        }
    }

    /// Get an operator to perform one-point crossover between two individuals.
    ///
    /// The subtree at a random position in one individual will be swapped with a random
    /// position in a second individual. Each swap points will be a terminal with `termpb`
    /// probability.
    #[doc(hidden)]
    pub fn one_point_leaf_biased(termpb: f32) -> Crossover {
        Crossover {
            mode: CrossoverMode::OnePointLeafBiased(termpb),
        }
    }

    /// Get an operator to perform one-point crossover between two individuals.
    /// 
    /// The subtree at a random position in one individual will be swapped with a random
    /// position in a second individual. Then the subrees will be pruned at `max_depth`
    /// and replaced with the first leaf they contain (depth-first search)
    pub fn hard_prune(max_depth: usize) -> Crossover {
        Crossover {
            mode: CrossoverMode::HardPrune(max_depth)
        }
    }

    /// Crossover (mate) two individuals according to the configured crossover mode.
    pub fn mate<T, R>(&self, indv1: &mut Individual<T>, indv2: &mut Individual<T>, rng: &mut R)
    where
        T: Tree,
        R: Rng,
    {
        match self.mode {
            CrossoverMode::OnePoint => self.mate_one_point(indv1, indv2, rng),
            CrossoverMode::OnePointLeafBiased(termpb) => {
                self.mate_one_point_leaf_biased(indv1, indv2, termpb, rng)
            }
            CrossoverMode::HardPrune(max_depth) => {
                self.mate_hard_prune(indv1, indv2, max_depth, rng)
            }
        }
    }

    fn mate_one_point<T: Tree, R: Rng>(
        &self,
        indv1: &mut Individual<T>,
        indv2: &mut Individual<T>,
        rng: &mut R,
    ) where
        T: Tree,
        R: Rng,
    {
        let target_index1 = rng.gen_range(0, indv1.nodes_count());
        let target_index2 = rng.gen_range(0, indv2.nodes_count());

        indv1.tree.map_while(|node1, index1, _| {
            if index1 == target_index1 {
                indv2.tree.map_while(|node2, index2, _| {
                    if index2 == target_index2 {
                        mem::swap(node1, node2);
                        false
                    } else {
                        true
                    }
                });
                false
            } else {
                true
            }
        });

        indv1.recalculate_metadata();
        indv2.recalculate_metadata();
    }

    fn mate_one_point_leaf_biased<T, R>(
        &self,
        indv1: &mut Individual<T>,
        indv2: &mut Individual<T>,
        bias: f32,
        rng: &mut R,
    ) where
        T: Tree,
        R: Rng,
    {
        let leaf = rng.gen_bool(f64::from(bias));

        let target_index1 = rng.gen_range(0, indv1.nodes_count());
        let mut target_index2 = rng.gen_range(0, indv2.nodes_count());
        let mut node_counter = 0;
        indv1.tree.map_while(|node1, index1, _| {
            if index1 == target_index1 {
                indv2.tree.map_while(|node2, _, _| {
                    let is_leaf = node2.count_children() == 0;
                    if is_leaf != leaf {
                        target_index2 -= 1;
                        true
                    } else {
                        let ret = if node_counter == target_index2 {
                            mem::swap(node1, node2);
                            false
                        } else {
                            true
                        };
                        node_counter += 1;
                        ret
                    }
                });
                false
            } else {
                true
            }
        });
    }
    fn mate_hard_prune<T, R>(
        &self,
        indv1: &mut Individual<T>,
        indv2: &mut Individual<T>,
        max_depth: usize,
        rng: &mut R,
    ) where
        T: Tree + Debug,
        R: Rng,
    {
        self.mate_one_point(indv1, indv2, rng);
        indv1.prune_at(max_depth);
    }

}
