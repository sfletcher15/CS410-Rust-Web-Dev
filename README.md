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
### Assignment 2 - Persistent Data
I'm using postgresql for the persistent backend for my persistent data and will be assuming you're on mac for this guide. 

To begin we'll install postgresql with the commands,
```zsh
brew update
brew install postgresql
```
Then we'll start our postgresql service with the command,
```zsh
brew services start postgresql
```
If this is the first time setting up postgresql we might need to initialize the database with the command,
```zsh
initdb /usr/local/var/postgres
```
Once we have the database initialized we can run postgresql to setup the database with,
```zsh
psql postgres
```
Once postgresql is running we'll set up our Q&A database with the commands,
```postgresql
CREATE USER dba WITH PASSWORD 'your_password';
CREATE DATABASE qa_db;
GRANT ALL PRIVILEGES ON DATABASE qa_db TO dba;
```
Now that our Q&A database is setup we can connect to it and create some tables for our questions and our answers.
First we'll connect to the new database with `psql qa_db` then we'll run the commands,
```postgresql
CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    question_text TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    question_id INTEGER NOT NULL,
    answer_text TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
);
```
Now that the database is setup and running we'll put the connection string in a .env file so our app can connect to it.
```
DATABASE_URL=postgres://localhost:5432/qa_db
```
Running the app should then be as simple as `cargo run`.