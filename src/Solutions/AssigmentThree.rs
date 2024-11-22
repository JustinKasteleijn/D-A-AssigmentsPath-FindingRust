use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdin, BufRead, BufReader};
use std::str::{FromStr, SplitWhitespace};

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
        T::Err: Debug,
    {
        self.split.next().unwrap().parse::<T>().unwrap()
    }

    fn pair<T: FromStr>(&mut self) -> (T, T)
    where
        T: FromStr,
        T::Err: Debug,
    {
        (self.next(), self.next())
    }

    fn collect<T: FromStr>(self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.split.map(|d| d.parse::<T>().unwrap()).collect::<Vec<T>>()
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
                writeln!(
                    f,
                    "  -> to: {}, weight: {}, departure time: {}",
                    edge.to, edge.weight, edge.departure_time
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    transfer_time: u32,
    time: u32,
    location: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.transfer_time.cmp(&self.transfer_time)
            .then_with(|| self.transfer_time.cmp(&other.transfer_time))
            .then_with(|| self.time.cmp(&other.time))
            .then_with(|| self.location.cmp(&other.location))
    }
}

fn dijkstra(graph: &Graph, start: Vertex, end: Vertex) -> Option<u32> {
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();
    let mut max_tranfer_time = 0;
    priority_queue.push(State { transfer_time: 0, time: 0, location: start });

    while let Some(State { transfer_time, time, location }) = priority_queue.pop() {
        println!("transfer time: {}, current time: {}, location {}", transfer_time, time, location);

        max_tranfer_time = max_tranfer_time.max(transfer_time);

        if location == end {
            return Some(max_tranfer_time as u32);
        }

        for edge in &graph.adjacency[location as usize] {
            if time > edge.departure_time {
                continue;
            }

            let wait_time = edge.departure_time - time;
            let travel_time = edge.departure_time + edge.weight;
            let next = State {
                transfer_time: transfer_time + (wait_time),
                time: travel_time,
                location: edge.to,
            };

            priority_queue.push(next);
        }
    }
    None
}

type Cache = HashMap<(Vertex, u32, u32), u32>;

fn dfs(
    graph: &Graph,
    start: Vertex,
    end: Vertex,
    time: u32,
    transfer_time: u32,
    visited: &mut HashSet<Vertex>,
    cache: &mut Cache,
    best_solution: &mut u32
) -> u32 {
    if start == end {
        return transfer_time;
    }

    if transfer_time >= *best_solution {
        return 0;
    }

    if let Some(&cached_result) = cache.get(&(start, time, transfer_time)) {
        return cached_result;
    }

    visited.insert(start);

    let mut max_transfer_time = 0;

    for edge in &graph.adjacency[start as usize] {
        if time <= edge.departure_time && !visited.contains(&edge.to) {
            let wait_time = edge.departure_time - time;

            let result = dfs(
                graph,
                edge.to,
                end,
                edge.departure_time + edge.weight,
                transfer_time + wait_time,
                visited,
                cache,
                best_solution
            );

            max_transfer_time = max_transfer_time.max(result);
        }
    }

    visited.remove(&start);
    cache.insert((start, time, transfer_time), max_transfer_time);

    if max_transfer_time > *best_solution {
        *best_solution = max_transfer_time;
    }

    max_transfer_time
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (b, l) = input.line().pair::<u32>();
    const MERCATOR: u32 = 0;
    let home: u32 = l - 1;
    let mut graph = Graph::new(l as usize);

    for _ in 0..b {
        let locations =
            input
                .line()
                .collect::<Vertex>();

        let departures = input
            .line()
            .collect::<u32>();

        locations
            .windows(2)
            .zip(departures.windows(2))
            .for_each(|(vertexes, weights)| {
                graph.add_edge(vertexes[0], vertexes[1], weights[1] - weights[0], weights[0]);
            });
    }

    //println!("{graph}");
    let mut x = u32::MAX;
    println!("{}", dfs(&graph, MERCATOR, home, 0, 0, &mut HashSet::new(), &mut HashMap::new(), &mut x));
    eprintln!("{b}, {l}");
}