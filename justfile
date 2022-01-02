set dotenv-load := false

act:
    #! /bin/zsh
    cd /users/alejandro/actix/ 
    ./target/release/guard &
    disown

kill:
    -killall guard
    -killall aoxo