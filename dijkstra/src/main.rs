use rand::Rng;
use std::cmp;

// number of vertices and sparsity for the randomly generated graph
const V: usize = 5;
const S: f64 = 1.;
// substitue for infinite distance. A cleaner way would be to do this with an Enum.
const INF: f64 = std::f64::MAX;

fn main() {
    // generate a random graph and initialize the bookkeeping data structures for the Dijkstra algo.
    let graph = make_graph(V, S);
    let mut visited = [false; V];
    let mut previous: [Option<usize>; V] = [Option::None; V];
    let mut distance = [INF; V];
    distance[0] = 0.;

    // start the algorithm with the first node and loop until the shortest path to the final node
    // has been found or all nodes have been visited (there is no connected path between start/end)
    let mut current_node: usize = 0;
    loop {
        visited[current_node] = true;

        // Loop over all edges and consider only those that start at the current node.
        // Then update the vectors for distance and previous nodes.
        for e in &graph {
            if e.src == current_node && distance[current_node] + e.len < distance[e.dst] {
                distance[e.dst] = distance[current_node] + e.len;
                previous[e.dst] = Some(current_node);
            } else if e.dst == current_node && distance[current_node] + e.len < distance[e.src] {
                distance[e.src] = distance[current_node] + e.len;
                previous[e.src] = Some(current_node);
            }
        }

        // find next node
        let mut next_distance = INF;
        for i in 0..V {
            if !visited[i] && distance[i] < next_distance {
                current_node = i;
                next_distance = distance[i];
            }
        }
        // if there are no more nodes left, the current_node has not been updated and we can abort
        if visited[current_node] {
            break;
        }
    }

    println!("The randomly generated graph is given by:");
    println!("SRC, DST, LEN");
    for e in &graph {
        println!(
            "{:3}, {:3}, {:3}",
            e.src,
            e.dst,
            (10. * e.len).round() / 10.
        );
    }
    println!("\n");

    if visited[V - 1] {
        println!(
            "The connected path from finish to start node  with a total length of {:3} is:",
            (10. * distance[V - 1]).round() / 10.
        );
        println!("SRC, DST, LEN");
        let mut current_node = V - 1;
        while current_node != 0 {
            match previous[current_node] {
                Some(x) => {
                    println!("{:3}, {:3}", current_node, x);
                    current_node = x
                }
                None => break,
            }
        }
    } else {
        println!("There is no connected path between start an finish node.");
    }
}

// This holds
struct Edge {
    src: usize,
    dst: usize,
    len: f64,
}

// This is just a function that randomly generates an example graph
// The sparseness parameter indicates how many edges each vertex should have on average
fn make_graph(no_of_vertices: usize, sparsity: f64) -> Vec<Edge> {
    let mut graph: Vec<Edge> = Vec::new();

    while (graph.len() as f64) / (no_of_vertices as f64) < sparsity {
        let one = rand::thread_rng().gen_range(0, no_of_vertices);
        let two = rand::thread_rng().gen_range(0, no_of_vertices);
        // by convention the smallest value is always the source (undirected graph)
        let src = cmp::min(one, two);
        let dst = cmp::max(one, two);

        // check if there is already an edge connecting the two nodes or if src and dst are the same
        let mut duplicate = false;
        for e in &graph {
            if e.src == src && e.dst == dst {
                duplicate = true;
                break;
            }
        }
        if one != two && !duplicate {
            let len: f64 = rand::thread_rng().gen_range(0., 1.);
            graph.push(Edge {
                src: src,
                dst: dst,
                len: len,
            })
        }
    }

    graph
}
