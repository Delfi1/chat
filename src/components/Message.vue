<script setup lang="ts">
  import { MessagePayload, UserPayload } from '../api';

  const props = defineProps<{
    user: UserPayload | undefined,
    payload: MessagePayload,
    self?: boolean
  }>();
  const emit = defineEmits(['on_edit']);


  // time formatter
  function time(unix: number): string {
    var date = new Date(unix);
    return date.toLocaleTimeString();
  }
</script>

<template>
  <div class="message">
    <div class="under-info">
      <h2> {{ props.user?.name }} </h2>
      <p> {{ time(props.payload.sent) }} </p>
    </div>
    <p> {{ props.payload.text }} </p>
  </div>
</template>

<style>
.message {
  width: 500px;
  padding-left: 5px;
  padding-top: 10px;
  padding-bottom: 10px;
}

.message:hover {
  background-color: #c6e6ff;
}

.message p {
  white-space: initial;
  inline-size: 80%;
  overflow-wrap: break-word;
  margin-left: 10px;
}

.under-info {
  display: flex;
}

.under-info p {
  margin: 5px;
}

</style>