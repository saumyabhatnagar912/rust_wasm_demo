# rust_wasm_demo
Rust wasm: simple addition

# Install nightly
First we need to install rust nightly channel to our environment. To install nightly channel run the following command:

    rustup install nightly

Then set rust default channel to nightly

    rustup default nightly
  
# Install wasm target
We need to add wasm target so that we can compile it later.

    rustup target add wasm32-unknown-unknown
  
# wasm-bindgen
we need to install the wasm-bindgen command line tool.

    cargo +nightly install wasm-bindgen-cli
    
# Create your wasm library

    cargo +nightly new <project_name> --lib

# Build Project
We need to use the wasm target to build the .wasm file. The file will be created 
inside "target/wasm32-unknown-unknown/debug" and will be named same as the project name.

     cargo +nightly build --target wasm32-unnknown-unknown
     
# Apply wasm-bindgen
The wasm file created is large in size and unoptimized. Now that we've generated the wasm module it's time to run the bindgen tool. This tool will postprocess the wasm file, generating a new wasm file and a set of JS bindings as well. This will give us a much more managed and readable js file that we can use. 

    wasm-bindgen target/wasm32-unknown-unknown/debug/<wasmfilename>.wasm --out-dir .

We tell wasm-bindgen to create the output in the directory that we are using hence the '.'

# Run the program
Finally we use npm to package the dependencies and run the program. First we need to install 
the packages defined in package.json

    npm install
    
We have defined a script in package.json that uses webpack-dev-server to run the application in
localhos port :8080 . We simply need to tell npm to run the script.

    npm run serve

    
