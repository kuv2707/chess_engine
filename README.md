# Chess Engine API

This project provides some utility tools for Chess, and is designed
for learning purposes. It includes the following features:

### Stockfish Support

Stockfish is a powerful open-source chess engine that is widely
regarded as one of the strongest chess engines in the world. Check
out the [Stockfish website](https://stockfishchess.org/) for more
information.

The project incorporates a wrapper around the Stockfish CLI,
allowing users to leverage its powerful chess-playing capabilities.
It spawns a Stockfish process and communicates with it via standard
input and output.

We can use the Stockfish engine to find all legal moves given a
position, calculate the best move for a given position, and more. We
can also configure the skill level of Stockfish.

### Custom Chess Engine

In addition to Stockfish, the project features a basic custom chess
engine that offers its own unique chess-playing capabilities.

The engine uses the **minimax algorithm** with **alpha-beta
pruning** to determine the best move in a given position. The engine
evaluates the board position based on the following factors:

-   Material balance
-   Positional advantage

It looks up to the depth of 3 moves ahead and uses a simple
evaluation function to determine the best move.

### Self-Made HTTP Server

The project includes a custom-built HTTP server implementation,
enabling users to access and interact with the chess engines through
a web interface. The server is single-threaded and can handle one
request at a time. It parses HTTP requests and responds with the
appropriate data, as defined by the handlers. Checkout the
[repository](https://github.com/kuv2707/repress-rs)

## How to run

-   Running `cargo run` will start the server on `localhost:4000`.
-   If using the stockfish engine, make sure the stockfish binary is
    present in the root directory. The repository includes Stockfish
    15.1 executables for Windows and Linux. For other versions:
    -   Download the Stockfish binary from the
        [Stockfish website](https://stockfishchess.org/download/).
    -   Rename the binary to `stockfish` and place it in the root
        directory of the project.

## Tech stack

Rust, Rust and Rust. The project is written entirely in Rust, and
entirely by myself with minimal use of libraries, except the
stockfish engine binary of course. This is in an attempt to learn
things at a lower level.

## Future Plans

The future plans for this Chess Engine API project include:

### Integration with a Node.js Chess Backend and Frontend

The aim is to connect the Chess Engine API with a Node.js-based
chess backend and frontend, creating a full-stack chess application.

### Improved Chess Engine

The engine could never compete with Stockfish in terms of strength,
but it would be a fun project and a great learning experience to
optimize the engine further. This could involve improving the
ordering of moves for alpha-beta pruning, enhancing the evaluation
function, etc.
