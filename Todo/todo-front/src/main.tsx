import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "modern-css-reset";
// import "./index.css"; このせいでbodyが左に寄る

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
