import type * as ktypes from "./kantuTypes";

export type ImageDb = { [key: string]: HTMLImageElement };

export function launchBoomborgApp<S>(app: ktypes.App<S>, imageDb: ImageDb) {
  const [, , render, getUpdatedState, init, handleEvent] = app;

  const imageCache = {};
  let state = init(getEWindow(), toTime(Date.now()));
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d")!;
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

function getEWindow(): ktypes.Window {
  const width = numToNat(window.innerWidth);
  const height = numToNat(window.innerHeight);
  return ["window", width, height];
}

function toTime(time: number): ktypes.Time {
  return ["time", numToNat(time)];
}

function drawEntities(
  ctx: CanvasRenderingContext2D,
  imageDb: ImageDb,
  imageCache: unknown,
  entitiesList: ktypes.List<ktypes.Entity>
) {
  const entities = toArr(entitiesList);
  const entitiesLen = entities.length;
  for (let i = 0; i < entitiesLen; ++i) {
    const entity = entities[i];
    drawEntity(ctx, imageDb, imageCache, entity);
  }
}

function drawEntity(
  ctx: CanvasRenderingContext2D,
  imageDb: ImageDb,
  imageCache: unknown,
  entity: ktypes.Entity
) {
  if (entity[0] === "unscaled") {
    const [, xRaw, yRaw, imageKeyRaw] = entity;
    const x = natToNum(xRaw);
    const y = natToNum(yRaw);
    const imageKey = toJsString(imageKeyRaw);
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y);
  } else if (entity[0] === "scaled") {
    const [, xRaw, yRaw, widthRaw, heightRaw, imageKeyRaw] = entity;
    const x = natToNum(xRaw);
    const y = natToNum(yRaw);
    console.log({ x });
    const imageKey = toJsString(imageKeyRaw);
    const width = natToNum(widthRaw);
    const height = natToNum(heightRaw);
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y, width, height);
  } else {
    throw { badEntity: entity };
  }
}

function getKeydownEvent(eventDotKey: string): ktypes.Event {
  return ["keydown", toKString(eventDotKey)];
}

function getKeyupEvent(eventDotKey: string): ktypes.Event {
  return ["keyup", toKString(eventDotKey)];
}

function getWindowResizeEvent(): ktypes.Event {
  return ["window_resize", getEWindow()];
}

// Utils for converting JS representations of
// Kantu types (e.g., Nat, List)
// to and from their native JS analogues (e.g., number, Array).

// function pnatToNum(nat) {
//   if (nat.length === 1) {
//     return 0;
//   }
//   if (nat.length === 2) {
//     return 1 + pnatToNum(nat[1]);
//   }
//   throw { badNat: nat };
// }

function toArr<T>(list: ktypes.List<T>): T[] {
  if (list.length === 2) {
    return [];
  }
  if (list.length === 4) {
    return [list[2]].concat(toArr(list[3]));
  }
  throw { badList: list };
}

function intToNum(int: ktypes.Int): number {
  if (int[0] === "neg") {
    return -posToNum(int[1]);
  }
  if (int[0] === "nat") {
    return natToNum(int[1]);
  }
  throw { badInt: int };
}

function natToNum(nat: ktypes.Nat): number {
  if (nat[0] === "zero") {
    return 0;
  }
  if (nat[0] === "pos") {
    return posToNum(nat[1]);
  }
  throw { badNat: nat };
}

function posToNum(pos: ktypes.Pos): number {
  return parseInt(posToBitString(pos), 2);
}

function posToBitString(pos: ktypes.Pos): string {
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

function bitToNum(bit: ktypes.Bit): number {
  if (bit[0] === "zero") {
    return 0;
  }
  if (bit[0] === "one") {
    return 1;
  }
  throw { badBit: bit };
}

function numToNat(n: number): ktypes.Nat {
  if (n < 0) {
    throw { cannotConvertNegNumToNat: n };
  }
  if (n === 0) {
    return ["zero"];
  }
  return ["pos", numToPos(n)];
}

function numToPos(n: number): ktypes.Pos {
  if (!Number.isInteger(n) || n < 0) {
    throw { cannotConvertNonNumToPos: n };
  }
  if (n === 1) {
    return ["one"];
  }
  return ["extend", numToPos(Math.floor(n / 2)), numToBit(n % 2)];
}

function numToBit(n: number): ktypes.Bit {
  if (n === 0) {
    return ["zero"];
  }
  if (n === 1) {
    return ["one"];
  }
  throw { cannotConvertNumToBit: n };
}

function toKString(s: string): ktypes.KString {
  const textEncoder = new TextEncoder();
  const buff = new Uint8Array(s.length * 3);
  const encodedResults = textEncoder.encodeInto(s, buff);
  const utf8 = Array.from(buff.subarray(0, encodedResults.written)).map(toU8);
  return ["utf8", toList(utf8)];
}

function toJsString(s: ktypes.KString): string {
  const bytes = new Uint8Array(toArr(s[1]).map(u8ToNum));
  const decoder = new TextDecoder();
  return decoder.decode(bytes);
}

function toU8(n: number): ktypes.U8 {
  if (n < 0 || n > 255) {
    throw { cannotConvertNumToU8: n };
  }
  return ["u8", numToNat(n), {}];
}

function u8ToNum(u8: ktypes.U8): number {
  return natToNum(u8[1]);
}

function toList<T>(arr: T[]): ktypes.List<T> {
  if (arr.length === 0) {
    return ["nil", typePlaceholder()];
  }
  return ["cons", typePlaceholder(), arr[0], toList(arr.slice(1))];
}

function typePlaceholder(): {} {
  return {};
}

function miscPlaceholder(): {} {
  return {};
}
