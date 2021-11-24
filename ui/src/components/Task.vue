<template>
  <div class="hello">
    <h1>{{ msg }}</h1>

    <span v-if="postHash">Created new Holochain entry! Post with hash {{ postHash }}</span>
    <span v-else>Creating...</span>

     <!-- kanban board info -->
    <div>
      <input type='text' name="board" placeholder="input your board name" />
      <br />
      <button onClick={createBoard}>
        Create a new board
      </button>

      <br />
      <span v-if="boardHash">Created a ne kanban board, hash {{ boardHash }}</span>
      <span v-else>Creating kanban board...</span>
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
  data(): { postHash: string | undefined, boardHash: String | undefined } {
    return {
      postHash: undefined,
      boardHash: undefined
    };
  },
  async createBoard() {
    const info = await appInfo();
    const cell_id = info.cell_data[0].cell_id;

    const appWs = await appWebsocket();
    this.boardHash = await appWs.callZome({
      cap: null,
      cell_id: cell_id,
      zome_name: 'career',
      fn_name: 'create_post',
      payload: 'my new post',
      provenance: cell_id[1],
    });
  },
  async mounted() {
    const info = await appInfo();
    const cell_id = info.cell_data[0].cell_id;

    const appWs = await appWebsocket();
    this.postHash = await appWs.callZome({
      cap: null,
      cell_id: cell_id,
      zome_name: 'career',
      fn_name: 'create_post',
      payload: 'my new post',
      provenance: cell_id[1],
    });
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
