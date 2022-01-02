set dotenv-load := false

act:
    #! /bin/sh
    cd /users/alejandro/actix/ 
    ./target/release/guard &
    disown

@kill:
    -killall guard aoxo