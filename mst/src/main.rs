// iFarbod 2023

use rand::Rng;
use std::collections::HashSet;
use std::iter::FromIterator;

const NODES: usize = 7;
const EDGES_PER_NODE: usize = 3;
const MIN_WEIGHT: u32 = 10;
const MAX_WEIGHT: u32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(usize);

#[derive(Debug, Clone, PartialEq)]
struct Edge
{
    from: Node,
    to: Node,
    weight: u32,
}

fn generate_graph(num_nodes: usize, num_edges: usize, min_weight: u32, max_weight: u32) -> (HashSet<Node>, Vec<Edge>)
{
    let mut rng = rand::thread_rng();
    let mut nodes: HashSet<Node> = HashSet::new();
    let mut edges: Vec<Edge> = Vec::new();

    for i in 0..num_nodes
    {
        nodes.insert(Node(i));
    }

    for _ in 0..num_nodes
    {
        let mut node_edges: Vec<Edge> = Vec::new();
        while node_edges.len() < num_edges
        {
            let from = rng.gen_range(0..num_nodes);
            let to = rng.gen_range(0..num_nodes);
            if from != to
            {
                let weight = rng.gen_range(min_weight..=max_weight);
                let edge = Edge {
                    from: Node(from),
                    to: Node(to),
                    weight,
                };
                if !edges.contains(&edge)
                {
                    node_edges.push(edge.clone());
                    edges.push(edge);
                }
            }
        }
    }

    (nodes, edges)
}

fn find(parents: &mut Vec<Node>, node: Node) -> Node
{
    if parents[node.0 as usize].0 != node.0
    {
        parents[node.0 as usize] = find(parents, parents[node.0 as usize]);
    }
    parents[node.0 as usize].clone()
}

fn union(parents: &mut Vec<Node>, rank: &mut Vec<usize>, node1: Node, node2: Node)
{
    let root1 = find(parents, node1);
    let root2 = find(parents, node2);

    if root1 == root2
    {
        return;
    }

    if rank[root1.0 as usize] > rank[root2.0 as usize]
    {
        parents[root2.0 as usize] = root1;
    }
    else
    {
        parents[root1.0 as usize] = root2;
        if rank[root1.0 as usize] == rank[root2.0 as usize]
        {
            rank[root2.0 as usize] += 1;
        }
    }
}

fn kruskal(edges: Vec<Edge>, nodes: HashSet<Node>) -> Vec<Edge>
{
    let mut sorted_edges = edges.clone();
    sorted_edges.sort_by_key(|edge| edge.weight);

    let mut parents: Vec<Node> = (0..nodes.len()).map(|i| Node(i)).collect();
    let mut rank: Vec<usize> = vec![0; nodes.len()];

    let mut mst: Vec<Edge> = Vec::new();

    for edge in sorted_edges
    {
        let root1 = find(&mut parents, edge.from);
        let root2 = find(&mut parents, edge.to);

        if root1 != root2
        {
            mst.push(edge.clone());
            union(&mut parents, &mut rank, edge.from, edge.to);
        }
    }

    mst
}

fn main()
{
    // The actual question
    let (nodes, edges) = generate_graph(NODES, EDGES_PER_NODE, MIN_WEIGHT, MAX_WEIGHT);

    println!("The graph we have:");
    for edge in &edges
    {
        println!(
            "{} to {} is {}",
            (edge.from.0 as u8 + b'A') as char,
            (edge.to.0 as u8 + b'A') as char,
            edge.weight
        );
    }

    let mst = kruskal(edges, HashSet::from_iter(nodes.clone()));

    println!("\nMinimum Spanning Tree:");
    for edge in mst
    {
        println!(
            "{} to {} is {}",
            (edge.from.0 as u8 + b'A') as char,
            (edge.to.0 as u8 + b'A') as char,
            edge.weight
        );
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn verify_kruskal()
    {
        let edges = vec![
            Edge {
                from: Node(0),
                to: Node(4),
                weight: 19,
            },
            Edge {
                from: Node(0),
                to: Node(1),
                weight: 15,
            },
            Edge {
                from: Node(1),
                to: Node(6),
                weight: 15,
            },
            Edge {
                from: Node(6),
                to: Node(2),
                weight: 20,
            },
            Edge {
                from: Node(2),
                to: Node(3),
                weight: 14,
            },
            Edge {
                from: Node(0),
                to: Node(6),
                weight: 10,
            },
            Edge {
                from: Node(4),
                to: Node(1),
                weight: 18,
            },
            Edge {
                from: Node(4),
                to: Node(3),
                weight: 12,
            },
            Edge {
                from: Node(2),
                to: Node(5),
                weight: 17,
            },
            Edge {
                from: Node(5),
                to: Node(3),
                weight: 11,
            },
        ];
        let nodes: HashSet<Node> = edges
            .iter()
            .flat_map(|edge| vec![edge.from.clone(), edge.to.clone()])
            .collect();

        let mst = kruskal(edges, HashSet::from_iter(nodes.clone()));

        assert_eq!(
            mst,
            [
                Edge {
                    from: Node(0),
                    to: Node(6),
                    weight: 10
                },
                Edge {
                    from: Node(5),
                    to: Node(3),
                    weight: 11
                },
                Edge {
                    from: Node(4),
                    to: Node(3),
                    weight: 12
                },
                Edge {
                    from: Node(2),
                    to: Node(3),
                    weight: 14
                },
                Edge {
                    from: Node(0),
                    to: Node(1),
                    weight: 15
                },
                Edge {
                    from: Node(4),
                    to: Node(1),
                    weight: 18
                }
            ]
        );
    }
}
