import Vue from 'vue'
import App from './App.vue'
import VueRouter from "vue-router";

import Counter from "./components/Counter.vue";
import Auth from "./components/Auth";

Vue.config.productionTip = false

Vue.use(VueRouter)

const router = new VueRouter({
	mode: 'history',
	routes: [
		{path: '/counter/:id', component: Counter},
		{path: '/auth', component: Auth},
	],
})

new Vue({
	router,
	render: h => h(App),
}).$mount('#app')
