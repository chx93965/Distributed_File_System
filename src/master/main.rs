/*
*   The following is based on prof. Ashvin Goels Slides on GFS
*/


/*  
*   Maintains filesystem's metadata in memory :
*       1. Chunk namespace, i.e., all chunk handles in the system
*           --> For each chunk : 
*                   I. Reference Count
*                   II. Version Number
*       
*       2.  File namespace, i.e., all file paths
*           --> For each file path:
*                   I. ACL
*                   II. File -> chunk_handle mappings
*
*   All the metatdata changes are logged to disk for persistency
*   Log is replicated to backup master nodes
*   Memory Structures changes are periodically checkpointed
*
*   Manage chunks and replicas :
*       1. Creates new chunks on chunkservers
*       2. Track chunk replicas by caching chunk locations, i.e., 
*           chunkservers on which a chunk is stored
*       3. Makes chunk replica placement decisions
*
*   Has "per-filepath" read-write locks : 
*       Example : 
*                To modify /a/b/c, 
*                acquire read locks on /a, /a/b, 
*                write lock on /a/b/c
* 
*   Communication with Chunkserver : 
*       HeartBeat messages :
*           Find chunk locations
*           Do lease management (Primary for a chunk)
*           Find stale chunk servers
*           Garbage collect orphaned and stale chunks
*
*/

fn main() {


    /*
    *   Input  : Get (file name, chunk index) from client
    *   Output : Ret (chunk handle, chunk locations) to client
    */

    /*
    *   Get chunkserver state from chunkservers
    *   Ret Instruction to chunkservers
    */

}

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
*                    Files          : Map ("File Name" -> File Node)
*                    Directories    : Map ("Dir Name" -> Dir Node)
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
*               Permissions     : Permission
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
fn namespace_manager(){

}


fn path_lookup(){

}

fn file_create(){

}

fn file_delete(){

}

fn list_directory(){
    
}
/*
*   Heartbeat message sent to chunkservers
*   Input : 
*   Output : Chunk Location - Send Data to Chunk
*/
fn heartbeat(){
    //TODO
}


