import { plans } from "./config.js";
const dialog = document.querySelector("#checkout");
for (const [id, plan] of Object.entries(plans)) {
  document.querySelector(`[data-price="${id}"]`).textContent =
    `$${plan.amount}`;
  const description = document.querySelector(`[data-description="${id}"]`);
  if (description) description.textContent = plan.description;
}
let selectedButton;
for (const button of document.querySelectorAll("[data-plan]")) {
  button.addEventListener("click", () => {
    selectedButton = button;
    const plan = plans[button.dataset.plan];
    document.querySelector("#checkout-title").textContent = `${plan.name} plan`;
    document.querySelector("#checkout-detail").textContent =
      `US$${plan.amount} · ${plan.interval}`;
    dialog.showModal();
  });
}
function closeCheckout() {
  dialog.close();
  selectedButton?.focus();
}
document.querySelector(".close").addEventListener("click", closeCheckout);
document
  .querySelector("#checkout-download")
  .addEventListener("click", () => dialog.close());
dialog.addEventListener("click", (event) => {
  if (event.target === dialog) closeCheckout();
});
