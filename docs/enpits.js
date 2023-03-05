function startApp(app, imageDb) {
  const [render, getUpdatedState, init, handleEvent] = app;

  const imageCache = {};
  let state = init(getEWindow(), toTime(Date.now()));
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  document.body.appendChild(canvas);
  canvas.style.position = "absolute";
  canvas.style.top = "0";
  canvas.style.left = "0";
  resizeCanvas();

  window.addEventListener("resize", resizeCanvasAndUpdate);
  window.addEventListener("keydown", (e) => {
    state = handleEvent(state, getKeydownEvent(e.key));
    updateAndRedraw();
  });
  window.addEventListener("keyup", (e) => {
    state = handleEvent(state, getKeyupEvent(e.key));
    updateAndRedraw();
  });

  animationIteration();

  function animationIteration() {
    updateAndRedraw();
    requestAnimationFrame(animationIteration);
  }

  function updateAndRedraw() {
    const currentTime = toTime(Date.now());
    state = getUpdatedState(state, currentTime);
    const entities = render(state);
    drawEntities(ctx, imageDb, imageCache, entities);
  }

  function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
  }

  function resizeCanvasAndUpdate() {
    resizeCanvas();
    state = handleEvent(state, getWindowResizeEvent());
    updateAndRedraw();
  }
}

/** @returns {enpits.Window} */
function getEWindow() {
  const width = numToNat(window.innerWidth);
  const height = numToNat(window.innerHeight);
  return ["window", width, height];
}

/**
 * @param {number} time - `Date.valueOf()`
 * @returns {enpits.Time}
 */
function toTime(time) {
  return ["time", numToNat(time)];
}

function drawEntities(ctx, imageDb, imageCache, entities) {
  const entitiesLen = entities.length;
  for (let i = 0; i < entitiesLen; ++i) {
    const entity = entities[i];
    drawEntity(ctx, imageDb, imageCache, entity);
  }
}

function drawEntity(ctx, imageDb, imageCache, entity) {
  if (entity[0] === "unscaled") {
    const [, x, y, imageKey] = entity;
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y);
  } else if (entity[0] === "scaled") {
    const [, x, y, width, height, imageKey] = entity;
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y, width, height);
  } else {
    throw { badEntity: entity };
  }
}

/**
 * @param {string} key
 * @returns {enpits.KeydownEvent}
 */
function getKeydownEvent(eventDotKey) {
  return ["keydown", toEnpitsString(eventDotKey)];
}

/**
 * @param {string} key
 * @returns {enpits.KeyupEvent}
 */
function getKeyupEvent(eventDotKey) {
  return ["keyup", toEnpitsString(eventDotKey)];
}

/**
 * @returns {enpits.WindowResizeEvent}
 */
function getWindowResizeEvent() {
  return ["window_resize", getEWindow()];
}

// Utils for converting JS representations of
// Kantu types (e.g., Nat, List)
// to and from their native JS analogues (e.g., number, Array).

function pnatToNum(nat) {
  if (nat.length === 1) {
    return 0;
  }
  if (nat.length === 2) {
    return 1 + pnatToNum(nat[1]);
  }
  throw { badNat: nat };
}

function toArr(list) {
  if (list.length === 2) {
    return [];
  }
  if (list.length === 4) {
    return [list[2]].concat(toArr(list[3]));
  }
  throw { badList: list };
}

function intToNum(int) {
  if (int[0] === "neg") {
    return -posToNum(int[1]);
  }
  if (int[0] === "nat") {
    return natToNum(int[1]);
  }
  throw { badInt: int };
}

function natToNum(nat) {
  if (nat[0] === "zero") {
    return 0;
  }
  if (nat[0] === "pos") {
    return posToNum(nat[1]);
  }
  throw { badNat: nat };
}

function posToNum(pos) {
  return parseInt(posToBitString(pos), 2);
}

function posToBitString(pos) {
  if (pos[0] === "one") {
    return "1";
  }
  if (pos[0] === "extend") {
    const left = posToBitString(pos[1]);
    const right = String(bitToNum(pos[2]));
    return left + right;
  }
  throw { badPos: pos };
}

function bitToNum(bit) {
  if (bit[0] === "zero") {
    return 0;
  }
  if (bit[0] === "one") {
    return 1;
  }
  throw { badBit: bit };
}

function numToNat(n) {
  if (n < 0) {
    throw { cannotConvertNegNumToNat: n };
  }
  if (n === 0) {
    return ["zero"];
  }
  return ["pos", numToPos(n)];
}

function numToPos(n) {
  if (n === 1) {
    return ["one"];
  }
  return ["extend", numToPos(n >> 1), numToBit(n & 1)];
}

function numToBit(n) {
  if (n === 0) {
    return ["zero"];
  }
  if (n === 1) {
    return ["one"];
  }
  throw { cannotConvertNumToBit: n };
}

function toEnpitsString(s) {
  // TODO: Convert to utf8 and then a list of bytes.
}
