import subprocess
import re
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from typing import Dict, List, Tuple
import json
import sys



def check_dependencies():
    """Check if required packages are installed."""
    required = {'pandas', 'matplotlib'}
    missing = []
    
    for package in required:
        try:
            __import__(package)
        except ImportError:
            missing.append(package)
    
    if missing:
        print("Missing required packages. Please install them using:")
        print(f"pip install {' '.join(missing)}")
        sys.exit(1)

# [Previous imports remain the same...]

def run_throughput_test(
    client_path: str,
    username: str,
    password: str,
    concurrent: int,
    file_size: int,
    duration: int,
    test_files_dir: str
) -> Dict[str, Dict[str, float]]:
    """Run a single throughput test with specified parameters."""
    cmd = [
        "python3", "dfs_throughput_test.py",
        "--username", username,
        "--password", password,
        "--client-path", client_path,
        "--concurrent", str(concurrent),
        "--file-size", str(file_size),
        "--duration", str(duration),
        "--test-files-dir", test_files_dir
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True)
        print("\nCommand output:")
        print(result.stdout)
        
        if result.returncode != 0:
            print(f"Error running test with {concurrent} threads:")
            print(result.stderr)
            return None
            
        # Parse the output
        metrics = {'create': {}, 'read': {}}
        current_op = None
        
        for line in result.stdout.split('\n'):
            print(f"Parsing line: {line}")  # Debug output
            if 'CREATE Operations:' in line:
                current_op = 'create'
            elif 'READ Operations:' in line:
                current_op = 'read'
            elif current_op and ':' in line:
                key, value = line.strip().split(':')
                key = key.strip().lower()
                
                # Map the output keys to our expected column names
                key_mapping = {
                    'throughput': 'throughput_mbps',
                    'average latency': 'latency_ms',
                    'success rate': 'success_rate',
                    'total operations': 'total_operations'
                }
                
                if key in key_mapping:
                    try:
                        value = float(value.split()[0])  # Extract number before unit
                        metrics[current_op][key_mapping[key]] = value
                        print(f"Parsed metric: {key_mapping[key]} = {value}")  # Debug output
                    except (ValueError, IndexError) as e:
                        print(f"Error parsing value for {key}: {e}")
                        continue
        
        print(f"\nFinal metrics for thread count {concurrent}:")
        print(json.dumps(metrics, indent=2))
        return metrics
    except Exception as e:
        print(f"Error during test execution: {e}")
        return None

def run_experiments(
    client_path: str,
    username: str,
    password: str,
    concurrent_threads: List[int],
    file_size: int = 1,
    duration: int = 30,
    repeats: int = 3
) -> pd.DataFrame:
    """Run experiments with different numbers of concurrent threads."""
    results = []
    
    for threads in concurrent_threads:
        print(f"\nTesting with {threads} concurrent threads")
        for repeat in range(repeats):
            print(f"  Repeat {repeat + 1}/{repeats}")
            metrics = run_throughput_test(
                client_path=client_path,
                username=username,
                password=password,
                concurrent=threads,
                file_size=file_size,
                duration=duration,
                test_files_dir=f"./test_files_{threads}_{repeat}"
            )
            
            if metrics:
                for op_type, op_metrics in metrics.items():
                    results.append({
                        'threads': threads,
                        'operation': op_type,
                        'repeat': repeat,
                        **op_metrics
                    })
    
    df = pd.DataFrame(results)
    print("\nDataFrame columns:", df.columns.tolist())  # Debug output
    print("\nDataFrame head:")
    print(df.head())
    return df

