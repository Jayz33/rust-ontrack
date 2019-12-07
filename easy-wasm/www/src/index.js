function loadWasm() {
  fetch('easy_wasm.wasm')
     .then(r => r.arrayBuffer())
     .then(instantiateWasm);
}

function instantiateWasm(buffer) {
  var imports = {
    env: {
      helloWorld: function() {
        console.log("Hello world");
      },
      alert: function(ptr, len) {
        var decoder = new TextDecoder();
        var text = decoder.decode(window.memory.subarray(ptr, ptr + len));
        alert(text);
      }
    }
  };
  WebAssembly
    .instantiate(buffer, imports)
    .then(wasm => {
      window.memory = new Uint8Array(wasm.instance.exports.memory.buffer);
      console.log(wasm.instance.exports.get_number());
      console.log(wasm.instance.exports.sum(5, 6));
      wasm.instance.exports.call_hello_world();
      wasm.instance.exports.greeting();
    });
}

loadWasm();