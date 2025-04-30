<script lang="ts">
export default {
  data() {
    return {
      username: '',
      password: '',
      login_page: false,
    };
  },
  methods: {
    login() {
      this.$emit('OnLogin', this.username, this.password)
    },
    signup() {
      this.$emit('OnSignup', this.username, this.password)
    }
  }
}
</script>

<template>
<div class="auth-page">
  <div class="card">
    <form v-if="login_page" @submit.prevent="login">
      <div class="title">Login</div>
      <input placeholder="Username" type="text" v-model="username" />
      <br />
      <input placeholder="Password" type="password" v-model="password" />
      <br />
      <button type="submit">Login</button>
    </form>

    <form v-if="!login_page" @submit.prevent="signup">
      <div class="title">Sign Up</div>
      <input placeholder="Username" type="text" v-model="username" />
      <br />
      <input placeholder="Password" type="password" v-model="password" />
      <br />
      <button type="submit">Sign Up</button>
    </form>
  </div>
</div>
</template>

<style>
* {
  font-family: Avenir;
  font-size: 22px;
}
body {
  margin: 0;
}
.auth-page {
  width: 100vw;
  margin: 0;
  height: 100vh;
  background: linear-gradient(
    180deg,
    rgba(117, 84, 160, 1) 7%,
    rgba(117, 84, 160, 1) 17%,
    rgba(106, 95, 168, 1) 29%,
    rgba(99, 103, 174, 1) 44%,
    rgba(87, 116, 184, 1) 66%,
    rgba(70, 135, 198, 1) 83%,
    rgba(44, 163, 219, 1) 96%,
    rgba(22, 188, 237, 1) 100%,
    rgba(0, 212, 255, 1) 100%
  );
}
.card {
  width: 200px;
  position: relative;
  left: calc(50vw - 100px);
  text-align: center;
  padding-top: 6vw;
}
.title {
  padding-top: 32px;
  font-size: 22px;
  color: white;
  font-weight: 700;
}
input {
  width: calc(100% - 16px);
  margin-top: 12px;
  padding: 8px;
  background-color: #e6f7ff;
  outline: none;
  border: 1px solid #e6f7ff;
}
button {
  margin-top: 12px;
  width: 100%;
  padding: 8px;
}
</style>