<script setup lang="ts">
import { ref } from 'vue';
const props = defineProps<{
  errorMsg: string,
}>();

const emit = defineEmits(['onSignup', 'onLogin']);
const login_page = ref(true);
const username = ref('');
const password = ref('');

function login() {
    emit('onLogin', username.value, password.value);
}

function signup() {
    emit('onSignup', username.value, password.value);
}
</script>

<template>
  <div class="wrapper">
    <form v-if="login_page" @submit.prevent="login">
      <h1>Login</h1>
      <input type="text" placeholder="Username" v-model="username" required/>
      <input type="password" placeholder="Password" v-model="password" required/>
      <p v-if="props.errorMsg">{{ props.errorMsg }}</p>

      <button type="submit" class="btn">Login</button>
      <div class="switch_link">
        <p>Don't have account? <a href="#" @click="login_page = !login_page">Sign up</a></p>
      </div>
    </form>

    <form v-show="!login_page" @submit.prevent="signup">
      <h1>Sign up</h1>
      <input type="text" placeholder="Username" v-model="username" required/>
      <input type="password" placeholder="Password" v-model="password" required/>
      <p v-show="props.errorMsg">{{ props.errorMsg }}</p>
      
      <button type="submit" class="btn">Sign up</button>
      <div class="switch_link">
        <p>Already have an account? <a href="#" @click="login_page = !login_page">Login</a></p>
      </div>
    </form>
  </div>
</template>

<style>
.wrapper{
  width: 430px;
  height: 520px;
  position: absolute;
  transform: translate(-50%,-50%);
  left: 50%;
  top: 50%;
}
.wrapper form{
  height: 520px;
  width: 400px;
  background-color: rgba(255,255,255,0.13);
  position: absolute;
  transform: translate(-50%,-50%);
  top: 50%;
  left: 50%;
  border-radius: 10px;
  backdrop-filter: blur(10px);
  border: 2px solid rgba(255,255,255,0.1);
  box-shadow: 0 0 40px rgba(8,7,16,0.6);
  padding: 50px 35px;
}

.wrapper form *{
  font-family: 'Poppins',sans-serif;
  color: black;
  letter-spacing: 0.5px;
  outline: none;
  border: none;
}

.wrapper form h1{
  font-size: 32px;
  color: black;
  font-weight: 500;
  line-height: 42px;
  text-align: center;
  margin-bottom: 15px;
}

.wrapper form p{
  color: black;
  font-weight: 500;
  line-height: 42px;
  text-align: center;
  margin-bottom: 15px;
}

.wrapper form a{
  color: black;
  font-weight: 500;
  line-height: 42px;
  text-align: center;
  margin-bottom: 15px;
}

.wrapper input{
  display: block;
  height: 50px;
  width: 100%;
  background-color: #91cfff;
  border-radius: 3px;
  padding: 0 10px;
  margin-top: 8px;
  font-size: 14px;
  font-weight: 300;
}
::placeholder{
  color: #0f121a;
}

.wrapper button{
  margin-top: 20px;
  width: 100%;
  background-color: #0091ff;
  transition: background-color 0.8s ease;
  transition: all 0.8s ease;
  transition-property: color, background-color;
  color: #080710;
  padding: 15px 0;
  font-size: 18px;
  font-weight: 600;
  border-radius: 5px;
  cursor: pointer;
}

.wrapper button:hover {
  background-color: #0065b1;
  color: #fff;
}

</style>