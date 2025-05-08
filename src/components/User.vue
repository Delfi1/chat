<script setup lang="ts">
  import { UserPayload } from '../api';

  const props = defineProps<{
    payload: UserPayload | undefined,
  }>();
  const emit = defineEmits([]);

</script>

<template>
  <div class="user">
    <h2> {{ props.payload?.name }} </h2>
  </div>
</template>

<style>
.user {
  width: 100%;
  height: 30px;
  margin-left: 10px;
  color: black;
}
</style>