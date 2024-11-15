
## Simple mutex-based channel

You can use a VecDeque as a data structure to allow different threads to send messages to each other. Threads that want to send a message can simply add it to the back of the queue and threads that want to recieve messages just remove one from the front of the queue.

One thing you would probably need to add is a Condvar to notify waiting receivers of a new message.