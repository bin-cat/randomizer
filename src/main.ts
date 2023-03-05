import "./style.scss";

import bulmaSlider from "bulma-slider/src/js";
import m from "mithril";

import App from "./components/app";

m.mount(document.getElementById("app") as Element, {
  view: () => m(App),
});

document.addEventListener("DOMContentLoaded", async () => {
  bulmaSlider.attach();
});
