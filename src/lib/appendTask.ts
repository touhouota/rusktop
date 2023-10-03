type TaskStatus = {
    todo: 0;
    doing: 1;
    suspend: 2; // 一時停止
    cancel: 3; // 実施せずにやめる
    done: 4; // 完了
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

let appendTask = (e: Event) => {
    console.log("appendTask!");
    
    if (e === null) { return ""; }

    const formData = new FormData(e.target as HTMLFormElement);
    for (const pair of formData.entries()) {
        console.log(pair);
    }
}

export { appendTask };