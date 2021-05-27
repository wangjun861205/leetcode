use std::collections::HashSet;

fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut nodes: HashSet<i32> = HashSet::new();
    nodes.insert(0);
    let mut count = 0;
    while nodes.len() < n as usize {
        for conn in connections.iter() {
            if conn[0] == 0 && nodes.contains(&conn[1]) {
                nodes.insert(conn[1]);
                count += 1;
            } else if conn[1] == 0 && nodes.contains(&conn[0]) {
                nodes.insert(conn[0]);
            } else if nodes.contains(&conn[0]) && !nodes.contains(&conn[1]) {
                count += 1;
                nodes.insert(conn[1]);
            } else if nodes.contains(&conn[1]) && !nodes.contains(&conn[0]) {
                nodes.insert(conn[0]);
            }
        }
    }
    count
}

fn main() {
    let count = min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]);
    println!("{}", count);
}
