<script>

</script>

<div class="taskItem">
    <h3 class="title nameArea">Task Name</h3>
    <details class="description descriptionArea" open>
        <summary>タスク内容</summary>
        タスクの詳細はあれこれを作るよ。
    </details>
    <canvas class="graph graphArea"></canvas>
    <div class="timeArea">
        <p class="timeItem estimate">
            作業見積:
            <time datetime="">00:00:00</time>
        </p>
        <p class="timeItem measurement">
            実測時間:
            <time datetime="">00:00:00</time>
        </p>
    </div>
</div>

<style lang="scss">
    .taskItem {
        display: grid;
        grid-template-areas: 
            "nameArea graphArea"
            "descriptionArea graphArea"
            "descriptionArea timeArea";
        grid-template-columns: 3fr 1fr;
        width: 100%;
        height: 300px;
        box-sizing: border-box;
        padding: 5px;

        .title {
            font-weight: bolder;
            font-size: x-large;
            text-align: left;
        }

        .description {
            font-size: large;
            text-align: left;
        }

        canvas {
            border-style: dotted;
        }

        .timeArea {
            .estimate {
                
            }

            .measurement {
                
            }
        }
    }

    .nameArea {
        grid-area: nameArea;
    }

    .timeArea {
        grid-area: timeArea;
    }
    
    .graphArea {
        grid-area: graphArea;
    }

    .descriptionArea {
        grid-area: descriptionArea;
    }
    
</style>