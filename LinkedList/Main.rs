extern mod LinkedList = "github.com/veblush/Test/tree/master/LinkedList#LinkedList:0.1";

use LinkedList::{List, print};

fn main() {
    let l: List<f32> = List::new();
    let m = l.push(1.0).push(2.0).push(3.0).pop();
    print(&m);
}
