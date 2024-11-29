import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/feed/:feedId',
      name: 'feedView',
      component: () => import('../views/FeedView.vue'),
    },
  ],
})

export default router
