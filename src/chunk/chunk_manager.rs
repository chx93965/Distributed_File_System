use std::path::Path;
use uuid::Uuid;

struct Chunk {
    data: Vec<u8>,
    _size: usize,
    id: uuid::Uuid,
}

struct Chunks {
    chunks: Vec<Chunk>,
    _chunk_size: usize,
    chunk_count: usize,
    chunks_dir: String,
}

impl Chunks {
    pub fn new(chunk_size: usize, chunks_dir: String) -> Self {
        Chunks {
            chunks: Vec::new(),
            _chunk_size: chunk_size,
            chunk_count: 0,
            chunks_dir: chunks_dir,
        }
    }
    pub fn add_chunk(&mut self, data: Vec<u8>, id: Uuid) {
        let size = data.len();
        let chunk = Chunk {
            data,
            _size: size,
            id: id,
        };

        // Create the file path for the new chunk
        let chunk_path = Path::new(&self.chunks_dir).join(id.to_string());

        // Write the chunk data to the file
        std::fs::write(&chunk_path, &chunk.data).unwrap();

        self.chunks.push(chunk);
        self.chunk_count += 1;
    }

    pub fn find_chunk(&self, id: Uuid) -> Option<&Chunk> {
        self.chunks.iter().find(|chunk| chunk.id == id)
    }
}

pub struct ChunkManager {
    chunks: Chunks,
    chunks_dir: String,
}

impl ChunkManager {
    pub fn new(chunk_size: usize, chunks_dir: String) -> Self {
        let mut chunk_manager = ChunkManager {
            chunks: Chunks::new(chunk_size, chunks_dir.clone()),
            chunks_dir: chunks_dir,
        };
        chunk_manager.init();
        chunk_manager
    }

    fn init(&mut self) {
        info!("Initializing ChunkManager...");
        let chunks_dir = Path::new(&self.chunks_dir);

        if !chunks_dir.exists() {
            std::fs::create_dir_all(chunks_dir).unwrap();
        }

        let chunk_files = std::fs::read_dir(chunks_dir).unwrap();
        for chunk_file in chunk_files {
            let chunk_file = chunk_file.unwrap();
            let chunk_path = chunk_file.path();
            let chunk_data = std::fs::read(&chunk_path).unwrap();
            let chunk_data_size = chunk_data.len();
            let chunk_id_str = chunk_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            if let Ok(chunk_id) = Uuid::parse_str(&chunk_id_str) {
                let chunk = Chunk {
                    data: chunk_data,
                    _size: chunk_data_size,
                    id: chunk_id,
                };
                self.chunks.chunks.push(chunk);
                self.chunks.chunk_count += 1;
            }
        }
        info!(
            "ChunkManager initialized with {} chunks",
            self.chunks.chunk_count
        );
    }

    pub fn add_chunk(&mut self, data: Vec<u8>, id: Uuid) {
        self.chunks.add_chunk(data, id);
    }

    pub fn get_chunk(&self, id: Uuid) -> Result<Vec<u8>, String> {
        let chunk = self.chunks.find_chunk(id);
        match chunk {
            Some(chunk) => Ok(chunk.data.clone()),
            None => Err("Chunk not found".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHUNKS_DIR: &str = "/tmp/chunks";

    fn pre_test(dir: &str) {
        // if directory exists, clear it
        if Path::new(dir).exists() {
            std::fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn test_add_chunk() {
        // dir be TEST_CHUNKS_DIR/test_add_chunk
        let dir = format!("{}/test_add_chunk", TEST_CHUNKS_DIR);
        pre_test(&dir);

        let mut chunk_manager = ChunkManager::new(1024, dir.to_string());
        let data = vec![0; 1024];
        let id = Uuid::new_v4();
        chunk_manager.add_chunk(data, id);
        assert_eq!(chunk_manager.chunks.chunk_count, 1);
    }

    #[test]
    fn test_init() {
        let dir = format!("{}/test_init", TEST_CHUNKS_DIR);
        pre_test(&dir);

        let chunk_manager = ChunkManager::new(1024, dir.to_string());

        assert_eq!(chunk_manager.chunks.chunk_count, 0);
    }

    #[test]
    fn test_init_with_existing_chunk() {
        let dir = format!("{}/test_init_with_existing_chunk", TEST_CHUNKS_DIR);
        pre_test(&dir);

        let mut chunk_manager = ChunkManager::new(1024, dir.to_string());
        chunk_manager.add_chunk(vec![0; 1024], Uuid::new_v4());
        chunk_manager.add_chunk(vec![0; 1024], Uuid::new_v4());

        let mut chunk_manager = ChunkManager::new(1024, dir.to_string());
        assert_eq!(chunk_manager.chunks.chunk_count, 2);
    }
}
