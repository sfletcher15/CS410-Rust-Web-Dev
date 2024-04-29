# Rust Web Example
### Sam Fletcher - Spring 2024
This repo contains the coursework and notes for the Portland State Rust Web Development course.

### Assignment 1 - Get the REST up
My implementation of this doesn't exactly follow the book nor the class's implementation.\
This web allows the posting, getting, updating, and deletion of questions to a non-persistent in memory hash map.\
There are some issues such as no error checking/messages, and an issue where if you delete the first question in the list it will continue to replace the second without ever growing the list.\
These will be fixed/updated as time permits.\

Some example cURL commands to test the functionality include.
```
GET:
curl -X GET http://127.0.0.1:3030/questions  

POST:
curl -X POST -H "Content-Type: application/json" \
-d '{"question":"What is the dog doing?"}' \
http://127.0.0.1:3030/questions

UPDATE:
curl -v -X PUT -H "Content-Type: application/json" \
-d '{"question":"What is a crate in Rust?"}' \
http://127.0.0.1:3030/questions/2

DELETE:
curl -X DELETE http://127.0.0.1:3030/questions/2 
```
