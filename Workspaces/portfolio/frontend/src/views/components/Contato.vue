<template>
  <div class="container mx-auto py-10">
    <h1 class="text-4xl text-center mb-6">Entre em Contato</h1>
    <form class="bg-white p-6 shadow-md rounded-lg">
      <div class="mb-4">
        <label class="block text-gray-700">Nome</label>
        <input type="text" class="w-full border border-gray-300 p-2 rounded-lg" placeholder="Seu nome">
      </div>
      <div class="mb-4">
        <label class="block text-gray-700">E-mail</label>
        <input type="email" class="w-full border border-gray-300 p-2 rounded-lg" placeholder="Seu e-mail">
      </div>
      <div class="mb-4">
        <label class="block text-gray-700">Mensagem</label>
        <textarea class="w-full border border-gray-300 p-2 rounded-lg" placeholder="Sua mensagem"></textarea>
      </div>
      <button class="bg-blue-600 text-white p-3 rounded-lg">Enviar</button>
    </form>
  </div>
</template>
