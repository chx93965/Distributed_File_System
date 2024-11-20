use chrono::prelude::*;
use chrono::DateTime;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
/*
*   A managers for all the files and directories
*   in the file system
*
*
*   Data Structures :
*       1.  Directory Node {
*                Directory Name         : String;
*                Directory Parent       : Directory Node;
*                Directory Metadata     : Metadata
*                Read Write Lock        : RW Lock
*
*                Children = {
*                    Files          : List < File Node >
*                    Directories    : List < Dir Node >
*                }
*
*
*            }
*
*       2.  File Node {
*               File Name           : String
*               File Parent         : Directory Node
*               Chunks              : List<Chunk Handle>
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

#[derive(Debug)]
struct Metadata {
    size: i32,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    permission: i32,
    owner: String,
    group: String,
}

impl Metadata {
    pub fn new(
        size: i32,
        permission: i32,
        owner: String,
        group: String,
    ) -> Self {
        let utc_now: DateTime<Utc> = Utc::now();
        Self {
            size : size,
            creation_time : utc_now,
            modification_time : utc_now,
            permission : permission,
            owner : owner,
            group : group
        }
    }
}

#[derive(Debug)]
struct FileNode {
    file_name       : String,
    file_parent     : String,
    file_metadata   : Metadata,
    rw_lock         : RwLock<i32>,
}

impl FileNode {
    pub fn new(file_name: String, file_parent: String, file_metadata: Metadata) -> Self {
        Self {
            file_name: file_name,
            file_parent: file_parent,
            file_metadata: file_metadata,
            rw_lock: RwLock::new(0),
        }
    }
}

const DIR_SIZE :i32 = 4000;

#[derive(Debug)]
struct DirectoryNode {
    dir_name: String,
    dir_parent: String,
    dir_metadata: Metadata,
    rw_lock: RwLock<i32>,
    children: DirectoryChildren,
}

impl DirectoryNode  {
    pub fn new(dir_name: String, dir_metadata: Metadata, dir_parent : String) {
        let node = DirectoryNode {
            dir_name: dir_name.clone(),
            dir_metadata: dir_metadata,
            dir_parent : dir_parent,
            rw_lock: RwLock::new(0),
            children: DirectoryChildren::new(),
        };
        /*
        *   Add the new node to the directory map
        */
        DIR_MAP.insert(dir_name.clone(), node);
    }
}

#[derive(Debug)]
struct DirectoryChildren {
    directories: Vec<DirectoryNode>,
    files: Vec<FileNode>,
}

impl DirectoryChildren {
    pub const fn new() -> Self {
        return Self {
            directories: Vec::new(),
            files: Vec::new(),
        };
    }
}

pub struct SafeMap {
    inner: Mutex<Option<HashMap<String, Arc<DirectoryNode>>>>,
}

impl SafeMap {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub fn init(&self) {
        let mut guard = self.inner.lock().unwrap();
        if guard.is_none() {
            *guard = Some(HashMap::new());
        }
    }

    pub fn insert(&self, key: String, value: DirectoryNode) -> Option<Arc<DirectoryNode>> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(map) = guard.as_mut() {
            map.insert(key, Arc::new(value))
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<DirectoryNode>> {
        let guard = self.inner.lock().unwrap();
        guard.as_ref().and_then(|map| map.get(key).cloned())
    }
}

static DIR_MAP: SafeMap = SafeMap::new();

pub fn namespace_manager_init() {
    DIR_MAP.init();
    let mut root_metadata = Metadata::new(DIR_SIZE, 0x666, "0".to_string(), "root".to_string());
    DirectoryNode::new("/".to_string(), root_metadata, "/".to_string());
    /*
    * Add some random files for test
    */
}

/////////////////////////////////////////////////////
/// Path Lookup

pub fn path_lookup(path: String, chunk_index: i32) {
   
}

////////////////////////////////////////////////////
/// File Operations

fn file_create(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}

fn file_delete(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}

////////////////////////////////////////////////////
/// Directory Operations

fn list_directory(path: String) {}

fn directory_create(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}

fn directory_delete(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}
