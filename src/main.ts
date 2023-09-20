import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";


//font awesome
///////////////////////////////////////
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faBars } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faBars)
////////////////////////////////////////


createApp(App)
.component('font-awesome-icon', FontAwesomeIcon)
.mount("#app");
