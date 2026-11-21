import { mount } from "svelte";
import "./spin.css";
import SpinGuestApp from "./SpinGuestApp.svelte";

const target = document.getElementById("app");
if (target) {
  mount(SpinGuestApp, { target });
}
