import subprocess
import time
import os
import random
import string
import concurrent.futures
from dataclasses import dataclass
from typing import List, Tuple, Set
import argparse
import statistics
from queue import Queue
import threading

@dataclass
class TestConfig:
    username: str
    password: str
    num_concurrent: int
    file_size_mb: int
    duration_seconds: int
    client_path: str
    test_files_dir: str

@dataclass
class TestResult:
    operation: str
    throughput_mbps: float
    latency_ms: float
    success_rate: float
    num_operations: int

def execute_dfs_command(config: TestConfig, target: str, action: str, local_path: str = None, remote_path: str = None) -> Tuple[bool, float]:
    """Execute a DFS command and return success status and execution time."""
    start_time = time.time()
    
    cmd = [
        config.client_path,
        "--username", config.username,
        "--password", config.password,
        "--target", target,
        "--action", action,
    ]
    
    if local_path:
        cmd.extend(["--local-path", local_path])
    if remote_path:
        cmd.extend(["--remote-path", remote_path])
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True)
        success = result.returncode == 0
        if not success:
            print(f"Command failed: {result.stderr}")
        duration = time.time() - start_time
        return success, duration
    except Exception as e:
        print(f"Error executing command: {e}")
        return False, time.time() - start_time

def create_remote_directories(config: TestConfig, num_workers: int) -> bool:
    """Create required directories in the DFS for testing."""
    print("\nCreating remote directories...")
    
    base_dir = "/test"
    success, _ = execute_dfs_command(config, "directory", "create", remote_path=base_dir)
    if not success:
        print(f"Failed to create base directory: {base_dir}")
        return False
        
    for worker_id in range(num_workers):
        worker_dir = f"{base_dir}/worker_{worker_id}"
        success, _ = execute_dfs_command(config, "directory", "create", remote_path=worker_dir)
        if not success:
            print(f"Failed to create worker directory: {worker_dir}")
            return False
            
    print("Successfully created all remote directories")
    return True

def generate_test_files(directory: str, num_files: int, size_mb: int):
    """Generate test files that will be used for upload testing."""
    os.makedirs(directory, exist_ok=True)
    files = []
    
    print(f"Generating {num_files} test files of {size_mb}MB each...")
    for i in range(num_files):
        filename = os.path.join(directory, f'test_file_{i}.dat')
        with open(filename, 'wb') as f:
            # Generate random data in 1MB chunks
            for _ in range(size_mb):
                chunk = random.randbytes(1024 * 1024)  # 1MB of random data
                f.write(chunk)
        files.append(filename)
    return files

def write_worker(config: TestConfig, worker_id: int, test_files: List[str], successful_writes: Queue) -> List[Tuple[bool, float]]:
    """Worker function to perform write operations and track successful writes."""
    results = []
    start_time = time.time()
    operation_count = 0
    
    while time.time() - start_time < config.duration_seconds:
        local_file = random.choice(test_files)
        timestamp = int(time.time() * 1000000)
        unique_name = f"file_{timestamp}_{worker_id}_{operation_count}.dat"
        remote_path = f"/test/worker_{worker_id}/{unique_name}"
        operation_count += 1
        
        result = execute_dfs_command(config, "file", "create", local_file, remote_path)
        results.append(result)
        
        # If write was successful, add the remote path to the queue
        if result[0]:
            successful_writes.put(remote_path)
                
    return results

def read_worker(config: TestConfig, worker_id: int, remote_files: List[str]) -> List[Tuple[bool, float]]:
    """Worker function to perform read operations from successfully written files."""
    results = []
    start_time = time.time()
    
    while time.time() - start_time < config.duration_seconds:
        if not remote_files:  # No files to read
            break
            
        remote_path = random.choice(remote_files)
        file_name = os.path.basename(remote_path)
        local_path = os.path.join(config.test_files_dir, f"downloaded_{file_name}")
        
        result = execute_dfs_command(config, "file", "read", local_path, remote_path)
        results.append(result)
        
        # Clean up downloaded file
        try:
            if os.path.exists(local_path):
                os.remove(local_path)
        except:
            pass
                
    return results

