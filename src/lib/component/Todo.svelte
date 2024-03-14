<script>
    import TaskItem from "./TaskItem.svelte";
    import { appendTask } from "../appendTask";
</script>

<div>
    <h2>ToDo</h2>
    <TaskItem />
    <form on:submit|preventDefault={appendTask}>
        <label for="">
            <input type="text" name="title" placeholder="タスク名" required>
            <textarea name="description" id="" cols="100" rows="5" placeholder="タスク詳細"></textarea>
            <input type="datetime" name="estimateSec" placeholder="見積時間" required>
        </label>
        <button id="appendTask" type="submit">タスク追加</button>
    </form>
</div>
