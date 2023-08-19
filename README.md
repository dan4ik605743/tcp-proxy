[![Linux](https://github.com/dan4ik605743/tcp-proxy/actions/workflows/Linux.yml/badge.svg)](https://github.com/dan4ik605743/tcp-proxy/actions/workflows/Linux.yml)
# tcp-proxy

Tcp Proxy with tcp read timeout and connection limit. There are also illustrative implementations of the server and the client that communicate with each other, the client sends a message to the server, thereby the server reads it and responds in response, the client displays the message sent by the server.

## Configs
All three binary crates ( client, server, proxy ) are configured using command line arguments, which are then converted into a json format config. All arguments have a default value, so when you run any of the programs, they will be taken and saved in the config according to the default patch. You can specify a patch to the config yourself, or the config will be read / generated from the default path.
For more information use: app -h
You can find config examples in: <a href="https://github.com/dan4ik605743/tcp-proxy/tree/master/configs">configs</a> 

* Default proxy path: ./.proxy-config.json
* Default server / client path: ./config.json

## Implementation
* Server / Client: The protocol for communication between the server is implemented through a tulpa with one string field, which is deserialized and serialized to find out the size of the message. Also, the client and server write logs about what is happening. If the server is turned off, the client will endlessly connect to it until it successfully connects, so when sending data during a server shutdown, all messages will be delivered after connecting to the server. The client and server are written asynchronously using tokio-rs. When logging messages, instead of Vec(u8), I used Bytes from tokio.
* Proxy: When connecting, the proxy adds the user to the HashMap, which consists of a key with IpAddr and a usize value, to count connected users. Proxy checks before reading/writing to readers/writers client and server whether this operation is allowed to restrict connected users with one IP. It also does all operations with IO with a given timeout in the config. Therefore, when the timeout ends without receiving data from the server / client to the readers, it breaks the connection between them. In parallel, reading of both readers and writing to writers will be performed. Also, the proxy has logging about all operations. 