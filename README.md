# Distributed File System in Rust
## Project Members
| Name                     | Student Number | Email Address                 |
|--------------------------|----------------|-------------------------------|
| Swapnil Patel            | 99728870       | Swap.patel@mail.utoronto.ca   |
| Hanxiao Chang            | [Insert Here]  | [Insert Here]                 |
| Mohammad Hooman Keshvari | 1011293869  | Hooman.keshvari@mail.utoronto.ca                 |

### Contribution 
- Swapnil Patel: Chunk server, cluster management, documentation
- Hanxiao Chang: 
- Mohammad Hooman Keshvari: Master Node

## Table of Contents
- [Proposals](./PROPOSAL.md)
- [Introduction](#introduction)
- [Architecture](#architecture)
- [Rest API](#rest-api)
- [How-to use](#how-to-use)
- [References](#references)

## Introduction
A **Distributed File System (DFS)** is a system that allows multiple computers to share a common file system, making data accessible and manageable across a network of interconnected machines. It provides a way to store, access, and manage files across various servers or nodes in a distributed manner. The main features of a DFS include:

1\. **Centralized Management**: Despite data being distributed, the DFS offers a unified view of files, allowing users and applications to interact with them as if they were on a local machine.

2\. **Scalability**: It can scale easily by adding more servers or nodes, improving performance and fault tolerance.

3\. **Redundancy and Fault Tolerance**: Files can be replicated across multiple servers to enhance reliability and availability. In case of a server failure, other copies can be accessed.

4\. **Efficiency**: It optimizes data storage, access speed, and resource utilization by distributing files across multiple nodes.

DFS is commonly used in cloud storage, big data processing, and content delivery networks, where access to large volumes of data from multiple locations is essential.

## Architecture
![Distributed File System](./imgs/dfs_arch.png "Google File System Architecture")

Our DFS is inspired by the Google File System (GFS) architecture, which is a distributed file system designed to store and manage large volumes of data across multiple servers. The GFS architecture consists of three main components:

1\. **Master Server**: The master server is responsible for managing the metadata of the file system, such as file locations, access permissions, and replication policies. It keeps track of the file system's state and coordinates operations across multiple servers.

2\. **Chunk Servers**: The chunk servers store the actual data in the form of fixed-size chunks. Each chunk is replicated across multiple chunk servers to ensure data availability and reliability. The chunk servers are responsible for storing, replicating, and serving data to clients. *note: Chunk replication is still in progress*

3\. **Client**: The client interacts with the master server to perform file operations such as reading, writing, and deleting files. The client communicates with the chunk servers to read and write data chunks.

The core features of the DFS include:

1\. **File Operations**: The DFS supports basic file operations such as creating, reading, writing, and deleting files. Clients can interact with the file system using a REST API.

## Rest API
The DFS provides a REST API for clients to interact with the system. The API allows clients to perform various operations such as uploading, downloading, deleting, and listing files. The API is implemented using the Rocket framework, which is a lightweight, high-performance web framework for Rust.

### Master

### Client

---
### Chunk Server
#### Method: `add_chunk`
- **Description**: Adds a chunk to the chunk manager. This endpoint expects a POST request with binary data as the body of the request and allows specifying a UUID to associate with the chunk.
- **Parameters:**
  - `chunk_id`: The UUID of the chunk to be added.
  - `data`: The binary data of the chunk.
- **Example Request:**
    -   ```bash
        curl -X POST "http://127.0.0.1:8100/add_chunk?id=<UUID>" \
            -H "Content-Type: application/octet-stream" \
            --data-binary @example.bin
        ```
- **Error Responses:**
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **413 Payload Too Large**: If the chunk size exceeds the maximum allowed size.
---
#### Method: `get_chunk`
- **Description**: Retrieves a chunk from the chunk manager. This endpoint expects a GET request with the UUID of the chunk to be retrieved.
- **Parameters:**
  - `chunk_id`: The UUID of the chunk to be retrieved.
- **Example Request:**
    -   ```bash
        curl -X GET "http://127.0.0.1:8100/get_chunk?id=<UUID>" --output chunk_output.bin
        ```
- **Error Responses:**
    - **400 Bad Request**: If the request is malformed or missing required parameters.
    - **404 Not Found**: If the chunk with the specified UUID does not exist.
---
#### Method: `get_chunk_list`
- **Description**: Retrieves a list of all chunks stored in the chunk manager. This endpoint expects a GET request without any parameters.
- **Example Request:**
    -   ```bash
        curl -X GET "http://127.0.0.1:8100/get_chunk_list"
        ```
- **Success Response:**
    - **Code**: 200
    - **Content**: A JSON array containing the UUIDs of all chunks stored in the chunk manager.
    - **Example Content**:
        ```json
        ["UUID1", "UUID2", "UUID3"]
        ```
- **Error Responses:**
    - **400 Bad Request**: If the request is malformed or contains invalid parameters.
---
#### Method: `delete_chunk`
- **Description**: Deletes a chunk from the chunk manager. This endpoint expects a DELETE request with the UUID of the chunk to be deleted.
- **Parameters:**
  - `chunk_id`: The UUID of the chunk to be deleted.
- **Example Request:**
    -   ```bash
        curl -X GET "http://127.0.0.1:8100/delete_chunk?id=<UUID>"
        ```
- **Error Responses:**
    - **400 Bad Request**: If the request is malformed or missing required parameters.
    - **404 Not Found**: If the chunk with the specified UUID does not exist.
---
## How-to use
### Build the project using "release" configuration
```bash 
cargo build --release 
```
### Launch the cluster from the root directory
```bash
launch_dfs.sh <number_of_nodes>
```
### Client Operations
Once the cluster is up and running, you can interact with the DFS using the client application. The client application provides a command-line interface to perform various operations on the DFS. You can run the client application using the following command:
#### Create a file
```bash
./target/release/client --target file --action create --local_path <local_path> --remote_path <remote_path>
```
#### Read a file
```bash
./target/release/client --target file --action read --local_path <local_path> --remote_path <remote_path>
```

 ## References
 Ghemawat, S., Gobioff, H., & Leung, S.T. (2003). The Google file system. In Proceedings of the Nineteenth ACM Symposium on Operating Systems Principles (pp. 29–43). Association for Computing Machinery.
