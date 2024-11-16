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
*               Locations       : List<Server Locations (IP)>
*               Size            : int
*               Last Modified   : DateTime
*               Valid Lease     : bool
*               Primary Server  : Server Location (IP)
*           }
*       3. 
*   
*/