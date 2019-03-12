use std::env;
use std::collections::{HashMap, HashSet};
use std::io::{Error as IoError, Write};

type Node = char;
type Adjacency = HashMap<Node, HashSet<Node>>;

#[derive(Default)]
struct Graph {
    nodes: HashSet<char>,
    edges: Vec<(char, char)>,
}

// Remove non-alphabetic characters and normalize to uppercase.
fn preprocess_string(s: &str) -> String {
    let filtered = s.chars().filter(|c| c.is_alphabetic()).collect::<String>();
    filtered.to_uppercase()
}

// Build the Adjacency representation: each letter is adjacent to the letter before and after.
fn build_adjacency(s: &str) -> Adjacency {
    let mut a = Adjacency::default();
    let chars: Vec<char> = s.chars().collect();
    for (prev, cur) in chars[..chars.len()].iter().zip(chars[1..].iter()) {
        if prev == cur {
            continue
        }
        let m = a.entry(*cur).or_insert_with(HashSet::new);
        m.insert(*prev);
        let m = a.entry(*prev).or_insert_with(HashSet::new);
        m.insert(*cur);
    }
    a
}

// Build a graph from Adjacency. It's helpful to realize that each edge is
// represented twice in the Adjacency; once in each order.
fn build_graph_from_adjacency(a: &Adjacency) -> Graph {
    let mut g = Graph::default();
    for (n, connected) in a.iter() {
        g.nodes.insert(*n);
        for c in connected.iter() {
            if !g.edges.contains(&(*c, *n)) {
                g.edges.push((*n, *c));
            }
        }
    }
    g
}

fn write_graph<W: Write>(g: &Graph, f: &mut W) -> Result<(), IoError> {
    write!(f, "strict graph {{\n")?;
    // Nodes appear unnecessary for GraphViz
    // for n in g.nodes.iter() {
    //     write!(f, "  {}[label=\"{}\"];\n", n, n)?;
    // }
    for (h, t) in g.edges.iter() {
        write!(f, "  {} -- {};\n", h, t)?;
    }
    writeln!(f, "}}")?;
    Ok(())
}

fn main() {
    let s: String = env::args().nth(1).expect("Expected string");
    let ps = preprocess_string(&s);
    let a = build_adjacency(&ps);
    println!("{:?}", a);
    let g = build_graph_from_adjacency(&a);

    use std::io::stdout;
    write_graph(&g, &mut stdout()).expect("I/O error");
    println!("Print with GraphViz or paste into http://www.webgraphviz.com/");
    // use std::fs::File;
    // let mut f = File::create("example1.dot").unwrap();
    // write_graph(&g, &mut f).expect("I/O error");
}
