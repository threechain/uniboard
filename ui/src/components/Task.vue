<template>
  <div class="hello">
    <h1>{{ msg }}</h1>

    <br />
    <!-- kanban board info -->
    <h2>Create Kanban Board</h2>
    <div>
      <input v-model="boardName" type='text' name="board" placeholder="input your board name" />
      <br />
      <button v-on:click="createBoard">
        Create a new board
      </button>

      <br />
      <span v-if="boardHash">Created a new kanban board, hash {{ boardHash }}</span>
    </div>

    <br />
    <h2>Create a new column for the board</h2>
    <div>
      <input v-model="columnTitle" type='text' name="column" placeholder="input your column title" />
      <br />
      <button v-on:click="createColumn">
        Create a new column
      </button>

      <br />
      <span v-if="columnHash">Created a new column for board, hash {{ columnHash }}</span>
    </div>

    <br />
    <h2>Create a new task for the column</h2>
    <div>
      <input v-model="taskDescription" type='text' name="task" placeholder="input your task description" />
      <br />
      <button v-on:click="createTask">
        Create a new task
      </button>

      <br />
      <span v-if="taskHash">Created a new task for column, hash {{ taskHash }}</span>
    </div>


    <br />
    <h2>The board information</h2>
    <div>
      <p>{{ this.boardName }}</p>

      <br />
      
    </div>

  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { appInfo, appWebsocket } from '../store/holochain';

export default defineComponent({
  name: 'Task',
  props: {
    msg: String,
  },
  data(): {
    boardHash: String | undefined,
    columnHash: String | undefined,
    taskHash: String | undefined,
    boardName: String,
    columnTitle: String,
    taskDescription: String,
  } {
    return {
      boardHash: undefined,
      columnHash: undefined,
      taskHash: undefined,
      boardName: "", // TODO check empty
      columnTitle: "",
      taskDescription: "",
    };
  },
  methods: {
    async createBoard() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      this.boardHash = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'create_board',
        payload: {
          name: this.boardName
        },
        provenance: cell_id[1],
      });
    },
    async createColumn() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      this.columnHash = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'create_column',
        payload: {
          column: {
            title: this.columnTitle
          },
          board: this.boardHash
        },
        provenance: cell_id[1],
      });
    },
    async createTask() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      this.taskHash = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'create_task',
        payload: {
          task: {
            description: this.taskDescription
          },
          board: this.boardHash,
          column: this.columnHash,
        },
        provenance: cell_id[1],
      });
    },
    async getBoard() {
    }
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
