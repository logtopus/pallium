# Pallium

The user interface to the logtopus server. 

## Development

### Requirements / Setup

Install wasm toolchain:

* wasm-pack
    
```
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

* npm, example for Ubuntu: 

```
    sudo apt install npm
```

* ensure npm latest version

```
    npm install npm@latest -g
```
      
See further documentation at:

https://rustwasm.github.io/book/game-of-life/setup.html


### Build

To update rust code, run wasm build from repository root.

```
wasm-pack build
```

Change to pkg folder, link the local package to avoid 
publishing during development:

``` 
npm link
```

Change to www folder, 
install or update npm dependencies and use locally 
linked package:

``` 
npm install
npm link pallium
```


### Run development webserver

From www folder execute:

``` 
npm run start
```

Open http://localhost:8080

