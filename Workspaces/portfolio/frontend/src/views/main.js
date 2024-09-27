import { createApp} from 'vue';
import App from './App.vue';

//Criando a aplicacao Vue
//createApp(App).mount('#app');

import { createRouter, createWebHistory } from 'vue-router';
import Skills from './components/Skills.vue';
import Formacao from './components/Formacao.vue';
import Contato from './components/Contato.vue';

// Configuração das rotas
const routes = [
  { path: '/', component: Skills },
  { path: '/formacao', component: Formacao },
  { path: '/contato', component: Contato }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

const app = createApp(App);
app.use(router);
app.mount('#app');
