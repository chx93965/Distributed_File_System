use chrono::prelude::*;
use chrono::DateTime;

use crate::chunk_manager;
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
const CREATED_DIR_SUCCESSFULLY : &str = "Successfully created directory";
const CREATED_FILE_SUCCESSFULLY : &str = "Successfully created file";
const DIR_SIZE: i32 = 4000;
const FILE_SIZE: i32 = 10000;
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
    chunks: Option<Vec<Uuid>>,
    rw_lock: RwLock<i32>,
}

impl FileNode {
    pub fn new(file_name: String, file_parent: String, file_metadata: Metadata) {
        let file = Self {
            file_name: file_name.clone(),
            file_parent: file_parent.clone(),
            chunks: None,
            file_metadata: file_metadata,
            rw_lock: RwLock::new(0),
        };

        if let Some(mut guard) = DIR_MAP.inner.lock().ok() {
            if let Some(map) = guard.as_mut() {
                if let Some(parent) = map.get_mut(&file_parent) {
                    // we get mutable access to the HashMap
                    // and then to the parent node through Arc::get_mut
                    if let Some(parent_node) = Arc::get_mut(parent) {
                        parent_node.files.insert(file_name, file);
                    }
                } else {
                    println!("This directory \"{}\" does not exist !!!", file_parent);
                }
            }
        }
    }
}

#[derive(Debug)]
struct DirectoryNode {
    dir_name: String,
    dir_parent: String,
    dir_metadata: Metadata,
    rw_lock: RwLock<i32>,
    files: HashMap<String, FileNode>,
}

impl DirectoryNode {
    pub fn new(dir_name: String, dir_metadata: Metadata, dir_parent: String) {
        let node = DirectoryNode {
            dir_name: dir_name.clone(),
            dir_metadata: dir_metadata,
            dir_parent: dir_parent.clone(),
            rw_lock: RwLock::new(0),
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

static DIR_MAP: SafeMap<DirectoryNode> = SafeMap::new();

pub fn namespace_manager_init() {
    DIR_MAP.init();
    let root_metadata = Metadata::new(DIR_SIZE, 0x666, "0".to_string(), "root".to_string());
    DirectoryNode::new("/".to_string(), root_metadata, "/".to_string());
    /*
     * Add some random files for test
     */
    let a_metadata = Metadata::new(DIR_SIZE, 0x666, "1".to_string(), "user".to_string());
    DirectoryNode::new("/a".to_string(), a_metadata, "/".to_string());

    let file_metadata = Metadata::new(FILE_SIZE, 0x666, "1".to_string(), "user".to_string());
    FileNode::new("k".to_string(), "/a".to_string(), file_metadata);

    // println!("root : {:?} ----", DIR_MAP.get("/").unwrap());
    // println!("/a : {:?} ----", DIR_MAP.get("/a").unwrap());
}

/////////////////////////////////////////////////////
/// Path Lookup

pub fn file_lookup(path: String, chunk_index: i32) {
    let (directory, filename) = path.rsplit_once('/').unwrap_or(("", &path));
    println!("dir {}, file {}", directory, filename);
    if let Some(dir) = DIR_MAP.get(&directory) {
        if let Some(file) = dir.files.get(filename) {
            println!("{:?}", file);
        } else {
            println!("{}", NO_FILE_EXIST);
        }
    } else {
        println!("{}", NO_DIR_EXIST);
    }
}

////////////////////////////////////////////////////
/// File Operations

/*
*   Example : file_create(/foo/bar.txt)
*/
pub fn file_create(path: String) {
    /*
     *   TODO : Call logger and wait to log operation
     *      .... Call logger ....
     */

    /*
     *      The file is empty until a write is made to it
     *      then we need to allocate some chunks for the
     *      actual data. But until then only existing in the
     *      namespace would safice :)
     */
    let (directory, filename) = path.rsplit_once('/').unwrap_or(("", &path));
    if let Some(dir) = DIR_MAP.get(directory) {
        dir.rw_lock.write();

        if let Some(file) = dir.files.get(filename) {
            println!("{}", FILE_ALREADY_EXIST);
        } else {
            /*
             *      This metadata should be given by the client but until then
             *      add a dummy metadata
             */

            /*
             *  Should check a bunch of metadata for permissions beforhand but will skip this 
             *  til' later
             *  .... Check Permissions ....
             */
            let m = Metadata::new(FILE_SIZE, 0x666, "1".to_string(), "user".to_string());
            FileNode::new(filename.to_string(), dir.dir_name.to_string(), m);
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
pub fn file_write(path: String, size: i64){

}



////////////////////////////////////////////////////
/// Directory Operations

pub fn list_directory(path: String) {}

pub fn directory_create(path: String) {
    /*
     *   Call logger and wait to log operation
     */
    if let Some(new_dir) = DIR_MAP.get(path.as_str()) {
        println!("{}", DIR_ALREADY_EXIST);
    } else {
        let (parent_dir, new_dir) = path.rsplit_once('/').unwrap_or(("", &path));
        if let Some(parent) = DIR_MAP.get(parent_dir) {
            parent.rw_lock.write();
            /*
             *      This metadata should be given by the client but until then
             *      add a dummy metadata
             */
            let m = Metadata::new(FILE_SIZE, 0x666, "1".to_string(), "user".to_string());
            DirectoryNode::new(path, m, parent.dir_name.to_string());
            println!("{}", CREATED_DIR_SUCCESSFULLY);
        } else {
            println!("{}", NO_DIR_EXIST);
        }
    }
}

pub fn directory_delete(path: String) {
    /*
     *   Call logger and wait to log operation
     */
}
