export enum TaskStatus {
    todo = 0,
    doing = 1,
    suspend = 2, // 一時停止
    cancel = 3, // 実施せずにやめる
    done = 4, // 完了
}

export class Task {
    id: number;
    name: string;
    explain: string | null;
    createdDate: number;
    updatedDate: number;
    estimateSec: number;
    measurementSec: number;
    status: number;

    constructor(id: number, name: string, explain: string, createdDate:number, updatedDate: number, estimate: number, measurement: number, status: TaskStatus) {
        this.id = id;
        this.name = name;
        this.explain = explain;
        this.createdDate = createdDate;
        this.updatedDate = updatedDate;
        this.estimateSec = estimate;
        this.measurementSec = measurement;
        this.status = status;
    }
}

export type TaskInstace = InstanceType<typeof Task>