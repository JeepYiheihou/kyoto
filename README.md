# osaka
A modular designed high performance cache

## Functionality
A key-value cache prototype.

Serves HTTP protocol. Client commands has to comply with HTTP request format, and the command information should be json in request body.

For example:
```
{ "command": "GET", "key": "foo" }
```
or:
```
{ "command": "SET", "key": "foo", value: "bar" }
```

## Idea
Modularize.

This project aim to test whether a database can be decoupled into essential modules: Network part, Machine part and Data Structure part.
