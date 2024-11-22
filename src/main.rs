use std::collections::{HashMap, VecDeque};
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
    to: Vertex,           // Destination location
    weight: i32,          // Negative weight for maximum path
    departure_time: u32,  // Departure time of the destination bus line
}

struct Graph {
    adjacency: Vec<Vec<Edge>>,  // Adjacency list for the graph
    in_degree: Vec<u32>,        // Track in-degree for topological sorting
}

impl Graph {
    fn new(n: usize) -> Graph {
        Graph {
            adjacency: vec![vec![]; 10000],
            in_degree: vec![0; 10000],
        }
    }

    fn add_edge(&mut self, from: Vertex, to: Vertex, weight: i32, departure_time: u32) {
        self.adjacency[from as usize].push(Edge { to, weight, departure_time });
        self.in_degree[to as usize] += 1;
    }

    fn topological_sort(&mut self) -> Vec<Vertex> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Collect nodes with no incoming edges (in-degree 0)
        for (i, &in_deg) in self.in_degree.iter().enumerate() {
            if in_deg == 0 {
                queue.push_back(i as u32);
            }
        }

        // Process nodes in topological order
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            for edge in &self.adjacency[vertex as usize] {
                let to = edge.to as usize;
                self.in_degree[to] -= 1;
                if self.in_degree[to] == 0 {
                    queue.push_back(edge.to);
                }
            }
        }

        result
    }

    // DAG Shortest Path with Negative Weights (using Topological Sort)
    fn dag_shortest_path(&mut self, start: Vertex) -> Vec<i32> {
        let n = self.adjacency.len();
        let mut dist = vec![i32::MIN; n]; // Initialize distances to negative infinity
        dist[start as usize] = 0; // Distance to start is 0

        let topological_order = self.topological_sort();

        // Relax edges in topological order
        for vertex in topological_order {
            for edge in &self.adjacency[vertex as usize] {
                let new_dist = dist[vertex as usize] + edge.weight;
                if new_dist > dist[edge.to as usize] {
                    dist[edge.to as usize] = new_dist;
                }
            }
        }

        dist
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (vertex, edges) in self.adjacency.iter().enumerate() {
            writeln!(f, "Vertex {}: ", vertex)?;  // Display each vertex
            for edge in edges {
                writeln!(
                    f,
                    "  -> to: {}, weight: {}, departure time: {}",
                    edge.to, edge.weight, edge.departure_time
                )?; // Show edges with departure time
            }
        }
        Ok(())
    }
}

fn main() {
    let input = stdin();
    let mut input = Input::new(BufReader::new(input.lock()));

    let (b, l) = input.line().pair::<u32>();
    const MERCATOR: u32 = 0;
    let home: u32 = l - 1; // Home location is always the last one (l-1)
    let mut graph = Graph::new(l as usize);

    // We will store the buses arriving at each location
    let mut location_buses: HashMap<u32, Vec<(u32, u32)>> = HashMap::new(); // Location -> (bus_line, departure_time)

    // Process the bus lines
    for bus_line in 0..b {
        let locations = input
            .line()
            .collect::<Vertex>();
        let departures = input
            .line()
            .collect::<u32>();

        for (i, &location) in locations.iter().enumerate() {
            let departure_time = departures[i];
            location_buses
                .entry(location)
                .or_insert_with(Vec::new)
                .push((bus_line, departure_time));
        }
    }

    // Now, process the transfers between bus lines at each location
    for (&location, buses) in location_buses.iter() {
        // Sort buses at this location by departure time to ensure correct transfer times
        let mut sorted_buses = buses.clone();
        sorted_buses.sort_by_key(|&(_, time)| time);

        // Loop through each pair of buses at this location and check if they belong to different bus lines
        for i in 0..sorted_buses.len() {
            for j in i + 1..sorted_buses.len() {
                let (bus_a, time_a) = sorted_buses[i];
                let (bus_b, time_b) = sorted_buses[j];

                // Only calculate transfer time between different bus lines
                if bus_a != bus_b && time_a < time_b {
                    // Calculate the transfer time between the two bus lines
                    let transfer_time = time_b - time_a; // Waiting time between buses

                    // Add edge between the locations of the different bus lines
                    // The edge represents a transfer between two different bus lines at the same location
                    // We should only add the edge between different locations, not to the same location
                    graph.add_edge(location, location + 1, -(transfer_time as i32), time_b); // Example connection
                }
            }
        }
    }

    // Print the graph to verify that edges are being added correctly
    println!("Graph after adding edges:");
    println!("{graph}");

    // Perform DAG Shortest Path (Maximum Path)
    let dist = graph.dag_shortest_path(0); // Start from vertex 0 (can be adjusted)

    println!("Maximum transfer times from start: {:?}", dist);

    // Check the maximum transfer time to home (l-1)
    let home_time = dist[home as usize];
    if home_time == i32::MIN {
        println!("No path found to home.");
    } else {
        println!("Maximum transfer time to home ({}): {}", home, home_time);
    }
}
