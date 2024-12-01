use chrono::prelude::*;
use chrono::DateTime;

use crate::chunk_manager;
use crate::chunk_manager::{get_chunks, write_chunks};
use crate::safe_map::SafeMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

/*
*   A managers for all the files and directories
*   in the file system
*
*
*   Data Structures :
*       1.  Directory Node {
*                Directory Name         : String
*                Directory Parent       : String
*                Directory Metadata     : Metadata
*                Read Write Lock        : RW Lock
*                Files                  : List<File Node>
*
*            }
*
*       2.  File Node {
*               File Name           : String
*               File Parent         : String
*               Chunks              : List<Chunk Handle(uuid)>
*               File Metadata       : Metadata
*               Read Write Lock     : RW Lock
*           }
*
*
*
*       3.  Directory Map (Path -> Directory Node)
*
*
*       4.  Metadata {
*               Size            : i64
*               Creation Time   : DateTime
*               Modify Time     : DateTime
*               Permissions     : i32
*               Owner           : String
*               Group           : String
*           }
*
*
*
*
*
*   Functions :
*
*       1. Lookup Path
*       2. Create File
*       3. Delete File
*       4. List Directory
*       5. Rename
*
*
*
*
*/

const NO_DIR_EXIST: &str = "No such directory exists";
const NO_FILE_EXIST: &str = "No such file exists";
const FILE_ALREADY_EXIST: &str = "This file already exists";
const DIR_ALREADY_EXIST: &str = "This directory already exists";
const CREATED_DIR_SUCCESSFULLY: &str = "Successfully created directory";
const CREATED_FILE_SUCCESSFULLY: &str = "Successfully created file";
const DIR_SIZE: i32 = 4000;
const FILE_SIZE: i32 = 10000;
#[derive(Debug, Clone)]
struct Metadata {
    size: i32,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    permission: i32,
    owner: String,
    group: String,
}

impl Metadata {
    pub fn new(size: i32, permission: i32, owner: String, group: String) -> Self {
        let utc_now: DateTime<Utc> = Utc::now();
        Self {
            size: size,
            creation_time: utc_now,
            modification_time: utc_now,
            permission: permission,
            owner: owner,
            group: group,
        }
    }
}

#[derive(Debug)]
struct FileNode {
    file_name: String,
    file_parent: String,
    file_metadata: Metadata,
    chunks: Vec<Vec<Uuid>>,
    // rw_lock: RwLock<i32>,
}

impl FileNode {
    pub fn new(
        file_name: String,
        file_parent: String,
        file_metadata: Metadata,
    ) -> Result<(), String> {
        let file = Self {
            file_name: file_name.clone(),
            file_parent: file_parent.clone(),
            chunks: Vec::new(),
            file_metadata: file_metadata,
        };

        // Acquire DIR_MAP lock first
        let dir_map_guard = DIR_MAP
            .inner
            .lock()
            .map_err(|_| "Failed to acquire lock on DIR_MAP")?;

        let map = dir_map_guard.as_ref().ok_or("DIR_MAP not initialized")?;

        // Get parent dir reference before modifying anything
        let parent = map
            .get(&file_parent)
            .ok_or_else(|| format!("Directory '{}' does not exist", file_parent))?;

        // Then acquire write lock on parent directory
        let mut parent_write = parent
            .write()
            .map_err(|_| "Failed to acquire write lock on parent directory")?;

        parent_write
            .files
            .insert(file_name, Arc::new(RwLock::new(file)));
        Ok(())
    }
}

#[derive(Debug)]
struct DirectoryNode {
    dir_name: String,
    dir_parent: String,
    dir_metadata: Metadata,
    files: HashMap<String, Arc<RwLock<FileNode>>>,
}

impl DirectoryNode {
    pub fn new(dir_name: String, dir_metadata: Metadata, dir_parent: String) {
        let node = DirectoryNode {
            dir_name: dir_name.clone(),
            dir_metadata: dir_metadata,
            dir_parent: dir_parent.clone(),
            files: HashMap::new(),
        };
        /*
         *   Add the new node to the directory map
         */
        if DIR_MAP.get(&dir_parent).is_some() || dir_parent == "/" {
            DIR_MAP.insert(dir_name.clone(), node);
        } else {
            println!("This directory \"{}\" does not exist !!!", dir_parent);
        }
    }
}

static DIR_MAP: SafeMap<String, DirectoryNode> = SafeMap::new();

pub fn namespace_manager_init() {
    DIR_MAP.init();
    let root_metadata = Metadata::new(DIR_SIZE, 0x666, "0".to_string(), "root".to_string());
    DirectoryNode::new("/".to_string(), root_metadata, "/".to_string());
}

/////////////////////////////////////////////////////
/// Path Lookup

