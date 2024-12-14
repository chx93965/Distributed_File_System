# Distributed File System in Rust

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

![Distributed File System](./imgs/dfs_arch.png "Google File System Architecture")

## Architecture

## Rest API
The DFS provides a REST API for clients to interact with the system. The API allows clients to perform various operations such as uploading, downloading, deleting, and listing files. The API is implemented using the Rocket framework, which is a lightweight, high-performance web framework for Rust.

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
