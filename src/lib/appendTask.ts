import { invoke } from "@tauri-apps/api";

type TaskStatus = {
    todo: 0;
    doing: 1;
    suspend: 2; // 一時停止
    cancel: 3; // 実施せずにやめる
    done: 4; // 完了
}

interface appendTask {
    [key: string]: any
}

class TaskItem {
    id: number;
    title: string;
    description: string;
    estimateSec: number;
    measurementSec: number;
    status: TaskStatus;
    subTasks: Array<TaskItem>;

    constructor(id: number, title: string, description: string, estimate: number, measurement: number, status: TaskStatus, childTasks: Array<TaskItem>) {
        this.id = id;
        this.title = title;
        this.description = description;
        this.estimateSec = estimate;
        this.measurementSec = measurement;
        this.status = status;
        this.subTasks = childTasks;
    }
}

async function appendTask(e: Event) {
    console.log("appendTask!");
    
    if (e === null) { return ""; }

    const formData = new FormData(e.target as HTMLFormElement);
    let task: appendTask = {
        title: "",
        description: "",
        estimateSec: 0
    };
    for (const key of formData.keys()) {
        task[key] = formData.get(key);
    }
    console.log(task);
    

    await invoke<null>("append_task", {title: task.title, description: task.description, estimateSec: Number(task.estimateSec)});
}

async function getTask(e: Event) {
    console.log("getTask");

    if (e === null) return "";

    await invoke<null>("show_tasks");
}

export { appendTask, getTask };