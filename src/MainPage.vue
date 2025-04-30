<script setup lang="ts">
import { UserPayload, MessagePayload } from './api';

const props = defineProps<{
  user: UserPayload | undefined
  users: UserPayload[],
  messages: MessagePayload[]
}>();
const emit = defineEmits(['onSignup', 'onLogin']);

</script>

<template>
    <h3>Username: {{ props.user?.name }}</h3>

</template>