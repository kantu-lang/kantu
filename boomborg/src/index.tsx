import "./index.css";
import reportWebVitals from "./reportWebVitals";
import { launchBoomborgApp } from "./launch";

launchBoomborgApp(todo(), {});

function todo(): any {
  throw new Error("Not implemented");
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
