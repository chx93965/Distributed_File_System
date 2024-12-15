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
#### Method: `user_authenticate`
- **Description**: Authenticates a user with the DFS. If the user does not already exist, they are registered automatically. This function interacts with two endpoints: `/user/register` and `/user/login`.
- **Parameters:**
  - `user` : A reference to a User struct containing the user's credentials.
- **Example Request**:
    - ```bash
        curl -X POST "http://<base_url>/user/register" \
            -H "Content-Type: application/json" \
            -d '{"username": "example_user", "password": "example_password"}'
        ```
    - ```bash
        curl -X POST "http://<base_url>/user/login" \
            -H "Content-Type: application/json" \
            -d '{"username": "example_user", "password": "example_password"}'
        ```
- **Error Responses**:
    - **400 Bad Request**: If the request is malformed or missing required parameters.
    - **401 Unauthorized**: If the user credentials are invalid.

---
#### Method: `create_file`
- **Description**: Creates a new file in the DFS at the specified remote path. This function sends a POST request to the server with the file path as a query parameter.

- **Parameters**:
  - `path`: A string slice representing the remote path where the new file should be created.

- **Example Request**:
    ```bash
    curl -X POST "http://<base_url>/file/create?path=/example/path"
    ```

- **Success Response**:
  - **Code**: 200
  - **Content**: A JSON object containing the metadata of the newly created file.
  - **Example Content**:
    ```json
    {
        "file_id": "1234-5678",
        "path": "/example/path",
        "created_at": "2024-12-15T12:34:56Z"
    }
    ```

- **Error Responses**:
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **409 Conflict**: If a file already exists at the specified path.
  - **500 Internal Server Error**: If an unexpected server error occurs during file creation.

---
#### Method: `read_file`
- **Description**: Reads the contents of a file from the DFS at the specified remote path. This function sends a GET request to the server to fetch the file's data as a list of `ChunkInfo`.

- **Parameters**:
  - `path`: A string slice representing the remote path of the file to be read.

- **Example Request**:
    ```bash
    curl -X GET "http://<base_url>/file/read?path=/example/path"
    ```

- **Success Response**:
  - **Code**: 200
  - **Content**: A JSON array containing metadata for each chunk of the file.
  - **Example Content**:
    ```json
    [
        {
            "chunk_id": "1234-5678",
            "offset": 0,
            "size": 1024
        },
        {
            "chunk_id": "5678-1234",
            "offset": 1024,
            "size": 2048
        }
    ]
    ```

- **Error Responses**:
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **404 Not Found**: If the file at the specified path does not exist.
  - **500 Internal Server Error**: If an unexpected server error occurs during file reading.

---
#### Method: `update_file`
- **Description**: Updates an existing file in the DFS at the specified remote path with a new size. This function sends a POST request to the server to update the file's metadata.

- **Parameters**:
  - `path`: A string slice representing the remote path of the file to be updated.
  - `size`: The new size for the file in bytes.

- **Example Request**:
    ```bash
    curl -X POST "http://<base_url>/file/update?path=/example/path&size=2048"
    ```

- **Success Response**:
  - **Code**: 200
  - **Content**: A JSON array containing the metadata for each updated chunk of the file.
  - **Example Content**:
    ```json
    [
        {
            "chunk_id": "1234-5678",
            "offset": 0,
            "size": 1024
        },
        {
            "chunk_id": "5678-1234",
            "offset": 1024,
            "size": 2048
        }
    ]
    ```

- **Error Responses**:
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **404 Not Found**: If the file at the specified path does not exist.
  - **500 Internal Server Error**: If an unexpected server error occurs during file update.

---
#### Method: `create_directory`
- **Description**: Creates a new directory in the DFS at the specified remote path. This function sends a POST request to the server to create a directory at the given path.

- **Parameters**:
  - `path`: A string slice representing the remote path where the new directory should be created.

- **Example Request**:
    ```bash
    curl -X POST "http://<base_url>/dir/create?path=/example/directory"
    ```

- **Success Response**:
  - **Code**: 200
  - **Content**: A string indicating the successful creation of the directory.
  - **Example Content**:
    ```json
    "Directory /example/directory created successfully."
    ```

- **Error Responses**:
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **409 Conflict**: If a directory already exists at the specified path.
  - **500 Internal Server Error**: If an unexpected server error occurs during directory creation.

---
#### Method: `read_directory`
- **Description**: Reads the contents of a directory in the DFS at the specified remote path. This function sends a GET request to the server to fetch information about the directory, including its contents.

- **Parameters**:
  - `path`: A string slice representing the remote path of the directory to be read.

- **Example Request**:
    ```bash
    curl -X GET "http://<base_url>/dir/read?path=/example/directory"
    ```

- **Success Response**:
  - **Code**: 200
  - **Content**: A JSON string containing the metadata of the directory and its contents.
  - **Example Content**:
    ```json
    {
        "path": "/example/directory",
        "contents": [
            {
                "name": "file1.txt",
                "size": 1024,
                "type": "file"
            },
            {
                "name": "subdir1",
                "type": "directory"
            }
        ]
    }
    ```

- **Error Responses**:
  - **400 Bad Request**: If the request is malformed or missing required parameters.
  - **404 Not Found**: If the directory at the specified path does not exist.
  - **500 Internal Server Error**: If an unexpected server error occurs during directory reading.
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
