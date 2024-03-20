<script lang="ts">
    import { TaskStatus, type TaskInstace } from "../types";
    import TaskItem from "./TaskItem.svelte";
    import { appendTask, getTask } from "../appendTask";
    import { onMount } from "svelte";

    let tasks: Array<TaskInstace> = [];
    getTask().then(result => tasks = result);
    console.log("todo: ", tasks);
    
    onMount(() => {
        getTask().then(result => tasks = result);
        console.log("todo: mound: ", tasks);
    })
</script>

<div>
    <h2>ToDo</h2>
    {#each tasks as task}
        <TaskItem taskinfo={task} />
    {/each}
    <form on:submit|preventDefault={appendTask}>
        <label for="">
            <input type="text" name="title" placeholder="タスク名" required>
            <textarea name="explain" id="" cols="100" rows="5" placeholder="タスク詳細"></textarea>
            <input type="datetime" name="estimateSec" placeholder="見積時間" required>
        </label>
        <button id="appendTask" type="submit">タスク追加</button>
    </form>
</div>
