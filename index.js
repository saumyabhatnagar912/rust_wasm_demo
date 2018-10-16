const rust = import('./rust_wasm_demo')

document.getElementById("add").onclick = function() {
    var num1 = document.getElementById("num1").value;
    var num2 = document.getElementById("num2").value;
    var el = document.getElementById("result");
    rust.then(rust => {
        rust.add(num1, num2, el);
    });
}




