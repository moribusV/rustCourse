Repository cleaned
# rust
# Application Description:
Client-server application allows multiple clients to connect to the server. An arbitrary client sends a request to the server, and the server broadcasts corresponding data to all connected participants.

# Usage

Running the Server
cargo run --bin server -- --address <ADDRESS:PORT>

Running the Client
cargo run --bin client -- --address <SERVER_ADDRESS:PORT>

Command-Line Arguments
Server
--address <ADDRESS:PORT>: Specifies the address and port for the server to bind. Defaults to 127.0.0.1:11111.
Client
--address <ADDRESS:PORT>: Specifies the address and port of the server to connect to. Defaults to 127.0.0.1:11111.


# Message Types
Text: Send a text message to all clients.
File: Request the server to send a file to all clients.
Image: Request the server to send an image to all clients.
Quit: Disconnect the client from the server.

# Commands
.text <message>: Send a text message to the server.
.file <path>: Request a file from the server by specifying its path on the server. The file received by the client will be stored in ./client_db/files dir.
.image <path>: Request an image from the server by specifying its path on the server. The image received by the client will be converted to .png and stored in ./client_db/images dir.
.quit: Disconnect from the server.

# Command Examples

.text Hello, World!
.file /path/to/file.txt
.image /path/to/image.png
.quit


