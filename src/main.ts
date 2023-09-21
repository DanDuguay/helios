import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";


//font awesome
///////////////////////////////////////
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faFire , faChartBar , faLayerGroup , faGear } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faFire)
library.add(faChartBar)
library.add(faLayerGroup)
library.add(faGear)
////////////////////////////////////////


createApp(App)
.component('font-awesome-icon', FontAwesomeIcon)
.mount("#app");
