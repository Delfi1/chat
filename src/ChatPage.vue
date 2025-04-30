<script lang="ts">
import { ref } from 'vue';
//import { invoke } from '@tauri-apps/api/core';

interface UserPayload {
  id: number,
  name: string,
  online: true
}

interface MessagePayload {
  user: number,
  time: number,
  text: string
}

export type { UserPayload, MessagePayload }
export default {
  setup() {
    return {
      user: ref<UserPayload>(),
      view: 0,
      messages: []
    }
  },
  methods: {
    get_posts() {

    } 
  }
}
</script>

<template>
<h2>Chat page</h2>
</template>

<style>

</style>