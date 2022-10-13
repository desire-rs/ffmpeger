import { createApp } from "vue";
import * as VueRouter from 'vue-router'
import App from "./App.vue";
import Home from "./Home.vue";
import About from "./About.vue";
import Options from "./Options.vue";


const routes = [
  { path: '/', component: Home },
  { path: '/options', component: Options },
  { path: '/about', component: About },
];
const router = VueRouter.createRouter({
  history: VueRouter.createWebHashHistory(),
  routes,
});
const app = createApp(App);
app.use(router);
app.mount("#app");
