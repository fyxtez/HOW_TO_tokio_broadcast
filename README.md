This a simple example on how to use: 1 Publisher -> Multiple listeners model.   

Downside: If reciever is slow to respond to new message, and publisher queue is full, messages might be dropped and cause "lag" hence listeners wont recieve the message.

Potential better solutions:  

1. flume

2. Nats

3. RabbitMQ
