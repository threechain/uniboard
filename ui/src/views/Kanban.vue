<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" class="object-contain h-48 w-96">
    <Task msg="Welcome to Elemental Kanban"/>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Task from '@/components/Task.vue'; // @ is an alias to /src

export default defineComponent({
  name: 'Kanban',
  components: {
    Task,
  },
});
</script>
