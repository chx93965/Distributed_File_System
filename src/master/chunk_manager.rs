/*
*   Manages and allocates chunks and chunk handles
*   also manages leases.
*
*
*
*   Data Structures : 
*       1.  Chunk Map           : Map (Chunk Handle -> Chunk Info)
*       2.  Chunk Info {
*               Version         : uint
*               Locations       : List<Server Locations (IP String)>
*               Size            : int
*               Last Modified   : DateTime
*               Valid Lease     : bool
*               Primary Server  : Server Location (IP String)
*           }
*       3. 
*   
*/

use chrono::{DateTime, Utc};

pub struct ChunkInfo {
    version : u16,
    locations : Vec<String>,
    size : i64,
    last_modified : DateTime<Utc>,
    primary_server : Vec<String>
}