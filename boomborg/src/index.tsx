import "./index.css";
import reportWebVitals from "./reportWebVitals";
import { ImageDb, launchBoomborgApp } from "./launch";
import { app } from "./app";
import type * as ktypes from "./kantuTypes";

getImageDb().then((imageDb) => {
  console.log("Launching app...");
  launchBoomborgApp(app as ktypes.App<unknown>, imageDb);
  console.log("Launched app.");
});

function getImageDb(): Promise<ImageDb> {
  const db: {
    A: null | HTMLImageElement;
    B: null | HTMLImageElement;
    C: null | HTMLImageElement;
  } = { A: null, B: null, C: null };
  return new Promise((resolve, reject) => {
    loadImage("A", "./background.png");
    loadImage("B", "./paddle.png");
    loadImage("C", "./ball.png");

    function loadImage(key: "A" | "B" | "C", url: string) {
      const img = new Image();
      img.src = url;
      img.addEventListener("load", () => {
        db[key] = img;
        resolveIfDone();
      });
      img.addEventListener("error", (err) => {
        reject(err);
      });
    }
    function resolveIfDone() {
      const { A, B, C } = db;
      if (A && B && C) {
        resolve(db as ImageDb);
      }
    }
  });
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
