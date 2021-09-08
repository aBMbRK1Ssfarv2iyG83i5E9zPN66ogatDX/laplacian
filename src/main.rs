fn main() {
    /* Original star graph given in the homework problem, feel free to
     * modify. My code does not check that the graph is a correct
     * (square) adjacency matrix but it should work on any size
     * graph */

    let my_star: Vec<Vec<i16>> = vec![
        vec![0, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0],
    ];

    println!("Original graph:");
    print_graph(&my_star);

    println!("graph Laplacian:");
    // L = D - A
    print_graph(&matrix_subtract(&compute_d(&my_star), &my_star));
}

// print an adjacency matrix to standard output
fn print_graph(graph: &Vec<Vec<i16>>) {
    for row in graph {
        println!("{:?}", &row);
    }
}

// compute matrix D for a given graph, which is the NxN identity matrix times the node degrees for each node
fn compute_d(graph: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    let identity_matrix: Vec<Vec<i16>> = identity_matrix(graph.len() as u16);

    // sum edges into one value k for each node
    let node_degrees: Vec<i16> = graph
        .iter()
        .map(|row: &Vec<i16>| row.iter().sum::<i16>())
        .collect();

    return identity_matrix
        .iter()
        .map(|row: &Vec<i16>| vec_multiply(&row, &node_degrees)) // multiply diagonal 1's times node degrees
        .collect();
}

// compute the identity matrix of a given size; all zeroes with ones along the diagonal where i=j
fn identity_matrix(size: u16) -> Vec<Vec<i16>> {
    let mut matrix = Vec::new();

    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            if j == i {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        matrix.push(row);
    }
    return matrix;
}

// multiply two vectors of integers
fn vec_multiply(vec1: &Vec<i16>, vec2: &Vec<i16>) -> Vec<i16> {
    let mut product = Vec::new();

    for i in 0..vec1.len() {
        product.push(vec1[i] * vec2[i]);
    }
    return product;
}

// subtract two adjacency matrices (graph1 - graph2)
fn matrix_subtract(graph1: &Vec<Vec<i16>>, graph2: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    let mut result_matrix = Vec::new();
    let size = graph1.len();

    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push(graph1[i][j] - graph2[i][j]);
        }
        result_matrix.push(row);
    }
    return result_matrix;
}
