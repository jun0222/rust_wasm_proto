// index.js
import init, { TodoList } from "./pkg/rust_wasm_proto.js";

async function main() {
  await init();
  window.todoList = new TodoList();
  window.addTask = () => {
    let task = prompt("Enter a task:");
    window.todoList.add(task);
    document.getElementById("tasks").innerText = window.todoList.get_items();
  };
  window.removeTask = () => {
    let index = prompt("Enter task index to remove:");
    window.todoList.remove(parseInt(index));
    document.getElementById("tasks").innerText = window.todoList.get_items();
  };
}

main();
