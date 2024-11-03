

# Distributed File System in Rust  
  
## Motivation  
The project aims to develop a distributed file storage system that provides reliable, efficient, and secure services to various applications. The idea is inspired by the heavy reliance of internet industry corporations on large-scale data centers, in which high-availability systems are needed to minimize impact on real-world operations due to possible errors or failures. Though not widely applied, data-driven applications benefit from this system’s inherent advantages over traditional databases and its alignment with the evolving trend of big data and AI technologies.  
  
### Reliability  
- A major advantage of a distributed file storage system is its high reliability. By replicating data across multiple nodes, the system ensures the quality of service even if some individual nodes fail. Data replicas in redundant servers enable quick recovery from system faults, which is important especially for mission-critical applications.  
- Data redundancy also allows for load balancing as data can be redistributed across nodes to handle increasing demands without compromising its availability.  
  
### Flexibility  
- The “master-slave” structure ensures simple node management, allowing nodes to be added or removed with minimal time and storage consumption. This structure significantly enhances the database scalability especially when storing large files or continuous data streams, as it can expand horizontally (within the same level) to append more storage without altering its underlying architecture.  
- Additionally, the system is adaptable to diverse data types such as structured, semi-structured, and unstructured data, and is capable of supporting a wide range of applications.  
  
### Efficiency  
- Fast data IO operations are essential for applications with strict timing requirements or resource consumption constraints. A distributed file storage system can improve access speed by serving data from the closest available location, which reduces latencies and potentially balances the workload across the network to avoid bottlenecks in data transmissions.  
  
### Security  
- Distributing data across multiple nodes effectively reduces the risk of data breaches as the probability of simultaneous attacks on multiple nodes is usually much lower.  
- More advanced implementations can secure each node through independently implemented encryption and access control which further protect data from both internal and external threats and ultimately enhance overall system reliability.  
 
In addition to these intrinsic features, a distributed file storage system also conforms with modern technology trends.
### Big Data and Machine Learning  
- With the explosion of data, distributed storage architectures are well-suited to the demands of analyzing large volumes of information. As popular data analysis algorithms perform better with sufficiently large datasets, traditional centralized storage lacks flexibility without appropriate data partitioning and processing. In contrast, the architecture of decentralized storage is in accordance with big data analysis techniques, providing a good source for future research.  
- Besides, machine learning can be employed to enhance the system’s fault tolerance and data recovery by analyzing file patterns and contents to restore lost or corrupted data. This capability makes the system even more resilient especially in scientific research. Additionally, abnormal patterns or data inconsistencies can be identified to improve the security and reliability of the system.  
  
### Importance in Rust  
Popular open-source distributed file storage systems like Hadoop Distributed File System (HDFS) or Google File System are often implemented using Java or Golang. However, Rust offers better performance without the need for in-memory garbage collection and is known for its reliability, efficiency, and memory safety, under heavy loads and in complex environments. Despite Rust’s remarkable growth in systems programming, it currently lacks a production-grade distributed file system that can handle modern data processing demands. Also, Rust's ownership model and thread safety guarantees are particularly valuable for managing the complex concurrent operations inherent in distributed file systems, potentially eliminating entire classes of bugs that traditionally plague such systems. Furthermore, Rust's growing ecosystem of async runtime support makes it well-suited for handling the numerous network operations and concurrent tasks essential to GFS's architecture.  
  
While a distributed file storage system offers significant benefits, it also requires careful management of data consistency and increased development complexity. Despite these challenges, distributed file storage systems are still optimal for specific types of applications.  

