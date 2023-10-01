import "./styles.css";
import App from "./App.svelte";

const targetTag = document.getElementById("app") ?? document;

const app = new App({
  target: targetTag,
});

export default app;
