<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" class="object-contain h-48 w-96">
    <h1 class="text-lg italic">Welcome to Elemental Kanban</h1>

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
      <input v-model="taskID" type='number' name="task" placeholder="input your task id" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <button v-on:click="createTask" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Create a new task
      </button>

      <br />
      <span v-if="taskHash">Created a new task for column, hash {{ taskHash }}</span>
    </div>

    <br />
    <h2>Delete a task for the column</h2>
    <div>
      <button v-on:click="deleteTask" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Delete the last task
      </button>
    </div>

    <br />
    <h2>The board information</h2>
    <div>
      <p>{{ this.boardName }}</p>

      <input v-model="boardNameForGet" type='text' placeholder="input your board name" class="bg-green-50 border border-green-500 text-green-900 text-sm rounded-lg focus:ring-green-500 focus:border-green-500 w-500 p-2.5 dark:bg-green-100 dark:border-green-400" />
      <br />
      <br />
      
      <button v-on:click="getBoard" class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
        Get the board
      </button>

      <br />
      <span v-if="taskHash">{{ boardInfo }}</span>
    </div>

    <div class="flex justify-center">
      <div class="min-h-screen flex overflow-x-scroll py-12">
        <div
          v-for="column in boardInfo"
          :key="column.title"
          class="bg-gray-100 rounded-lg px-3 py-3 column-width rounded mr-4"
        >
          <p class="text-gray-700 font-semibold font-sans tracking-wide text-sm">{{column.title}}</p>
          <!-- Draggable component comes from vuedraggable. It provides drag & drop functionality -->
          <draggable :list="column.tasks" :animation="200" ghost-class="ghost-card" group="tasks">
            <!-- Each element from here will be draggable and animated. Note :key is very important here to be unique both for draggable and animations to be smooth & consistent. -->
            <template #item="{ element }">
              <task-card
                :task="element"
                class="mt-3 cursor-move"
                >
              </task-card>
            </template>
          </draggable>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Task from '@/components/Task.vue'; // @ is an alias to /src
import draggable from "vuedraggable";
import TaskCard from "@/components/TaskCard.vue";
import { appInfo, appWebsocket } from '../store/holochain';

export default defineComponent({
  name: 'Kanban',
  components: {
    Task,
    TaskCard,
    draggable
  },
  data() {
    return {
      columns: [
        {
          title: "Backlog",
          tasks: [
            {
              id: 1,
              title: "Add discount code to checkout page",
            },
            {
              id: 2,
              title: "Provide documentation on integrations",
            },
            {
              id: 3,
              title: "Design shopping cart dropdown",
            },
            {
              id: 4,
              title: "Add discount code to checkout page",
            },
            {
              id: 5,
              title: "Test checkout flow",
            }
          ]
        },
        {
          title: "In Progress",
          tasks: [
            {
              id: 6,
              title: "Design shopping cart dropdown",
            },
            {
              id: 7,
              title: "Add discount code to checkout page",
            },
            {
              id: 8,
              title: "Provide documentation on integrations",
            }
          ]
        },
        {
          title: "Review",
          tasks: [
            {
              id: 9,
              title: "Provide documentation on integrations",
            },
            {
              id: 10,
              title: "Design shopping cart dropdown",
            },
            {
              id: 11,
              title: "Add discount code to checkout page",
            },
            {
              id: 12,
              title: "Design shopping cart dropdown",
            },
            {
              id: 13,
              title: "Add discount code to checkout page",
            }
          ]
        },
        {
          title: "Done",
          tasks: [
            {
              id: 14,
              title: "Add discount code to checkout page",
            },
            {
              id: 15,
              title: "Design shopping cart dropdown",
            },
            {
              id: 16,
              title: "Add discount code to checkout page",
            }
          ]
        }
      ],
      boardInfo: undefined,
      boardHash: undefined,
      profileHash: undefined,
      handle: undefined,
      columnHash: undefined,
      taskHash: undefined,
      boardName: "", // TODO check empty
      boardNameForGet: "", // TODO check empty
      columnTitle: "",
      taskDescription: "",
      taskID: 0,
    }
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
            id: this.taskID,
            description: this.taskDescription
          },
          board: this.boardHash,
          column: this.columnHash,
        },
        provenance: cell_id[1],
      });
    },
    async deleteTask() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      const result = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'delete_task',
        payload: {
          task: this.taskHash,
          column: this.columnHash,
        },
        provenance: cell_id[1],
      });
      console.log(result);
    },
    async getBoard() {
      const info = await appInfo();
      const cell_id = info.cell_data[0].cell_id;

      const appWs = await appWebsocket();
      const board = await appWs.callZome({
        cap: null,
        cell_id: cell_id,
        zome_name: 'board',
        fn_name: 'get_board_by_name',
        payload: this.boardNameForGet,
        provenance: cell_id[1],
      });
      console.log("mytasks:", board);
      console.log("mytasks json:", JSON.stringify(board));

      this.boardInfo = board;
    }
  },
});
</script>
