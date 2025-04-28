<script lang="js">
import { ref } from 'vue';
import { listen } from '@tauri-apps/api/event';

export default {
  data() {
    return {
      user: '',
      connected: false,
    };
  },
  methods: {

  },
  beforeMount() {
      // setup listeners
      listen('chat_inserted', (event) => {
        
      });
      
      // Message inserted
      listen('message_inserted', (event) => {
        
      });
  },
}

</script>

<template>
<h1>Tauri + SpacetimeDB + cpal chat application</h1>
</template>

<style>

</style>