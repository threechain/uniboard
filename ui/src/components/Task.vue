<template>
  <div class="hello">
    <h1 class="text-lg italic">{{ msg }}</h1>

    <br />

    <!-- profile info -->
    <h2>User profile</h2>
    <div>
      <input v-model="handle" type='text' name="board" placeholder="create your profile" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <button v-on:click="createProfile" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Create profile
      </button>

      <br />
      <span v-if="profileHash">Created profile successfully, hash {{ profileHash }}</span>
    </div>


    <!-- kanban board info -->
    <h2>Create Kanban Board</h2>
    <div>
      <input v-model="boardName" type='text' name="board" placeholder="input your board name" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <button v-on:click="createBoard" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Create a new board
      </button>

      <br />
      <span v-if="boardHash">Created a new kanban board, hash {{ boardHash }}</span>
    </div>

    <br />
    <h2>Create a new column for the board</h2>
    <div>
      <input v-model="columnTitle" type='text' name="column" placeholder="input your column title" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <button v-on:click="createColumn" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Create a new column
      </button>

      <br />
      <span v-if="columnHash">Created a new column for board, hash {{ columnHash }}</span>
    </div>

    <br />
    <h2>Create a new task for the column</h2>
    <div>
      <input v-model="taskDescription" type='text' name="task" placeholder="input your task description" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <button v-on:click="createTask" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
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
      
      <button v-on:click="getBoard" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Get the board
      </button>

      <br />
      <span v-if="taskHash">{{ boardInfo }}</span>
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
    profileHash: String | undefined,
    handle: String | undefined,
    boardHash: String | undefined,
    columnHash: String | undefined,
    taskHash: String | undefined,
    boardName: String,
    columnTitle: String,
    taskDescription: String,
    boardInfo: String,
  } {
    return {
      profileHash: undefined,
      handle: undefined,
      boardHash: undefined,
      columnHash: undefined,
      taskHash: undefined,
      boardName: "", // TODO check empty
      columnTitle: "",
      taskDescription: "",
      boardInfo: "",
    };
  },
  methods: {
    async createProfile() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      this.profileHash = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: "profile",
        fn_name: "create_profile",
        payload: {
          handle: this.handle
        },
        provenance: cell_id[1],
      });
    },
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
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      const board = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'get_board',
        payload: this.boardHash,
        provenance: cell_id[1],
      });
      this.boardInfo = JSON.stringify(board);
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
