#!/bin/bash


# Function to launch a tmux session with a specific name and command
launch_tmux_session() {
    local session_name=$1
    local command=$2

    tmux new-session -d -s "$session_name" "$command"
}

# Cleanup function to kill all tmux sessions for master, client, and chunkservers
cleanup() {
    echo "Cleaning up all tmux sessions..."
    tmux kill-session -t master 2>/dev/null
    tmux kill-session -t client 2>/dev/null
    for session in $(tmux list-sessions -F "#S" | grep "chunkserver_"); do
        tmux kill-session -t "$session"
    done
    echo "All sessions terminated."
}

# Trap EXIT signal to call cleanup when the script exits
trap cleanup EXIT

# Check for required arguments
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <num_chunkservers>"
    exit 1
fi

# Variables
NUM_CHUNKSERVERS=$1
MASTER_BINARY="./target/release/master"
CHUNK_BINARY="./target/release/chunk"

# Check if required binaries exist
if [ ! -f "$MASTER_BINARY" ]; then
    echo "Error: Master binary not found at $MASTER_BINARY"
    exit 1
fi

if [ ! -f "$CHUNK_BINARY" ]; then
    echo "Error: Chunkserver binary not found at $CHUNK_BINARY"
    exit 1
fi

# Start the master server
launch_tmux_session "master" "$MASTER_BINARY"
echo "Master server started in tmux session 'master'."

# Start the specified number of chunk servers
for ((i = 0; i < NUM_CHUNKSERVERS; i++)); do
    PORT=$((8100 + i))
    SESSION_NAME="chunkserver_$i"
    launch_tmux_session "$SESSION_NAME" "$CHUNK_BINARY --port $PORT"
    echo "Chunkserver $i started in tmux session '$SESSION_NAME' on port $PORT."
done

# Attach to the master session
tmux attach-session -t master
