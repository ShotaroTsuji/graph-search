use std::marker::PhantomData;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub trait AdjacentVertices {
    type Vertex;

    fn adjcent_vertices(&self, v: Self::Vertex) -> Vec<Self::Vertex>;

    fn dfs<'a>(&'a self, start: Self::Vertex) -> DfsIter<'a, Self>
        where
            Self: Sized,
            Self::Vertex: Eq + Hash + Clone,
    {
        DfsIter::new(self, start)
    }

    fn bfs<'a>(&'a self, start: Self::Vertex) -> BfsIter<'a, Self>
        where
            Self: Sized,
            Self::Vertex: Eq + Hash + Clone,
    {
        BfsIter::new(self, start)
    }
}

#[derive(Debug,Clone)]
pub struct AdjVec(Vec<Vec<usize>>);

fn validate_adj_vec(vec: &[Vec<usize>]) -> bool {
    let max_index = vec.len();
    vec.iter()
        .all(|adj_list| {
            adj_list.iter()
                .all(|v| *v <= max_index)
        })
}

impl AdjVec {
    pub fn from_vec(vec: Vec<Vec<usize>>) -> Option<Self> {
        if validate_adj_vec(&vec) {
            Some(Self(vec))
        } else {
            None
        }
    }

    pub fn is_undirected(&self) -> bool {
        for (index, adj_list) in self.0.iter().enumerate() {
            for vertex in adj_list.iter().cloned() {
                if !self.0[vertex].contains(&index) {
                    eprintln!("Edge[{a} -> {b}] was found, but Edge[{b} -> {a}] was not found",
                        a=index, b=vertex);
                    return false;
                }
            }
        }
        true
    }
}

impl AdjacentVertices for AdjVec {
    type Vertex = usize;

    fn adjcent_vertices(&self, v: usize) -> Vec<usize> {
        self.0.get(v).cloned()
            .unwrap_or(Vec::new())
    }
}

pub trait VertexContainer {
    type Vertex;

    fn new() -> Self;
    fn insert(&mut self, v: Self::Vertex);
    fn take(&mut self) -> Option<Self::Vertex>;
}

pub struct VertexStack<V>(Vec<V>);

impl<V> VertexContainer for VertexStack<V> {
    type Vertex = V;

    fn new() -> Self {
        VertexStack(Vec::new())
    }

    fn insert(&mut self, v: V) {
        self.0.push(v);
    }

    fn take(&mut self) -> Option<V> {
        self.0.pop()
    }
}

pub struct VertexQueue<V>(VecDeque<V>);

impl<V> VertexContainer for VertexQueue<V> {
    type Vertex = V;

    fn new() -> Self {
        VertexQueue(VecDeque::new())
    }

    fn insert(&mut self, v: V) {
        self.0.push_back(v);
    }

    fn take(&mut self) -> Option<V> {
        self.0.pop_front()
    }
}

pub type DfsIter<'a, G> = GraphSearchIter<'a, G, VertexStack<<G as AdjacentVertices>::Vertex>>;
pub type BfsIter<'a, G> = GraphSearchIter<'a, G, VertexQueue<<G as AdjacentVertices>::Vertex>>;

pub struct GraphSearchIter<'a, G: AdjacentVertices, C> {
    graph: &'a G,
    will_visit: C,
    visited: HashSet<<G as AdjacentVertices>::Vertex>,
    _phantom: PhantomData<&'a G>,
}

impl<'a, G, C> GraphSearchIter<'a, G, C>
where
    G: AdjacentVertices,
    <G as AdjacentVertices>::Vertex: Eq + Hash + Clone,
    C: VertexContainer<Vertex=<G as AdjacentVertices>::Vertex>,
{
    pub fn new(graph: &'a G, start: <G as AdjacentVertices>::Vertex) -> GraphSearchIter<'a, G, C> {
        let mut will_visit = C::new();
        let mut visited = HashSet::new();
        visited.insert(start.clone());
        will_visit.insert(start);
        GraphSearchIter {
            graph: graph,
            will_visit: will_visit,
            visited: visited,
            _phantom: PhantomData,
        }
    }
}

impl<'a, G, C> Iterator for GraphSearchIter<'a, G, C>
where
    G: AdjacentVertices,
    <G as AdjacentVertices>::Vertex: Eq + Hash + Clone,
    C: VertexContainer<Vertex=<G as AdjacentVertices>::Vertex>,
{
    type Item = <G as AdjacentVertices>::Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vertex) = self.will_visit.take() {
            let adjcent_vertices = self.graph.adjcent_vertices(vertex.clone());

            for n in adjcent_vertices.into_iter() {
                if !self.visited.contains(&n) {
                    self.will_visit.insert(n.clone());
                    self.visited.insert(n);
                }
            }

            Some(vertex)
        } else {
            None
        }
    }
}
