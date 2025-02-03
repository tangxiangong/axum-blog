import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
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
      path: '/login',
      name: 'login',
      component: () => import('@/views/admin/LoginView.vue'),
      meta: { requiresAuth: false }
    },
    {
      path: '/admin',
      component: () => import('@/views/admin/AdminLayout.vue'),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'dashboard',
          component: () => import('@/views/admin/DashboardView.vue')
        },
        {
          path: 'posts',
          name: 'posts',
          component: () => import('@/views/admin/posts/PostListView.vue')
        },
        {
          path: 'posts/create',
          name: 'createPost',
          component: () => import('@/views/admin/posts/PostEditView.vue')
        },
        {
          path: 'posts/:id/edit',
          name: 'editPost',
          component: () => import('@/views/admin/posts/PostEditView.vue')
        },
        {
          path: 'categories',
          name: 'categories', 
          component: () => import('@/views/admin/CategoryView.vue')
        },
        {
          path: 'tags',
          name: 'tags',
          component: () => import('@/views/admin/TagView.vue')
        },
        {
          path: 'profile',
          name: 'profile',
          component: () => import('@/views/admin/ProfileView.vue')
        }
      ]
    }
  ],
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login')
  } else {
    next()
  }
})

export default router
