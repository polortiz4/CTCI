// Solutions to Exercise 4.7

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct CyclicError;

impl fmt::Display for CyclicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot build projects in order")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Project {
    pub name: String,
}

type ProjectPair = (Project, Project);

/// Given a list of projects and a list of project dependencies, return a list with them in
/// a valid order of execution
///
/// # Examples:__rust_force_expr!
/// ```
/// use ctci::trees_and_graphs::build_order::build_order;
/// use ctci::trees_and_graphs::build_order::Project;
///
/// let a = Project {name: String::from("a"),};
/// let b = Project {name: String::from("b"),};
///
/// let projects = vec![a.clone(), b.clone()];
///
/// let ab = (a.clone(), b.clone());
///
/// let dependencies = vec![ab.clone()];
///
/// let res = build_order(&projects, &dependencies);
/// assert_eq!(vec![a.clone(), b.clone()], res.unwrap());
/// ```
pub fn build_order(
    projects: &Vec<Project>,
    dependencies: &Vec<ProjectPair>,
) -> Result<Vec<Project>, CyclicError> {
    let mut dependant_map = HashMap::<Project, HashSet<Project>>::new();
    let mut dependency_map = HashMap::<Project, Vec<Project>>::new();

    for project in projects {
        dependant_map.insert(project.clone(), HashSet::new());
        dependency_map.insert(project.clone(), Vec::new());
    }
    for (dependency, dependant) in dependencies {
        // We use unwrap here as we assume the inputs are well-formed: all items in dependencies are also in projects
        dependant_map
            .get_mut(&dependant)
            .unwrap()
            .insert(dependency.clone());
        dependency_map
            .get_mut(&dependency)
            .unwrap()
            .push(dependant.clone());
    }

    let mut queue = VecDeque::<Project>::new();
    let mut result = Vec::<Project>::new();
    while {
        while let Some(project) = queue.pop_front() {
            dependant_map.remove(&project);
            if let Some(dependants) = dependency_map.get(&project) {
                for dependant in dependants {
                    if let Some(dep_set) = dependant_map.get_mut(dependant) {
                        dep_set.remove(&project);
                    }
                }
            }
        }
        for (dependant, dependency) in &dependant_map {
            if dependency.is_empty() {
                queue.push_back(dependant.clone());
                result.push(dependant.clone());
            }
        }
        !queue.is_empty()
    } {}
    if !dependant_map.is_empty() {
        Err(CyclicError)
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main_test() {
        let a = Project {
            name: String::from("a"),
        };
        let b = Project {
            name: String::from("b"),
        };
        let c = Project {
            name: String::from("c"),
        };
        let d = Project {
            name: String::from("d"),
        };
        let e = Project {
            name: String::from("e"),
        };
        let f = Project {
            name: String::from("f"),
        };

        let projects = vec![
            a.clone(),
            b.clone(),
            c.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
        ];

        let ad = (a.clone(), d.clone());
        let fb = (f.clone(), b.clone());
        let bd = (b.clone(), d.clone());
        let fa = (f.clone(), a.clone());
        let dc = (d.clone(), c.clone());
        let dependencies = vec![ad.clone(), fb.clone(), bd.clone(), fa.clone(), dc.clone()];
        let res = build_order(&projects, &dependencies);
        let mut possible_results: Vec<Vec<Project>> = Vec::new();
        possible_results.push(vec![
            f.clone(),
            e.clone(),
            a.clone(),
            b.clone(),
            d.clone(),
            c.clone(),
        ]);
        possible_results.push(vec![
            e.clone(),
            f.clone(),
            b.clone(),
            a.clone(),
            d.clone(),
            c.clone(),
        ]);
        possible_results.push(vec![
            e.clone(),
            f.clone(),
            a.clone(),
            b.clone(),
            d.clone(),
            c.clone(),
        ]);
        possible_results.push(vec![
            f.clone(),
            e.clone(),
            b.clone(),
            a.clone(),
            d.clone(),
            c.clone(),
        ]);
        assert!(possible_results.contains(&res.unwrap()));

        let cf = (c.clone(), f.clone());
        let dependencies = vec![
            ad.clone(),
            fb.clone(),
            bd.clone(),
            fa.clone(),
            dc.clone(),
            cf.clone(),
        ];

        assert_eq!(
            Some(CyclicError),
            build_order(&projects, &dependencies).err()
        );
    }
}
