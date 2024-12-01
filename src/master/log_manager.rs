/*
*   The operation logger.
*   The purpose is to log each operation before doing it 
*   so that if an operation fails, we could redo the opertaions
*   in logs.
*
*   Purpose : 
*       1. Crash Recovery 
*       2. Replication to other masters (eventually)
*
*
*   Functions : 
*       1. Log Operation Write ( Synchronous Call From Namespace Manager)
*       2. Recover
*       3. Create Checkpoint
*       4. Log Operation Read
*           
*/