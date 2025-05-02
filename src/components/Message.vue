<script setup lang="ts">
  import { MessagePayload, UserPayload } from '../api';

  const props = defineProps<{
    user: UserPayload | undefined,
    payload: MessagePayload
  }>();
  const emit = defineEmits(['on_edit']);


  // time formatter
  function time(unix: number): string {
    var date = new Date(unix);
    var hour = date.getHours();
    var min = date.getMinutes();
    var sec = date.getSeconds();

    return hour + ':' + min + ':' + sec
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
  padding-left: 5px;
  padding-top: 10px;
}

.message p {
  margin-left: 10px;
}

.under-info {
  display: flex;
}

.under-info p {
  margin: 5px;
}

</style>