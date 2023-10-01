import { createApp } from "vue";
import { createWebHistory, createRouter , RouterView , Router , RouteRecordRaw } from 'vue-router'
import "./styles.css";
import App from "./App.vue";
import Home from "./pages/Home.vue";
import Add from "./pages/Add.vue"
import Setting from "./pages/Setting.vue"
import Groups from  "./pages/Groups.vue"
import Simulation from "./pages/Simulations.vue"


//router
///////////////////////////////////////

const routes : RouteRecordRaw[]= [
    { path : "/" , component : Home},
    { path : '/add' , component : Add },
    { path : '/simulations' , component : Simulation},
    { path : '/setting' , component : Setting},
    { path : '/groups' , component : Groups}
]


const router : Router = createRouter({
    history: createWebHistory(),
    routes : routes
})


//font awesome
///////////////////////////////////////
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faFire , faChartBar , faLayerGroup , faGear , faPlus } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faFire)
library.add(faChartBar)
library.add(faLayerGroup)
library.add(faGear)
library.add(faPlus)
////////////////////////////////////////


createApp(App)
.use(router)
.component('font-awesome-icon', FontAwesomeIcon)
.mount("#app");
