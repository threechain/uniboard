import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import Home from '../views/Home.vue'
import Board from '../views/Board.vue'
import Kanban from '../views/Kanban.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  },
  {
    path: '/board',
    name: "Boared",
    component: Board
  },
  {
    path: '/kanban',
    name: "Kanban",
    component: Kanban
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
