function log() {
  const args = [...arguments];
  const str = args.join(" ");
  const elem = document.createElement('pre');
  console.log.apply(this, args);
  elem.innerText = str;
  document.body.appendChild(elem);
};

function wraplog(channelName) {
  return function _log() {
    const args = [...arguments];
    return log.apply(this, [`<${channelName}>`].concat(args));
  }
};

function makeContextObject(env) {
  const log = wraplog(env);
  const context = {};
  context.exports = {
    wasi_snapshot_preview1: { // needed by the TinyGo implementation
      fd_write: () => {} // ??
    },
    env: {
      foo: () => {
        log("foo called!");
      },
      load_texture: (width, height, array_length, arrayptr) => {
        log("load_texture():", width, height, array_length, arrayptr);
        const texturebuf = new Uint8ClampedArray(context.memory.buffer.slice(arrayptr, arrayptr + array_length));
        const imageData = new ImageData(texturebuf, width, height, { colorSpace: 'srgb' });
        const canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        canvas.getContext('2d').putImageData(imageData, 0, 0);
        const img = document.createElement('img');
        img.src = canvas.toDataURL('image/png');
        const scale = 1;
        img.style.transform = `scale(${scale}, ${scale})`;
        img.style.transformOrigin = "top left";
        img.title = `<${env}> ${width} x ${height}, ${array_length} bytes total (displayed at ${scale}x scale)`;
        document.body.appendChild(img);
      }
    }
  };
  return context;
};

fetch('go/ai.wasm').then(async response => {
  const log = wraplog('go');
  const buf = await response.arrayBuffer();
  const context = makeContextObject('go');
  await WebAssembly.instantiate(buf, context.exports).then(results => {
    log(results.instance.exports.sum(42, 49));
  });
});

fetch('rust/target/wasm32-unknown-unknown/debug/deps/ai.wasm').then(async response => {
  const log = wraplog('rust');
  const buf = await response.arrayBuffer();
  const context = makeContextObject('rust');
  await WebAssembly.instantiate(buf, context.exports).then(results => {
    context.memory = results.instance.exports.memory;
    log(results.instance.exports.sum(42, 49));
    results.instance.exports.init();
  });
});

fetch('c/ai.wasm').then(async response => {
  const log = wraplog('c');
  const buf = await response.arrayBuffer();
  const context = makeContextObject('c');
  await WebAssembly.instantiate(buf, context.exports).then(results => {
    context.memory = results.instance.exports.memory;
    log(results.instance.exports.sum(42, 49));
    results.instance.exports.init();
  });
});
