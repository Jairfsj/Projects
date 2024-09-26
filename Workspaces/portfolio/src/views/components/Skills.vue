<template>
  <section class="bg-gray-100 py-12">
   <div class="container  mx-auto">
      <h1 class="text-4xl font-bold text-center text-gray-800 mb-8">
        Habilidades Tecnicas de Jair Francisco da Silva Junior
      <h1>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div v-for="(skillCategory,index) in skill " :key=index" class="bg-white p-6 shadow-md rounded-lg">
          <h2 class="text-2xl font-semibold text-gray-700 mb-4">{{ skillCategory.category }}</h2>
          <ul class="list-disc list-inside">
            <li v-for="(skill, i) in skillCategory.items" :key="i" class="text-gray-600">
              {{ skill }}
            </li>
          </ul>
        </div>
      </div>
    </div>      
  </section>
</template>

<script>
export default {
  data() {
    return {
      skills: [
        {
       
          category: 'Backend',
          items: ['Rust', 'Actix', 'SQL', 'PostgreSQL']

        },
        { 
       
          category: 'Frontend',
          items: ['HTML5', 'CSS3', 'JavaScript', 'Vue.js', 'Tailwind CSS']
      
        },
        {
        
          category: 'Database',
          items: ['SQL', 'PostgreSQL', 'SQlite', 'Redis']
        }
      ]
    };
  }
};
</script>

<style scoped>
/*Estilos adicionais */
</style>