def run_write_test(config: TestConfig, test_files: List[str]) -> Tuple[TestResult, List[str]]:
    """Run write throughput test and collect successful write paths."""
    print(f"\nStarting write test with {config.num_concurrent} concurrent workers...")
    
    successful_writes = Queue()
    
    with concurrent.futures.ThreadPoolExecutor(max_workers=config.num_concurrent) as executor:
        futures = [
            executor.submit(write_worker, config, i, test_files, successful_writes)
            for i in range(config.num_concurrent)
        ]
        
        all_results = []
        for future in concurrent.futures.as_completed(futures):
            all_results.extend(future.result())
    
    # Collect all successful write paths
    successful_paths = []
    while not successful_writes.empty():
        successful_paths.append(successful_writes.get())
    
    # Calculate metrics
    successful_ops = [r for r, _ in all_results if r]
    latencies = [lat for _, lat in all_results]
    
    num_operations = len(all_results)
    success_rate = len(successful_ops) / num_operations if num_operations > 0 else 0
    avg_latency = statistics.mean(latencies) * 1000 if latencies else 0
    
    total_data_mb = num_operations * config.file_size_mb
    throughput_mbps = total_data_mb / config.duration_seconds
    
    return TestResult(
        operation="create",
        throughput_mbps=throughput_mbps,
        latency_ms=avg_latency,
        success_rate=success_rate,
        num_operations=num_operations
    ), successful_paths

def run_read_test(config: TestConfig, remote_files: List[str]) -> TestResult:
    """Run read throughput test using list of successfully written files."""
    print(f"\nStarting read test with {config.num_concurrent} concurrent workers...")
    
    if not remote_files:
        print("No files available for read test!")
        return TestResult("read", 0, 0, 0, 0)
    
    with concurrent.futures.ThreadPoolExecutor(max_workers=config.num_concurrent) as executor:
        futures = [
            executor.submit(read_worker, config, i, remote_files)
            for i in range(config.num_concurrent)
        ]
        
        all_results = []
        for future in concurrent.futures.as_completed(futures):
            all_results.extend(future.result())
    
    # Calculate metrics
    successful_ops = [r for r, _ in all_results if r]
    latencies = [lat for _, lat in all_results]
    
    num_operations = len(all_results)
    success_rate = len(successful_ops) / num_operations if num_operations > 0 else 0
    avg_latency = statistics.mean(latencies) * 1000 if latencies else 0
    
    total_data_mb = num_operations * config.file_size_mb
    throughput_mbps = total_data_mb / config.duration_seconds if num_operations > 0 else 0
    
    return TestResult(
        operation="read",
        throughput_mbps=throughput_mbps,
        latency_ms=avg_latency,
        success_rate=success_rate,
        num_operations=num_operations
    )

def main():
    parser = argparse.ArgumentParser(description='DFS Throughput Testing Tool')
    parser.add_argument('--username', required=True, help='Username for DFS')
    parser.add_argument('--password', required=True, help='Password for DFS')
    parser.add_argument('--client-path', required=True, help='Path to DFS client binary')
    parser.add_argument('--concurrent', type=int, default=4, help='Number of concurrent operations')
    parser.add_argument('--file-size', type=int, default=1, help='Size of each test file in MB')
    parser.add_argument('--num-files', type=int, default=10, help='Number of test files to generate')
    parser.add_argument('--duration', type=int, default=60, help='Test duration in seconds')
    parser.add_argument('--test-files-dir', default='./test_files', help='Directory for test files')
    
    args = parser.parse_args()
    
    config = TestConfig(
        username=args.username,
        password=args.password,
        num_concurrent=args.concurrent,
        file_size_mb=args.file_size,
        duration_seconds=args.duration,
        client_path=args.client_path,
        test_files_dir=args.test_files_dir
    )
    
    # First create the remote directories
    if not create_remote_directories(config, args.concurrent):
        print("Failed to create remote directories. Exiting.")
        return
    
    # Generate test files
    test_files = generate_test_files(config.test_files_dir, args.num_files, args.file_size)
    print(f"Generated {len(test_files)} test files in {config.test_files_dir}")
    
    try:
        # Run write tests and collect successful writes
        write_results, successful_paths = run_write_test(config, test_files)
        print(f"\nSuccessfully wrote {len(successful_paths)} files")
        
        # Run read tests using the successful writes
        read_results = run_read_test(config, successful_paths)
        
        # Print results
        print("\n=== Test Results ===")
        for result in [write_results, read_results]:
            print(f"\n{result.operation.upper()} Operations:")
            print(f"Throughput: {result.throughput_mbps:.2f} MB/s")
            print(f"Average Latency: {result.latency_ms:.2f} ms")
            print(f"Success Rate: {result.success_rate * 100:.2f}%")
            print(f"Total Operations: {result.num_operations}")
            
    finally:
        # Cleanup test files
        if os.path.exists(config.test_files_dir):
            print(f"\nCleaning up test files in {config.test_files_dir}")
            for file in test_files:
                try:
                    os.remove(file)
                except:
                    pass
            try:
                os.rmdir(config.test_files_dir)
            except:
                pass

if __name__ == "__main__":
    main()