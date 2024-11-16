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

/////////////////////////////////////////////////////
/// Path Lookup

fn path_lookup(path:String){

}

////////////////////////////////////////////////////
/// File Operations

fn file_create(path:String){
/*
*   Call logger and wait to log operation
*/


}

fn file_delete(path:String){
/*
*   Call logger and wait to log operation
*/

}

////////////////////////////////////////////////////
/// Directory Operations

fn list_directory(path:String){
    
}


fn directory_create(path:String){
/*
*   Call logger and wait to log operation
*/

}


fn directory_delete(path:String){
/*
*   Call logger and wait to log operation
*/

}