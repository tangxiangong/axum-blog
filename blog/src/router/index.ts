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
    // {
    //   path: '/posts/:id',
    //   name: 'post',
    //   component: () => import('../views/PostView.vue')
    // },
    // {
    //   path: '/tags',
    //   name: 'tags',
    //   component: () => import('../views/TagsView.vue')
    // },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue'),
    },
  ],
})

export default router
