async function init() {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./bare_metal_wasm.wasm"),
    {
      "env": {
        "js_tan": Math.tan,
      },
    }
  );

  const width = 512;
  const height = 512;

  let x = 0
  let y = 0
  let targetX = 0
  let targetY = 0
  let speed = 128

  window.addEventListener('keydown', check,false);

  function check(e) {
      var code = e.keyCode;
      switch (code) {
          case 37: targetX -= speed; break;
          case 38: targetY -= speed; break;
          case 39: targetX += speed; break;
          case 40: targetY += speed; break;
      }
  }

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;

  const buffer_address = instance.exports.BUFFER.value;
  const image = new ImageData(
      new Uint8ClampedArray(
          instance.exports.memory.buffer,
          buffer_address,
          4 * width * height,
      ),
      width,
  );

  const ctx = canvas.getContext("2d");

  const render = () => {
    instance.exports.go(x, y);
    ctx.putImageData(image, 0, 0);
    x += Math.round((targetX - x) / 8)
    y += Math.round((targetY - y) / 8)
    if(targetX <= 0) {
      targetX -= targetX / 2
    }
    if(targetY <= 0) {
      targetY -= targetY / 2
    }
    requestAnimationFrame(render);
  };

  render();
}

init();
