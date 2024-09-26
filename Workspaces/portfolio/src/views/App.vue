<template>
  <div>

    <header class="bg-blue-500 py-4">
      <nav class="container mx-auto text-white text-center">
        <a href="/" class="text-2xl font-bold">Portifolio de Jair Francisco da Silva</a>
      </nav>
    </header>

    <main>
      <skills />
    </main>  
    

    <footer class="bg-gray-800 py-4 text-white text-center">
      <p>© 2024 Jair francisco da Silva - Todos os direitos reservados</p>
    </footer>  
  </div>
</template>

<script>
import Skills from './components/Skills.vue';

export default {
  components: {
    Skills
  }
};
</script>

<style>
/* Importar o Tailwind CSS */
@import './assets/tailwind.css';
</style>
