/*
*   The following is based on prof. Ashvin Goels Slides on GFS
*/

mod namespace_manager;
mod chunk_manager;

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
    namespace_manager::namespace_manager_init();
    file_read("/a/k".to_string(), 2);
}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Create File Entry
*       5. Release Directory Lock
*/
fn file_create(){

}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Delete File Entry
*       5. Release Directory Lock
*/
fn file_delete(){
    
}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Release Directory Lock
*/
fn file_read(file_name:String, chunk_index:i32){
    namespace_manager::path_lookup(file_name, chunk_index);
}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Release Directory Lock
*/
fn file_write(){

}


/*
*   All done by namespace manager : 
*       1. Lookup Parent Directory 
*       2. Acquire Parent Lock
*       3. Check Permissions
*       4. Create Directory
*       5. Release Parent Lock
*/
fn direcotry_create(){

}

/*
*   All done by namespace manager : 
*       1. Lookup Parent Directory 
*       2. Acquire Parent Lock
*       3. Check Permissions
*       4. Delete Directory
*       5. Release Parent Lock
*/
fn directory_delete(){
    
}


fn update_namespace(){

}


/*
*   Heartbeat received from chunkservers.
*   Called by chunkservers with : Chunkserver ID + Chunks + Chunk Info
*   Input : 
*   Output : Chunk Location - Send Data to Chunk
*/
fn heartbeat(){
    
}