def plot_results(df: pd.DataFrame, output_prefix: str = "dfs_benchmark"):
    """Create various plots from the results."""
    print("\nColumns in DataFrame:", df.columns.tolist())  # Debug output
    
    # Set up the style
    plt.style.use('default')
    colors = {'create': 'blue', 'read': 'red'}
    markers = {'create': 'o', 'read': 's'}
    
    # Use only the metrics that are actually present in the DataFrame
    available_metrics = [
        metric for metric in ['throughput_mbps', 'latency_ms', 'success_rate', 'total_operations']
        if metric in df.columns
    ]
    
    print("\nAvailable metrics for plotting:", available_metrics)  # Debug output
    
    # Calculate mean and std for each metric
    agg_df = df.groupby(['threads', 'operation'])[available_metrics].agg(['mean', 'std']).reset_index()
    
    # Create plots for each metric
    for metric in available_metrics:
        plt.figure(figsize=(10, 6))
        
        for operation in ['create', 'read']:
            op_data = df[df['operation'] == operation]
            mean_data = agg_df[agg_df['operation'] == operation]
            
            # Plot individual points
            plt.scatter(op_data['threads'], op_data[metric], 
                       alpha=0.3, color=colors[operation], marker=markers[operation],
                       label=f'{operation} (individual runs)')
            
            # Plot mean with error bars
            plt.errorbar(mean_data['threads'], 
                        mean_data[(metric, 'mean')],
                        yerr=mean_data[(metric, 'std')],
                        label=f'{operation} (mean ± std)',
                        color=colors[operation],
                        capsize=5, capthick=2, linewidth=2)
        
        plt.xlabel('Number of Concurrent Threads')
        metric_name = metric.replace('_', ' ').title()
        if metric == 'throughput_mbps':
            metric_name = 'Throughput (MB/s)'
        elif metric == 'latency_ms':
            metric_name = 'Latency (ms)'
        elif metric == 'success_rate':
            metric_name = 'Success Rate (%)'
        
        plt.ylabel(metric_name)
        plt.title(f'DFS Performance: {metric_name} vs Concurrent Threads')
        plt.legend()
        plt.grid(True)
        plt.savefig(f'{output_prefix}_{metric}.png')
        plt.close()
    
    # Save raw data
    df.to_csv(f'{output_prefix}_raw_data.csv', index=False)
    
    # Generate summary statistics
    summary = df.groupby(['threads', 'operation'])[available_metrics].agg(['mean', 'std', 'min', 'max'])
    summary.to_csv(f'{output_prefix}_summary.csv')

# [Rest of the code remains the same...]
def main():
    # Check dependencies first
    check_dependencies()
    
    parser = argparse.ArgumentParser(description='DFS Benchmarking Tool')
    parser.add_argument('--username', required=True, help='Username for DFS')
    parser.add_argument('--password', required=True, help='Password for DFS')
    parser.add_argument('--client-path', required=True, help='Path to DFS client binary')
    parser.add_argument('--max-threads', type=int, default=16, 
                        help='Maximum number of concurrent threads to test')
    parser.add_argument('--file-size', type=int, default=1,
                        help='Size of test files in MB')
    parser.add_argument('--duration', type=int, default=30,
                        help='Duration of each test in seconds')
    parser.add_argument('--repeats', type=int, default=3,
                        help='Number of times to repeat each test')
    parser.add_argument('--output', default='dfs_benchmark',
                        help='Prefix for output files')
    
    args = parser.parse_args()
    
    # Generate thread counts to test (powers of 2 up to max-threads)
    thread_counts = [2**i for i in range(0, args.max_threads.bit_length())]
    if thread_counts[-1] < args.max_threads:
        thread_counts.append(args.max_threads)
    
    print(f"Will test with {thread_counts} concurrent threads")
    print(f"Each test will run for {args.duration} seconds")
    print(f"Each configuration will be repeated {args.repeats} times")
    
    results_df = run_experiments(
        client_path=args.client_path,
        username=args.username,
        password=args.password,
        concurrent_threads=thread_counts,
        file_size=args.file_size,
        duration=args.duration,
        repeats=args.repeats
    )
    
    plot_results(results_df, args.output)
    print(f"\nResults have been saved with prefix '{args.output}'")
    print("Generated files:")
    print(f"- {args.output}_raw_data.csv (Raw test data)")
    print(f"- {args.output}_summary.csv (Statistical summary)")
    print(f"- {args.output}_throughput_mbps.png (Throughput plot)")
    print(f"- {args.output}_latency_ms.png (Latency plot)")
    print(f"- {args.output}_success_rate.png (Success rate plot)")
    print(f"- {args.output}_total_operations.png (Operations plot)")

if __name__ == "__main__":
    main()