fn main() {
trait Graph {
type N;
type E;
fn has_edge(&self,n1:&Self::N,n2:&Self::N) -> bool;
fn edges(&self, n:&Self::N) -> Vec<Self::E>;
}
struct Node;
struct Edge;
struct MyGraph;
impl Graph for MyGraph {
type N = Node;
type E = Edge;
fn has_edge(&self,n1:&Node,n2:&Node) -> bool {
true
}
fn edges(&self,n:&Node) -> Vec<Edge> {
Vec::new()
}
}
let graph = MyGraph;
let _obj = Box::new(graph) as Box<dyn Graph<N=Node,E=Edge>>;
   
}
