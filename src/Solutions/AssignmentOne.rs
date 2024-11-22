use std::fmt::{Debug, Display, Formatter};
use std::io::{stdin, BufRead, BufReader};
use std::str::{FromStr, SplitWhitespace};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

struct Input<B> {
    inner: B,
    buffer: String,
}

impl<B: BufRead> Input<B> {
    pub fn new(inner: B) -> Input<B> {
        Self {
            inner,
            buffer: String::new(),
        }
    }

    pub fn line(&mut self) -> Line {
        self.buffer.clear();
        self.inner.read_line(&mut self.buffer).unwrap();
        Line {
            split: self.buffer.split_whitespace(),
        }
    }
}

struct Line<'a> {
    split: SplitWhitespace<'a>,
}

impl<'a> Line<'a> {
    fn next<T: FromStr>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug
    {
        self
            .split
            .next()
            .unwrap()
            .parse::<T>()
            .unwrap()
    }
    fn pair<T: FromStr>(&mut self) -> (T, T)
    where
        T: FromStr,
        T::Err: Debug
    {
        (self.next(), self.next())
    }
    fn collect<T: FromStr>(self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug
    {
        self
            .split
            .map(|d| d.parse::<T>().unwrap())
            .collect::<Vec<T>>()
    }
}

type Vertex = u32;

#[derive(Clone, Debug)]
struct Edge {
    to: Vertex,
    weight: u32,
    departure_time: u32,
}

struct Graph {
    adjacency: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        Graph {
            adjacency: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, from: Vertex, to: Vertex, weight: u32, departure_time: u32) {
        self.adjacency[from as usize].push(Edge { to, weight, departure_time });
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (vertex, edges) in self.adjacency.iter().enumerate() {
            writeln!(f, "Vertex {}: ", vertex)?;
            for edge in edges {
                writeln!(f, "  -> to: {}, weight: {}, departure time: {}", edge.to, edge.weight, edge.departure_time)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    time: u32,
    position: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.time.cmp(&self.time)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn dijkstra(graph: &Graph, start: Vertex, end: Vertex) -> Option<u32> {
    let mut dist: Vec<u32> = (0..graph.adjacency.len()).map(|_| u32::MAX).collect();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<Vertex> = HashSet::new();

    dist[start as usize] = 0;
    heap.push(State { time: 0, position: start });

    while let Some(State { time: current_time, position }) = heap.pop() {
        // Skip the node if it has already been visited
        if visited.contains(&position) {
            continue;
        }

        // Mark the node as visited
        visited.insert(position);

        // If we reach the destination, return the time
        if position == end {
            return Some(current_time);
        }

        // Relax edges
        for edge in &graph.adjacency[position as usize] {
            if current_time > edge.departure_time { continue; }

            let next = State { time: edge.departure_time + edge.weight, position: edge.to };
            if next.time < dist[next.position as usize] {
                heap.push(next);
                dist[next.position as usize] = next.time;
            }
        }
    }
    None
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (b, l) = input.line().pair::<u32>();
    const MERCATOR: u32 = 0;
    let home: u32 = l-1;
    let mut graph = Graph::new(l as usize);

    (0..b).for_each(|_| {
        let locations = input
            .line()
            .collect::<Vertex>();
        // Loop invariant: At this point, all locations at te current line are present in the list in order of representation

        let departures = input
            .line()
            .collect::<u32>();
        // Loop invariant: At this point, all departure times on the current line are present in the list in strictly increasing order

        locations
            .windows(2)
            .zip(departures.windows(2))
            .for_each(|(vertexes, weights)| {
                graph.add_edge(
                    vertexes[0],
                    vertexes[1],
                    weights[1] - weights[0],
                    weights[0]
                );
            });
        // Loop invariant: At this point, edges are added to the graph for each pair of consecutive locations with the correct weight and departure time.
    });

    //println!("{graph}");
    println!("{}", dijkstra(&graph, MERCATOR, home).unwrap());
}