pub fn file_lookup(path: String) -> Result<Vec<Vec<Uuid>>, String>{
    let (directory, filename) = match path.rsplit_once('/') {
        Some((dir, name)) if dir.is_empty() => ("/", name),
        Some((dir, name)) => (dir, name),
        None => ("/", path.as_str()),
    };
    println!("dir {}, file {}", directory, filename);
    if let Some(dir) = DIR_MAP.get(&directory.to_string()) {
        // println!("this is dir : {:?}", dir);
        if let Some(file) = dir.read().unwrap().files.get(filename) {
            return Ok(file.read().unwrap().chunks.clone());
        } else {
            return Err(format!("{}",NO_FILE_EXIST));
        }
    } else {
        return Err(format!("{}",NO_DIR_EXIST));
    }
}

////////////////////////////////////////////////////
/// File Operations

/*
*   Example : file_create(/foo/bar.txt)
*/
pub fn file_create(path: String) {
    let (directory, filename) = match path.rsplit_once('/') {
        Some((dir, name)) if dir.is_empty() => ("/", name),
        Some((dir, name)) => (dir, name),
        None => ("/", path.as_str()),
    };

    // Get directory lock first
    if let Some(dir) = DIR_MAP.get(&directory.to_string()) {
        // Then check files with a read lock
        let dir_read = match dir.read() {
            Ok(guard) => guard,
            Err(_) => {
                println!("Failed to acquire read lock on directory");
                return;
            }
        };

        if dir_read.files.get(filename).is_some() {
            println!("{}", FILE_ALREADY_EXIST);
            return;
        }

        // Drop read lock before creating file
        drop(dir_read);

        let m = Metadata::new(FILE_SIZE, 0x666, "1".to_string(), "user".to_string());
        if let Err(e) = FileNode::new(filename.to_string(), directory.to_string(), m) {
            println!("Failed to create file: {}", e);
        } else {
            println!("{}", CREATED_FILE_SUCCESSFULLY);
        }
    } else {
        println!("{}", NO_DIR_EXIST);
    }
}

pub fn file_delete(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}

/*
 *      Write to a file.
 *      1. Allocate some chunks according to the
 *         chunkmanagers best fit chunkservers
 *
 *      2. Update chunkservers to tell them the
 *         chunk handles.
 *
 *
 */
pub fn file_write(path: String, size: usize) -> Result<(Vec<(Uuid,String)>), String> {
    let (directory, filename) = match path.rsplit_once('/') {
        Some((dir, name)) if dir.is_empty() => ("/", name),
        Some((dir, name)) => (dir, name),
        None => ("/", path.as_str()),
    };

    let parent = DIR_MAP
        .get(&directory.to_string())
        .ok_or_else(|| format!("Directory '{}' does not exist", directory))?;

    let dir_read = match parent.read() {
        Ok(guard) => guard,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let file: &Arc<RwLock<FileNode>> = dir_read
        .files
        .get(filename)
        .ok_or_else(|| format!("No such file {} exists", filename))?;

    // parent.read().unwrap().files.get()
    let chunks = write_chunks(size);
    file.write().unwrap().chunks.push(chunks.iter().map(|x| x.0).collect::<Vec<Uuid>>());
    Ok(chunks)
}

pub fn file_read(path: String, chunk_index: usize) -> Result<Vec<(Uuid, String)>, String>{
    let chunks = file_lookup(path)?;
    let tuples = get_chunks(chunks[chunk_index].clone());
    Ok(tuples)
}

////////////////////////////////////////////////////
/// Directory Operations

pub fn list_directory(path: String) {
    if let Some(dir) = DIR_MAP.get(&path) {
        println!("------------------------------------------");
        println!("{:?}", dir.read().unwrap().files);
    } else {
        println!("{}", NO_DIR_EXIST);
    }
}

pub fn directory_create(path: String) {
    /*
     *   Call logger and wait to log operation
     */
    if let Some(new_dir) = DIR_MAP.get(&path) {
        println!("{}", DIR_ALREADY_EXIST);
    } else {
        let (parent_dir, dir) = match path.rsplit_once('/') {
            Some((dir, name)) if dir.is_empty() => ("/", name),
            Some((dir, name)) => (dir, name),
            None => ("/", path.as_str()),
        };
        if let Some(parent) = DIR_MAP.get(&parent_dir.to_string()) {
            /*
             *      This metadata should be given by the client but until then
             *      add a dummy metadata
             */
            let m = Metadata::new(FILE_SIZE, 0x666, "1".to_string(), "user".to_string());
            DirectoryNode::new(path, m, parent.read().unwrap().dir_name.to_string());
            println!("{}", CREATED_DIR_SUCCESSFULLY);
        } else {
            println!("{} : {}", NO_DIR_EXIST, parent_dir);
        }
    }
}

pub fn directory_delete(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}
