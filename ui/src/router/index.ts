import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue'),
    meta: {
      title: '登录'
    }
  },
  {
    path: '/admin',
    component: () => import('@/layouts/AdminLayout.vue'),
    meta: {
      requiresAuth: true
    },
    children: [
      {
        path: '',
        redirect: '/admin/dashboard'
      },
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/views/admin/Dashboard.vue'),
        meta: {
          title: '仪表盘'
        }
      },
      {
        path: 'website',
        name: 'Website',
        component: () => import('@/views/admin/Website.vue'),
        meta: {
          title: '网站设置'
        }
      },
      {
        path: 'profile',
        name: 'Profile',
        component: () => import('@/views/admin/Profile.vue'),
        meta: {
          title: '个人资料'
        }
      },
      {
        path: 'categories',
        name: 'Categories',
        component: () => import('@/views/admin/CategoryView.vue'),
        meta: {
          title: '分类管理'
        }
      },
      {
        path: 'tags',
        name: 'Tags',
        component: () => import('@/views/admin/TagView.vue'),
        meta: {
          title: '标签管理'
        }
      },
      {
        path: 'articles',
        name: 'Articles',
        component: () => import('@/views/admin/ArticleListView.vue'),
        meta: {
          title: '文章管理'
        }
      },
      {
        path: 'articles/create',
        name: 'CreateArticle',
        component: () => import('@/views/admin/ArticleEditView.vue'),
        meta: {
          title: '写文章'
        }
      },
      {
        path: 'articles/edit/:id',
        name: 'EditArticle',
        component: () => import('@/views/admin/ArticleEditView.vue'),
        meta: {
          title: '编辑文章'
        }
      }
    ]
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/login'
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('token')

  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (!token) {
      next({
        path: '/login',
        query: { redirect: to.fullPath }
      })
    } else {
      next()
    }
  } else {
    if (token && to.path === '/login') {
      next('/admin')
    } else {
      next()
    }
  }
})

export default router
