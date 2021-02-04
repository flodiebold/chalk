use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::usize;

pub(crate) struct Stack {
    // program: Arc<ProgramEnvironment>,
    entries: Vec<StackEntry>,
    overflow_depth: usize,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub(crate) struct StackDepth {
    depth: usize,
}

/// The data we actively keep for each goal on the stack.
pub(crate) struct StackEntry {
    /// Was this a coinductive goal?
    coinductive_goal: bool,

    /// Initially false, set to true when some subgoal depends on us.
    cycle: bool,
}

impl Stack {
    pub(crate) fn new(
        // program: &Arc<ProgramEnvironment>,
        overflow_depth: usize,
    ) -> Self {
        Stack {
            // program: program.clone(),
            entries: vec![],
            overflow_depth,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn push(&mut self, coinductive_goal: bool) -> StackDepth {
        let depth = StackDepth {
            depth: self.entries.len(),
        };

        if depth.depth >= self.overflow_depth {
            // This shoudl perhaps be a result or something, though
            // really I'd prefer to move to subgoal abstraction for
            // guaranteeing termination. -nmatsakis
            panic!("overflow depth reached")
        }

        self.entries.push(StackEntry {
            coinductive_goal,
            cycle: false,
        });
        depth
    }

    pub(crate) fn pop(&mut self, depth: StackDepth) {
        assert_eq!(
            depth.depth + 1,
            self.entries.len(),
            "mismatched stack push/pop"
        );
        self.entries.pop();
    }

    /// True if either all the goals from the top of the stack down to (and
    /// including) the given depth are  coinductive or if all goals are inductive
    /// (i.e. not coinductive).
    pub(crate) fn mixed_inductive_coinductive_cycle_from(&self, depth: StackDepth) -> bool {
        let (inductive, coinductive) =
            self.entries[depth.depth..]
                .iter()
                .fold((false, false), |(ind, coind), entry| {
                    (
                        ind || !entry.coinductive_goal,
                        coind || entry.coinductive_goal,
                    )
                });
        inductive && coinductive
    }
}

impl StackEntry {
    pub(crate) fn flag_cycle(&mut self) {
        self.cycle = true;
    }

    pub(crate) fn read_and_reset_cycle_flag(&mut self) -> bool {
        mem::replace(&mut self.cycle, false)
    }
}

impl Index<StackDepth> for Stack {
    type Output = StackEntry;

    fn index(&self, depth: StackDepth) -> &StackEntry {
        &self.entries[depth.depth]
    }
}

impl IndexMut<StackDepth> for Stack {
    fn index_mut(&mut self, depth: StackDepth) -> &mut StackEntry {
        &mut self.entries[depth.depth]
    }
}
