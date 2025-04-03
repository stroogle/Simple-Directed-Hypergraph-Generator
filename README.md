
#  Simple Directed Hypergraph Generator 🌐🔗

Welcome to the repository, this is a basic command-line tool that generates simple, random directed hypergraphs based on specific parameters.

## Features ✨
- Generate **random directed hypergraphs** with customizable parameters.
- Simple, easy-to-use command-line interface.
- Generate hypergraphs with a specific number of **nodes** and **hyperarcs**.
- Output as a bipartite graph representation.

## Installation 🛠️

To install the tool, follow these steps:

### Using cargo (if available)
```bash
cargo install --git https://github.com/stroogle/Simple-Directed-Hypergraph-Generator.git
```

## Usage 🏃‍♂️

Once installed, you can generate a random directed hypergraph by running the command:

```bash
graph_gen --left-column-size <LEFT_COLUMN_SIZE> --right-column-size <RIGHT_COLUMN_SIZE> --generate-n-graphs <GENERATE_N_GRAPHS> --edge-chance <EDGE_CHANCE>
```

### Command-line options:
- `-l, --left-column-size <LEFT_COLUMN_SIZE>`    
- `-r, --right-column-size <RIGHT_COLUMN_SIZE>` 
- `-g, --generate-n-graphs <GENERATE_N_GRAPHS>`  
- `-e, --edge-chance <EDGE_CHANCE>`  

Example:
```bash
graph_gen -l 10 -r 3 -e 50 -g 1
```

This will generate a hypergraph with 10 nodes, 3 hyperarcs, and ~50% of each node appearing in any tail/head set.

## Example Output 📊

Here’s an example of the generated output

```
[[[0,1,0],[0,0,0],[0,0,1],[1,1,0],[0,0,0],[1,0,1],[1,1,0],[0,0,1],[1,0,0],[0,0,0]],[[1,1,0,0,0,0,0,0,0,1],[0,1,1,0,0,1,0,1,0,0],[0,1,0,0,0,0,1,0,1,1]]]
```

## License 📜

This tool is licensed under the MIT License. 