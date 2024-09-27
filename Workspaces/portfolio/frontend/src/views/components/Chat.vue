<template>
  <div class="container mx-auto py-10">
    <h1 class="text-4xl text-center mb-6">Chat em Tempo Real</h1>
    <div class="bg-gray-100 p-6 rounded-lg">
      <div class="mb-4" v-for="message in messages" :key="message">{{ message }}</div>
      <input type="text" v-model="message" @keyup.enter="sendMessage" class="w-full border border-gray-300 p-2 rounded-lg">
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      socket: null,
      message: '',
      messages: []
    };
  },
  mounted() {
    this.socket = new WebSocket('ws://127.0.0.1:8080/chat');
    this.socket.onmessage = (event) => {
      this.messages.push(event.data);
    };
  },
  methods: {
    sendMessage() {
      if (this.message !== '') {
        this.socket.send(this.message);
        this.message = '';
      }
    }
  }
};
</script>

