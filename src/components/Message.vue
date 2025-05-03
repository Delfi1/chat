<script setup lang="ts">
  import { ref } from 'vue';
  import { MessagePayload, UserPayload } from '../api';

  const props = defineProps<{
    self: UserPayload | undefined,
    user: UserPayload | undefined,
    payload: MessagePayload,
  }>();
  const emit = defineEmits(['edit', 'reply', 'remove']);
  const mouseHover = ref(false);

  // time formatter
  function time(unix: number): string {
    var date = new Date(unix);
    var current = new Date();

    // if today
    if (current.toLocaleDateString() == date.toLocaleDateString()) {
      return date.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
    }
    
    return date.toLocaleDateString();
  }

  function is_owner(): boolean {
    return props.user?.id == props.self?.id;
  }

  function can_remove(): boolean {
    return is_owner() || props.self?.id_admin as boolean;
  }

  function remove() {
    console.log("Remove", props.payload.id);
    emit("remove", props.payload.id);
  }

  function reply() {

  }

  function edit() {

  }
</script>

<template>
  <div @mouseover="mouseHover = true" @mouseleave="mouseHover = false" class="message">
    <div class="under-info">
      <h2> {{ props.user?.name }} </h2>
      <p> {{ time(props.payload.sent) }} </p>
      <div v-if="mouseHover" class="controls">
        <button v-if="can_remove()" @click="remove"><i class="pi pi-trash"></i></button>
        <button @click="reply"><i class="pi pi-reply"></i></button>
        <button v-if="is_owner()" @click="edit"><i class="pi pi-pencil"></i></button>
      </div>
    </div>
    <p> {{ props.payload.text }} </p>
  </div>
</template>

<style>

.message {
  width: 100%;
  padding-left: 5px;
  padding-top: 10px;
  padding-bottom: 10px;
  margin-top: 5px;
}

.message .controls {
  position: relative;
  right: 10px;
  padding: 2px;
  top: -15px;
  width: 150px;
  height: 32px;
  background-color: rgb(173, 145, 255);
  border-radius: 10px;
  display: flexbox;
  align-items: center;
}

.controls button {
  float: right;
  width: 15px;
  padding: 4px;
  margin: 2px;  
  width: 25px;
  right: 0px;
}

.message:hover {
  background-color: #c6e6ff;
}

.message p {
  inline-size: 90%;
  width: 80%;
  white-space: initial;
  overflow-wrap: anywhere;
  margin-left: 10px;
}

.under-info {
  display: flex;
}

.under-info p {
  margin: 5px;
}

</style>