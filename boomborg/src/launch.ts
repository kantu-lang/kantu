import type * as ktypes from "./kantuTypes";
import * as kconvert from "./kantuConvert";

export type ImageDb = { [key: string]: HTMLImageElement };

export function launchBoomborgApp<S>(app: ktypes.App<S>, imageDb: ImageDb) {
  const [, , render, getUpdatedState, init, handleEvent] = app;

  (window as any).kconvert = kconvert;

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
  const width = kconvert.numToNat(window.innerWidth);
  const height = kconvert.numToNat(window.innerHeight);
  return ["window", width, height];
}

function toTime(time: number): ktypes.Time {
  return ["time", kconvert.numToNat(time)];
}

function drawEntities(
  ctx: CanvasRenderingContext2D,
  imageDb: ImageDb,
  imageCache: unknown,
  entitiesList: ktypes.List<ktypes.Entity>
) {
  const entities = kconvert.toArr(entitiesList);
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
    const x = kconvert.natToNum(xRaw);
    const y = kconvert.natToNum(yRaw);
    const imageKey = kconvert.toJsString(imageKeyRaw);
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y);
  } else if (entity[0] === "scaled") {
    const [, xRaw, yRaw, widthRaw, heightRaw, imageKeyRaw] = entity;
    const x = kconvert.natToNum(xRaw);
    const y = kconvert.natToNum(yRaw);
    const imageKey = kconvert.toJsString(imageKeyRaw);
    const width = kconvert.natToNum(widthRaw);
    const height = kconvert.natToNum(heightRaw);
    const image = imageDb[imageKey];
    ctx.drawImage(image, x, y, width, height);
  } else {
    throw { badEntity: entity };
  }
}

function getKeydownEvent(eventDotKey: string): ktypes.Event {
  return ["keydown", kconvert.toKString(eventDotKey)];
}

function getKeyupEvent(eventDotKey: string): ktypes.Event {
  return ["keyup", kconvert.toKString(eventDotKey)];
}

function getWindowResizeEvent(): ktypes.Event {
  return ["window_resize", getEWindow()];
}
