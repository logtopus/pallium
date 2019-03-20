# Pallium

The user interface to the logtopus server. 

## Development

### Requirements

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

```
wasm-pack build
```

### Package for web

```
npm init wasm-app www
```