## Objective
The primary objective of this project is to design and implement a distributed file storage system that leverages Rust's performance and safety features to provide a reliable, efficient, and secure solution for storing and managing large volumes of data across multiple nodes. This system aims to address the growing need for scalable and resilient storage solutions in the era of big data and AI technologies, while offering a unique approach that emphasizes Rust's strengths in performance and memory safety. We aim to achieve this by closely following the design of [Google File System](https://research.google.com/archive/gfs-sosp2003.pdf) and implementing it in Rust.
## Key Features

```
*** must have
**  should have
*   nice to have
```

### Scalable Architecture  
  
Our distributed file storage system is built on a robust and scalable architecture that combines the simplicity of centralized management with the power of distributed storage. By adopting a single master server design for metadata management while distributing actual data across multiple chunk servers, the system achieves both architectural simplicity and unlimited storage scalability. This hybrid approach enables seamless system growth, efficient resource utilization, and straightforward system management, making it ideal for organizations with rapidly growing storage needs.  
  
- **Centralized Management**: Single master server architecture for simplified metadata management ( *** )  
- **Distributed Storage**: Multiple chunk servers for scalable data storage ( *** )  
- **Dynamic Expansion**: Support for runtime addition of storage nodes without system downtime ( ** )  
- **Load Distribution**: Automatic load balancing across storage nodes to optimize resource utilization ( *** )  
  
### Data Reliability and Redundancy  
  
In today's data-critical environment, maintaining data integrity and availability is paramount. Our system implements a comprehensive approach to data reliability through multiple redundancy mechanisms. At its core, the system employs a replication strategy that ensures data survival even in the face of multiple simultaneous failures. By combining automatic triple replication with intelligent cross-rack distribution and continuous health monitoring, we provide a self-healing storage environment that maintains data integrity without requiring manual intervention.  
  
- **Automatic Replication**: Triple replication of data chunks across different physical servers ( *** )  
- **Intelligent Placement**: Cross-rack distribution of replicas to handle rack failures  
- **Failure Detection**: Continuous health monitoring of storage nodes ( *** )  
- **Self-Healing**: Automatic re-replication of data when node failures or corruptions are detected ( *** )  
- **Data Integrity**: Checksum verification for all stored chunks ( *** )  
  
### High Performance Design  
  
Our high-performance architecture is engineered to maximize throughput while minimizing latency in large-scale storage operations. Through careful optimization of chunk sizes, intelligent data transfer mechanisms, and efficient caching strategies, the system delivers exceptional performance even under heavy load. The design emphasizes reducing network overhead and maximizing data locality, resulting in a storage system that can handle massive data volumes while maintaining responsive access times for all operations.  
  
- **Optimized Chunk Size**: 64MB chunk size to minimize metadata overhead  
  - Could have other sizes as well for medium - small files  
- **Efficient Data Transfer**:  
  - Pipelined data transfers between nodes  
  - Direct client-to-chunkserver data transfer  
- **Caching Strategy**: In-memory caching of metadata in master server  
- **Batch Operations**: Support for batch processing of file operations  
  
### Consistency and Concurrency  
  
Managing concurrent access while maintaining data consistency is a critical challenge in distributed systems. Our solution implements a sophisticated consistency model that guarantees atomic operations while supporting high levels of concurrent access. Through an innovative lease-based management system and optimized append operations, we ensure that multiple clients can simultaneously access and modify data without compromising consistency. This approach provides strong consistency guarantees while maintaining the high-performance characteristics essential for modern distributed applications.  
  
- **Atomic Operations**: Guaranteed atomicity for metadata operations  
- **Lease Management**: Chunk lease mechanism to ensure consistent mutations  
- **Concurrent Access**: Support for multiple clients accessing the system simultaneously ( *** )  
- **Append Optimization**: Efficient handling of concurrent append operations  
- **Version Control**: Basic file versioning support for conflict resolution  
  
### Security and Access Control  
  
- **Authentication**: Client authentication using secure protocols ( *** )  
- **Authorization**: Role-based access control for files and directories  
- **Encryption**: Data encryption during transfer and at rest  
- **Audit Logging**: Comprehensive logging of all system operations  
  
### Monitoring and Management (Stretch Goal)  
  
- **System Dashboard**: Web-based interface for system monitoring  
- **Performance Metrics**:  
  - Real-time storage capacity monitoring  
  - Network bandwidth utilization  
  - System latency measurements  
  - Node health status  
- **Alert System**: Automated notifications for:  
  - Storage capacity thresholds  
  - Node failures  
  - Performance degradation  
  - System errors  
  
### Additional Features (Stretch Goal)  
  
- **Garbage Collection**: Automatic cleanup of deleted and orphaned chunks  
- **Snapshot Support**: Point-in-time snapshots for backup purposes  
- **Data Migration**: Tools for data import/export and migration  
- **Quota Management**: Storage quota enforcement at directory/user level  
- **Maintenance Mode**: Support for graceful system maintenance  

## Tentative Plan
| Week | Date   | Tasks                                                                 |  
|------|--------|-----------------------------------------------------------------------|  
| 1    | Nov. 4 | Node Implementation and Master-slave Architecture                     |  
| 2    | Nov. 11| Data Partitioning and Replication                                     |  
| 3    | Nov. 18| Concurrent Access                                                     |  
| 4    | Nov. 25| Fault Tolerance and Load Balancing                                    |  
| 5    | Dec. 2 | User Authentication                                                   |  
|      |        | System Monitor (Stretch Goal)                                         |  
|      |        | GUI Design (Stretch Goal)                                             |  
| 6    | Dec. 9 | Functional and Stress Testing                                         |  
|      |        | Code Review and Documentation                                         |  
|      |        | System Monitor (Stretch Goal)                                         |  
|      |        | GUI Design (Stretch Goal)                                             |  
### Week 1:   
**Tasks:**  
- **Establish the foundational structure, set up nodes under a master-slave architecture.**  
  - **Hooman**  
    - Implement the master server to manage metadata and control interface for chunk servers  
  - **Swapnil**  
    - Implement the initial chunk server as nodes for distributed data storage  
    - Test communication between the master and chunk servers  
  - **Hanxiao**  
    - Establish node configurations in terms of network settings and storage paths  
    - Define initialization processes for system startup ensuring master and chunk servers are synchronized  
  
### Week 2:   
**Implement a scalable data partitioning and replication system with efficient chunking, metadata management, and fault-tolerant replication strategies**  
  
**Tasks:**  
- **Hooman**  
  - Develop chunk and Metadata Management  
  - Implement an algorithm to efficiently divide files into chunks and distribute it to chunk servers with robust metadata management system  
- **Swapnil**  
  - Design and Implement Replication Strategy  
  - Develop the logic for creating and maintaining replicas of each chunk across multiple nodes.  
- **Hanxiao**  
  - Chunk-server synchronization  
  - Develop protocols for synchronizing replicas across chunk and automatically handle node failures  
  
### Week 3:   
**Implement a robust concurrent access system ensuring data consistency and efficient locking mechanism.**  
  
**Tasks:**  
- **Hooman**  
  - Design Consistency and locking mechanism  
  - Develop consistency model with atomic operations and locking mechanism for concurrent read/write  
- **Swapnil**  
  - Lease Management and Append Optimization  
  - Create lease management system for data modification and optimize append operations to handle concurrent data writes.  
- **Hanxiao**  
  - Develop client coordination and conflict resolution  
  - Protocol for client coordination for shared data access and implement version control for resolving concurrent data modifications.  
  
### Week 4:   
**Tasks:**  
- **Hooman**  
  - Implement master node failover  
  - Develop chunk rebalancing logic  
  - Create failure detection system  
  - Implement load distribution algorithm  
  - Develop system recovery procedures  
- **Swapnil**  
  - Implement chunk recovery mechanisms  
  - Develop corrupt chunk handling  
  - Create chunk migration system  
  - Implement rack awareness  
  - Develop failure recovery tests  
- **Hanxiao**  
  - Implement client-side failure handling  
  - Create automatic failover in client  
  - Develop connection pooling  
  - Implement load-aware client  
  - Create fault tolerance tests  
  
### Week 5:   
**Tasks:**  
- **Hooman**  
  - Implement authentication system  
  - Develop access control lists  
  - Create user management system  
  - Begin system monitoring (if time permits)  
  - Start performance optimization  
- **Swapnil**  
  - Implement security in chunk servers  
  - Develop secure data transfer  
  - Create quota management  
  - Help with monitoring (if time permits)  
  - Optimize chunk operations  
- **Hanxiao**  
  - Implement client authentication  
  - Create secure client sessions  
  - Start GUI development (if time permits)  
  - Develop user management interface  
  - Create security tests  
  
### Week 6:   
**Tasks:**  
- **Review and test the code by work done each week, and document key features as needed**  
- **Finalize system monitoring and GUI design based on the time remaining**  
- **Hooman**  
  - Conduct functional tests on node creation, and data partition  
  - Complete system monitoring of system health and resource usage (if time permits)  
- **Swapnil**  
  - Conduct functional tests on system concurrency, fault tolerance, load balancing, and user authentication  
- **Hanxiao**  
  - Conduct stress tests to evaluate system performance and stability under high loads  
  - Complete the GUI design and verify its functionality (if time permits)
 
 ## References
 Ghemawat, S., Gobioff, H., & Leung, S.T. (2003). The Google file system. In Proceedings of the Nineteenth ACM Symposium on Operating Systems Principles (pp. 29–43). Association for Computing Machinery.
