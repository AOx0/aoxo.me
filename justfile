set dotenv-load := false
alias update:= up

default: up

act:
    #! /bin/zsh
    cd /users/alejandro/actix/ 
    ./target/release/guard &
    disown

@kill:
    -killall guard
    -killall aoxo

_replace:
    #! /Library/Frameworks/Python.framework/Versions/3.10/bin/python3
    with open("/Users/alejandro/actix/public/index.html", "r+") as f:
        text = f.read()
        f.close()

    with open("/Users/alejandro/actix/public/index.html", "w") as fw:
        text = text.replace("https://aox0.github.io", "https://aoxo.me")
        fw.write(text)
        f.close()
@up: && _replace
    cp /Users/alejandro/AOx0.github.io/index.html /Users/alejandro/actix/public
