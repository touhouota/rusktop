import { invoke } from "@tauri-apps/api";
import type { Task } from "./types";


interface appendTask {
    [key: string]: any
}

let appendTask = async (e: Event) => {
    console.log("appendTask!");
    
    if (e === null) { return ""; }

    const formData = new FormData(e.target as HTMLFormElement);
    let task: appendTask = {
        title: "",
        explain: "",
        estimateSec: 0
    };
    for (const key of formData.keys()) {
        task[key] = formData.get(key);
    }
    console.log(task);
    
    await invoke<null>("append_task", {title: task.title, explain: task.explain, estimateSec: Number(task.estimateSec)});
}

let getTask = async () => {
    console.log("getTask");

    let tasks = await invoke<Task[]>("get_tasks");
    console.log(tasks);

    return tasks;

    // return await invoke<Task[]>("get_tasks")
}

export { appendTask, getTask };