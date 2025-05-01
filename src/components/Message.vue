<script setup lang="ts">
  import { MessagePayload, UserPayload } from '../api';

  const props = defineProps<{
    user: UserPayload | undefined,
    value: MessagePayload
  }>();
  const emit = defineEmits(['on_edit']);

</script>

<template>
  <div class="message">
    <h2> {{ props.user?.name }} </h2>
    <p> {{ props.value.text }} </p>
  </div>
</template>

<style lang="css" scoped>
  .message {
    padding: 0.4em;
  }
  .post h2 {
    
  }

</style>