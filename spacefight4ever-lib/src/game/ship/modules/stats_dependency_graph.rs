// now it's getting complicated

/// each stat 'knows':
/// how to compute itself
/// and what it depends on
/// example:
/// fn build_stat_definitions() -> HashMap<StatKey, StatDefinition> {
///     use StatKey::*;
///
///     let mut defs = HashMap::new();
///
///     defs.insert(MaxSpeed, StatDefinition {
///         deps: vec![Thrust, Mass],
///         compute: |s| {
///             let thrust = s.values.get(&Thrust).copied().unwrap_or(0.0);
///             let mass = s.values.get(&Mass).copied().unwrap_or(1.0);
///             thrust / mass
///         },
///     });
///
///     defs.insert(Speed, StatDefinition {
///         deps: vec![MaxSpeed],
///         compute: |s| {
///             // could be dynamic, but for demo:
///             s.values.get(&MaxSpeed).copied().unwrap_or(0.0)
///         },
///     });
///
///     defs
/// }
pub struct StatDefinition {
    pub deps: Vec<StatKey>,
    pub compute: fn(&Stats) -> f32,
}

/// dependency graph (reverse lookup) - who depends on the value?
pub struct StatGraph {
    pub dependents: HashMap<StatKey, Vec<StatKey>>,
}

pub struct StatDefinitions {
    pub defs:HashMap<StatKey, StatDefinition>,
}

/// build stat dependency graph from stat definitions
fn build_graph(defs: &HashMap<StatKey, StatDefinition>) -> StatGraph {
    let mut graph = HashMap::<StatKey, Vec<StatKey>>::new();

    for (stat, def) in defs {
        for dep in &def.deps {
            graph.entry(dep.clone())
                .or_default()
                .push(stat.clone());
        }
    }

    StatGraph { dependents: graph }
}

/// dirty tracking per entity
#[derive(Component, Default)]
pub struct DirtyStats {
    pub set: HashSet<StatKey>,
}

/// mark something dirty
fn mark_dirty(
    dirty: &mut DirtyStats,
    graph: &StatGraph,
    stat: StatKey,
) {
    let mut stack = vec![stat];

    while let Some(s) = stack.pop() {
        if dirty.set.insert(s.clone()) {
            if let Some(deps) = graph.dependents.get(&s) {
                for d in deps {
                    stack.push(d.clone());
                }
            }
        }
    }
}

/// reactive compute system
pub fn compute_dirty_stats(
    mut query: Query<(&mut Stats, &mut DirtyStats)>,
    defs: Res<HashMap<StatKey, StatDefinition>>,
) {
    for (mut stats, mut dirty) in &mut query {
        let dirty_list: Vec<_> = dirty.set.drain().collect();

        for stat in dirty_list {
            if let Some(def) = defs.get(&stat) {
                let value = (def.compute)(&stats);
                stats.values.insert(stat, value);
            }
        }
    }
}